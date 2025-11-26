use crate::image_handler::{create_thumbnail, get_image_info, ImageInfo};
use crate::pack_parser::{scan_pack_directory, PackInfo};
use crate::zip_handler::{cleanup_temp_files, create_zip, extract_zip, get_temp_extract_dir, validate_pack_zip};
use std::path::{Path, PathBuf};
use tauri::State;
use std::sync::Mutex;
use font_kit::source::SystemSource;

/// 应用状态
pub struct AppState {
    pub current_pack_path: Mutex<Option<PathBuf>>,
    pub current_pack_info: Mutex<Option<PackInfo>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_pack_path: Mutex::new(None),
            current_pack_info: Mutex::new(None),
        }
    }
}

/// 导入材质包
#[tauri::command]
pub async fn import_pack_zip(zip_path: String, state: State<'_, AppState>) -> Result<PackInfo, String> {
    let zip_path = Path::new(&zip_path);
    
    // 验证ZIP文件
    if !validate_pack_zip(zip_path)? {
        return Err("Invalid resource pack: pack.mcmeta not found".to_string());
    }
    
    // 解压到临时目录
    let temp_dir = get_temp_extract_dir();
    let extract_path = temp_dir.join(
        zip_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    );
    
    extract_zip(zip_path, &extract_path)?;
    
    // 扫描材质包
    let pack_info = scan_pack_directory(&extract_path)?;
    
    // 保存状态
    *state.current_pack_path.lock().unwrap() = Some(extract_path);
    *state.current_pack_info.lock().unwrap() = Some(pack_info.clone());
    
    Ok(pack_info)
}

/// 检查文件夹是否有pack.mcmeta
#[tauri::command]
pub async fn check_pack_mcmeta(folder_path: String) -> Result<bool, String> {
    let folder_path = Path::new(&folder_path);
    
    if !folder_path.exists() {
        return Err("Folder does not exist".to_string());
    }
    
    Ok(folder_path.join("pack.mcmeta").exists())
}

/// 导入材质包
#[tauri::command]
pub async fn import_pack_folder(folder_path: String, state: State<'_, AppState>) -> Result<PackInfo, String> {
    let folder_path = Path::new(&folder_path);
    
    if !folder_path.exists() {
        return Err("Folder does not exist".to_string());
    }
    
    // 扫描材质包(即使没有pack.mcmeta也允许导入)
    let pack_info = scan_pack_directory(folder_path)?;
    
    // 保存状态
    *state.current_pack_path.lock().unwrap() = Some(folder_path.to_path_buf());
    *state.current_pack_info.lock().unwrap() = Some(pack_info.clone());
    
    Ok(pack_info)
}

/// 获取当前材质包信息
#[tauri::command]
pub async fn get_current_pack_info(state: State<'_, AppState>) -> Result<Option<PackInfo>, String> {
    let pack_info = state.current_pack_info.lock().unwrap();
    Ok(pack_info.clone())
}

/// 获取当前材质包路径
#[tauri::command]
pub async fn get_current_pack_path(state: State<'_, AppState>) -> Result<String, String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    match pack_path.as_ref() {
        Some(path) => Ok(path.to_string_lossy().to_string()),
        None => Err("No pack loaded".to_string()),
    }
}

/// 获取图片缩略图
#[tauri::command]
pub async fn get_image_thumbnail(image_path: String, max_size: u32, state: State<'_, AppState>) -> Result<String, String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    
    let full_path = match pack_path.as_ref() {
        Some(base_path) => {
            let path = Path::new(&image_path);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                base_path.join(path)
            }
        }
        None => {
            PathBuf::from(&image_path)
        }
    };
    
    create_thumbnail(&full_path, max_size)
}

/// 获取图片信息
#[tauri::command]
pub async fn get_image_details(image_path: String, state: State<'_, AppState>) -> Result<ImageInfo, String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    
    let full_path = match pack_path.as_ref() {
        Some(base_path) => {
            let path = Path::new(&image_path);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                base_path.join(path)
            }
        }
        None => {
            PathBuf::from(&image_path)
        }
    };
    
    get_image_info(&full_path)
}

/// 导出材质包
#[tauri::command]
pub async fn export_pack(output_path: String, state: State<'_, AppState>) -> Result<(), String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    
    match pack_path.as_ref() {
        Some(path) => {
            let output = Path::new(&output_path);
            create_zip(path, output)?;
            Ok(())
        }
        None => Err("No pack loaded".to_string()),
    }
}

/// 清理临时文件
#[tauri::command]
pub async fn cleanup_temp() -> Result<(), String> {
    cleanup_temp_files()
}

/// 读取文件内容
#[tauri::command]
pub async fn read_file_content(file_path: String, state: State<'_, AppState>) -> Result<String, String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    
    let full_path = match pack_path.as_ref() {
        Some(base_path) => {
            let path = Path::new(&file_path);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                base_path.join(path)
            }
        }
        None => {
            PathBuf::from(&file_path)
        }
    };
    
    std::fs::read_to_string(&full_path)
        .map_err(|e| format!("Failed to read file: {}", e))
}

/// 写入文件内容
#[tauri::command]
pub async fn write_file_content(file_path: String, content: String, state: State<'_, AppState>) -> Result<(), String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    
    let full_path = match pack_path.as_ref() {
        Some(base_path) => {
            let path = Path::new(&file_path);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                base_path.join(path)
            }
        }
        None => {
            // 如果没有加载材质包，尝试直接使用路径
            PathBuf::from(&file_path)
        }
    };
    
    std::fs::write(&full_path, content)
        .map_err(|e| format!("Failed to write file: {}", e))
}

/// 创建新文件
#[tauri::command]
pub async fn create_new_file(
    file_path: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    
    let full_path = match pack_path.as_ref() {
        Some(base_path) => {
            let path = Path::new(&file_path);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                base_path.join(path)
            }
        }
        None => {
            return Err("No pack loaded".to_string());
        }
    };
    
    // 创建父目录
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    
    // 写入文件
    std::fs::write(&full_path, content)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    Ok(())
}

/// 创建新文件夹
#[tauri::command]
pub async fn create_new_folder(
    folder_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    
    let full_path = match pack_path.as_ref() {
        Some(base_path) => {
            let path = Path::new(&folder_path);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                base_path.join(path)
            }
        }
        None => {
            return Err("No pack loaded".to_string());
        }
    };
    
    // 创建文件夹
    std::fs::create_dir_all(&full_path)
        .map_err(|e| format!("Failed to create folder: {}", e))?;
    
    Ok(())
}

/// 删除文件
#[tauri::command]
pub async fn delete_file(
    file_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    
    let full_path = match pack_path.as_ref() {
        Some(base_path) => {
            let path = Path::new(&file_path);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                base_path.join(path)
            }
        }
        None => {
            return Err("No pack loaded".to_string());
        }
    };
    
    // 判断是文件还是目录
    let metadata = std::fs::metadata(&full_path)
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;
    
    if metadata.is_dir() {
        std::fs::remove_dir_all(&full_path)
            .map_err(|e| format!("Failed to delete folder: {}", e))?;
    } else {
        std::fs::remove_file(&full_path)
            .map_err(|e| format!("Failed to delete file: {}", e))?;
    }
    
    Ok(())
}

/// 重命名文件
#[tauri::command]
pub async fn rename_file(
    old_path: String,
    new_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    
    let full_old_path = match pack_path.as_ref() {
        Some(base_path) => {
            let path = Path::new(&old_path);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                base_path.join(path)
            }
        }
        None => {
            return Err("No pack loaded".to_string());
        }
    };
    
    let full_new_path = match pack_path.as_ref() {
        Some(base_path) => {
            let path = Path::new(&new_path);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                base_path.join(path)
            }
        }
        None => {
            return Err("No pack loaded".to_string());
        }
    };
    
    std::fs::rename(&full_old_path, &full_new_path)
        .map_err(|e| format!("Failed to rename file: {}", e))?;
    
    Ok(())
}

/// 获取pack.mcmeta内容
#[tauri::command]
pub async fn get_pack_mcmeta(state: State<'_, AppState>) -> Result<String, String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    
    match pack_path.as_ref() {
        Some(path) => {
            let mcmeta_path = path.join("pack.mcmeta");
            std::fs::read_to_string(mcmeta_path)
                .map_err(|e| format!("Failed to read pack.mcmeta: {}", e))
        }
        None => Err("No pack loaded".to_string()),
    }
}

/// 更新pack.mcmeta
#[tauri::command]
pub async fn update_pack_mcmeta(
    content: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    
    match pack_path.as_ref() {
        Some(path) => {
            let mcmeta_path = path.join("pack.mcmeta");
            std::fs::write(mcmeta_path, content)
                .map_err(|e| format!("Failed to write pack.mcmeta: {}", e))?;
            
            // 重新扫描材质包
            let pack_info = scan_pack_directory(path)?;
            drop(pack_path);
            *state.current_pack_info.lock().unwrap() = Some(pack_info);
            
            Ok(())
        }
        None => Err("No pack loaded".to_string()),
    }
}

/// 获取所有物品/方块
#[tauri::command]
pub async fn get_all_minecraft_items() -> Result<Vec<crate::minecraft_data::MinecraftItem>, String> {
    Ok(crate::minecraft_data::get_all_items())
}

/// 按类别获取物品
#[tauri::command]
pub async fn get_items_by_category(
    category: crate::minecraft_data::ItemCategory,
) -> Result<Vec<crate::minecraft_data::MinecraftItem>, String> {
    Ok(crate::minecraft_data::get_items_by_category(category))
}

/// 搜索物品
#[tauri::command]
pub async fn search_minecraft_items(query: String) -> Result<Vec<crate::minecraft_data::MinecraftItem>, String> {
    Ok(crate::minecraft_data::search_items(&query))
}

/// 创建新材质包
#[tauri::command]
pub async fn create_new_pack(
    output_path: String,
    pack_name: String,
    pack_format: i32,
    description: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let path = std::path::Path::new(&output_path);
    crate::pack_creator::create_new_pack(path, &pack_name, pack_format, &description)?;
    
    // 自动加载新创建的材质包
    let pack_info = crate::pack_parser::scan_pack_directory(path)?;
    *state.current_pack_path.lock().unwrap() = Some(path.to_path_buf());
    *state.current_pack_info.lock().unwrap() = Some(pack_info);
    
    Ok(())
}

/// 为物品创建模型
#[tauri::command]
pub async fn create_item_model(
    item_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let pack_path_guard = state.current_pack_path.lock().unwrap();
    let pack_info_guard = state.current_pack_info.lock().unwrap();
    
    let path = pack_path_guard.as_ref().ok_or("No pack loaded")?;
    let info = pack_info_guard.as_ref().ok_or("No pack loaded")?;
    let pack_format = info.pack_format;
    let path_clone = path.clone();
    
    drop(pack_path_guard);
    drop(pack_info_guard);
    
    crate::pack_creator::create_item_model(&path_clone, &item_id, pack_format)?;
    
    // 重新扫描材质包
    let new_pack_info = crate::pack_parser::scan_pack_directory(&path_clone)?;
    *state.current_pack_info.lock().unwrap() = Some(new_pack_info);
    
    Ok(())
}

/// 为方块创建模型
#[tauri::command]
pub async fn create_block_model(
    block_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let pack_path_guard = state.current_pack_path.lock().unwrap();
    let path = pack_path_guard.as_ref().ok_or("No pack loaded")?.clone();
    drop(pack_path_guard);
    
    crate::pack_creator::create_block_model(&path, &block_id)?;
    
    // 重新扫描材质包
    let pack_info = crate::pack_parser::scan_pack_directory(&path)?;
    *state.current_pack_info.lock().unwrap() = Some(pack_info);
    
    Ok(())
}

/// 批量创建物品模型
#[tauri::command]
pub async fn create_multiple_item_models(
    item_ids: Vec<String>,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let pack_path_guard = state.current_pack_path.lock().unwrap();
    let pack_info_guard = state.current_pack_info.lock().unwrap();
    
    let path = pack_path_guard.as_ref().ok_or("No pack loaded")?;
    let info = pack_info_guard.as_ref().ok_or("No pack loaded")?;
    let pack_format = info.pack_format;
    let path_clone = path.clone();
    
    drop(pack_path_guard);
    drop(pack_info_guard);
    
    let created = crate::pack_creator::create_multiple_item_models(
        &path_clone,
        &item_ids,
        pack_format,
    )?;
    
    // 重新扫描材质包
    let new_pack_info = crate::pack_parser::scan_pack_directory(&path_clone)?;
    *state.current_pack_info.lock().unwrap() = Some(new_pack_info);
    
    Ok(created)
}
/// 批量创建方块模型
#[tauri::command]
pub async fn create_multiple_block_models(
    block_ids: Vec<String>,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let pack_path_guard = state.current_pack_path.lock().unwrap();
    let path = pack_path_guard.as_ref().ok_or("No pack loaded")?.clone();
    drop(pack_path_guard);
    
    let created = crate::pack_creator::create_multiple_block_models(&path, &block_ids)?;
    
    // 重新扫描材质包
    let pack_info = crate::pack_parser::scan_pack_directory(&path)?;
    *state.current_pack_info.lock().unwrap() = Some(pack_info);
    
    Ok(created)
}

/// 获取系统已安装的字体列表
#[tauri::command]
pub async fn get_system_fonts() -> Result<Vec<String>, String> {
    let source = SystemSource::new();
    let mut font_names = std::collections::HashSet::new();
    
    // 获取所有字体族
    let families = source.all_families()
        .map_err(|e| format!("Failed to get font families: {}", e))?;
    
    for family in families {
        font_names.insert(family);
    }
    
    // 转换为排序的向量
    let mut fonts: Vec<String> = font_names.into_iter().collect();
    fonts.sort();
    
    Ok(fonts)
}

/// 文件树节点
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileTreeNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Option<Vec<FileTreeNode>>,
}

/// 递归读取目录结构
fn read_directory_tree(path: &Path, base_path: &Path) -> Result<Vec<FileTreeNode>, String> {
    let mut nodes = Vec::new();
    
    let entries = std::fs::read_dir(path)
        .map_err(|e| format!("Failed to read directory: {}", e))?;
    
    let mut entries: Vec<_> = entries.collect();
    entries.sort_by_key(|entry| {
        entry.as_ref()
            .ok()
            .and_then(|e| e.file_name().into_string().ok())
            .unwrap_or_default()
    });
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let entry_path = entry.path();
        let metadata = entry.metadata()
            .map_err(|e| format!("Failed to read metadata: {}", e))?;
        
        let relative_path = entry_path.strip_prefix(base_path)
            .unwrap_or(&entry_path)
            .to_string_lossy()
            .replace('\\', "/");
        
        let name = entry.file_name().to_string_lossy().to_string();
        
        let node = if metadata.is_dir() {
            let children = read_directory_tree(&entry_path, base_path)?;
            FileTreeNode {
                name,
                path: relative_path,
                is_dir: true,
                children: Some(children),
            }
        } else {
            FileTreeNode {
                name,
                path: relative_path,
                is_dir: false,
                children: None,
            }
        };
        
        nodes.push(node);
    }
    
    Ok(nodes)
}

/// 获取材质包的文件树结构
#[tauri::command]
pub async fn get_file_tree(state: State<'_, AppState>) -> Result<FileTreeNode, String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    
    match pack_path.as_ref() {
        Some(path) => {
            let pack_name = path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            
            let children = read_directory_tree(path, path)?;
            
            Ok(FileTreeNode {
                name: pack_name,
                path: String::new(),
                is_dir: true,
                children: Some(children),
            })
        }
        None => Err("No pack loaded".to_string()),
    }
}

/// 创建透明PNG图片
#[tauri::command]
pub async fn create_transparent_png(
    file_path: String,
    width: u32,
    height: u32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let pack_path = state.current_pack_path.lock().unwrap();
    
    let full_path = match pack_path.as_ref() {
        Some(base_path) => {
            let path = Path::new(&file_path);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                base_path.join(path)
            }
        }
        None => {
            return Err("No pack loaded".to_string());
        }
    };
    
    crate::image_handler::create_transparent_png(&full_path, width, height)?;
    
    Ok(())
}

/// 保存编辑后的图片
#[tauri::command]
pub async fn save_image(
    image_path: String,
    base64_data: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    use base64::{Engine as _, engine::general_purpose};
    
    let pack_path = state.current_pack_path.lock().unwrap();
    
    let full_path = match pack_path.as_ref() {
        Some(base_path) => {
            let path = Path::new(&image_path);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                base_path.join(path)
            }
        }
        None => {
            return Err("No pack loaded".to_string());
        }
    };
    
    // 解码base64数据
    let image_data = general_purpose::STANDARD
        .decode(&base64_data)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;
    
    // 确保父目录存在
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    
    // 写入文件
    std::fs::write(&full_path, image_data)
        .map_err(|e| format!("Failed to save image: {}", e))?;
    
    Ok(())
}

/// 获取版本清单
#[tauri::command]
pub async fn get_minecraft_versions() -> Result<crate::version_downloader::VersionManifest, String> {
    crate::version_downloader::fetch_version_manifest().await
}

/// 下载指定的版本jar文件
#[tauri::command]
pub async fn download_minecraft_version(
    version_id: String,
) -> Result<String, String> {
    // 获取src-tauri目录的路径
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?;
    let exe_dir = exe_path.parent()
        .ok_or("Failed to get exe directory")?;
    
    // 创建temp目录
    let temp_dir = exe_dir.join("temp");
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    // 下载版本
    crate::version_downloader::download_version(&version_id, &temp_dir).await
}

/// 下载最新的release版本
#[tauri::command]
pub async fn download_latest_minecraft_version() -> Result<String, String> {
    // 获取src-tauri目录的路径
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?;
    let exe_dir = exe_path.parent()
        .ok_or("Failed to get exe directory")?;
    
    // 创建temp目录
    let temp_dir = exe_dir.join("temp");
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    // 下载最新版本
    crate::version_downloader::download_latest_release(&temp_dir).await
}

/// 从jar文件中提取assets到指定目录
#[tauri::command]
pub async fn extract_assets_from_jar(
    jar_path: String,
    output_path: String,
) -> Result<(), String> {
    let jar = Path::new(&jar_path);
    let output = Path::new(&output_path);
    
    crate::version_downloader::extract_assets_from_jar(jar, output)
}

/// 下载版本并提取assets到材质包
#[tauri::command]
pub async fn download_and_extract_template(
    version_id: String,
    pack_path: String,
    keep_cache: bool,
) -> Result<String, String> {
    // 获取temp目录
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?;
    let exe_dir = exe_path.parent()
        .ok_or("Failed to get exe directory")?;
    let temp_dir = exe_dir.join("temp");
    
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    let output = Path::new(&pack_path);
    
    // 下载并提取
    crate::version_downloader::download_and_extract_version(
        &version_id,
        &temp_dir,
        output,
        keep_cache,
    ).await
}

/// 清理模板缓存
#[tauri::command]
pub async fn clear_template_cache() -> Result<(), String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?;
    let exe_dir = exe_path.parent()
        .ok_or("Failed to get exe directory")?;
    let temp_dir = exe_dir.join("temp");
    
    crate::version_downloader::clear_template_cache(&temp_dir)
}