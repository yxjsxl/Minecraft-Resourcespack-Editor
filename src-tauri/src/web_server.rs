use axum::Router;
use tower_http::{
    services::ServeDir,
    cors::CorsLayer,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;

#[derive(Default, Clone)]
pub struct WebServerState {
    pub running: Arc<Mutex<bool>>,
    pub handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

pub async fn start_web_server(
    port: u16,
    pack_path: String,
    bind_all: bool,
) -> Result<tokio::task::JoinHandle<()>, String> {
    // 创建服务目录
    let serve_dir = ServeDir::new(pack_path.clone())
        .append_index_html_on_directories(true);

    // 创建路由
    let app = Router::new()
        .nest_service("/", serve_dir)
        .layer(CorsLayer::permissive());

    // 确定绑定地址
    let addr = if bind_all {
        SocketAddr::from(([0, 0, 0, 0], port))
    } else {
        SocketAddr::from(([127, 0, 0, 1], port))
    };

    println!("Starting web server on {}", addr);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;

    let handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            eprintln!("Server error: {}", e);
        }
    });

    Ok(handle)
}

#[tauri::command]
pub async fn start_server(
    port: u16,
    mode: String,
    state: State<'_, WebServerState>,
    app_state: State<'_, crate::commands::AppState>,
) -> Result<String, String> {
    let mut running = state.running.lock().await;
    
    if *running {
        return Err("Server is already running".to_string());
    }

    // 获取当前材质包路径
    let pack_path_str = {
        let pack_path = app_state.current_pack_path.lock().unwrap();
        match pack_path.as_ref() {
            Some(path) => path.to_string_lossy().to_string(),
            None => return Err("No pack loaded".to_string()),
        }
    };

    let bind_all = mode == "all";
    
    match start_web_server(port, pack_path_str, bind_all).await {
        Ok(handle) => {
            *state.handle.lock().await = Some(handle);
            *running = true;
            
            let addr = if bind_all {
                format!("0.0.0.0:{}", port)
            } else {
                format!("127.0.0.1:{}", port)
            };
            
            Ok(format!("Server started on {}", addr))
        }
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn stop_server(state: State<'_, WebServerState>) -> Result<String, String> {
    let mut running = state.running.lock().await;
    
    if !*running {
        return Err("Server is not running".to_string());
    }

    if let Some(handle) = state.handle.lock().await.take() {
        handle.abort();
    }
    
    *running = false;
    Ok("Server stopped".to_string())
}

#[tauri::command]
pub async fn get_server_status(state: State<'_, WebServerState>) -> Result<bool, String> {
    Ok(*state.running.lock().await)
}