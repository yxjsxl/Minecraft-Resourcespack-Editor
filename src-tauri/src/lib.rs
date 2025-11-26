mod commands;
mod image_handler;
mod pack_parser;
mod zip_handler;
mod minecraft_data;
mod pack_creator;
mod history_manager;
mod version_downloader;

#[cfg(feature = "web-server")]
mod web_server;

use commands::*;
use tauri::Manager;

#[cfg(feature = "web-server")]
use web_server::{WebServerState, start_server, stop_server, get_server_status};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::default());

    #[cfg(feature = "web-server")]
    {
        builder = builder.manage(WebServerState::default());
    }

    builder
        .invoke_handler(tauri::generate_handler![
            import_pack_zip,
            import_pack_folder,
            check_pack_mcmeta,
            get_current_pack_info,
            get_current_pack_path,
            get_image_thumbnail,
            get_image_details,
            export_pack,
            cleanup_temp,
            read_file_content,
            write_file_content,
            create_new_file,
            create_new_folder,
            delete_file,
            rename_file,
            get_pack_mcmeta,
            update_pack_mcmeta,
            get_all_minecraft_items,
            get_items_by_category,
            search_minecraft_items,
            create_new_pack,
            create_item_model,
            create_block_model,
            create_multiple_item_models,
            create_multiple_block_models,
            get_system_fonts,
            get_file_tree,
            create_transparent_png,
            save_image,
            get_minecraft_versions,
            download_minecraft_version,
            download_latest_minecraft_version,
            extract_assets_from_jar,
            download_and_extract_template,
            clear_template_cache,
            history_manager::save_file_history,
            history_manager::load_file_history,
            history_manager::get_history_stats,
            history_manager::clear_file_history,
            history_manager::clear_all_history,
            history_manager::get_pack_size,
            #[cfg(feature = "web-server")]
            start_server,
            #[cfg(feature = "web-server")]
            stop_server,
            #[cfg(feature = "web-server")]
            get_server_status,
        ])
        .setup(|app| {
            // 初始化
            let window = app.get_webview_window("main").unwrap();
            
            #[cfg(debug_assertions)]
            {
                window.open_devtools();
            }
            
            // 亚克力
            #[cfg(target_os = "windows")]
            {
                use window_vibrancy::apply_acrylic;
                let _ = apply_acrylic(&window, Some((30, 30, 30, 125)));
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
