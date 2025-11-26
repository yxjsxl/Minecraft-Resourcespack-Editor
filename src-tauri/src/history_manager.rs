use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryEntry {
    pub timestamp: String,
    pub content: String,
    pub file_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryMetadata {
    pub version: String,
    pub max_history_per_file: u32,
    pub files: HashMap<String, FileHistoryInfo>,
    pub total_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileHistoryInfo {
    pub history_count: u32,
    pub last_modified: String,
    pub size: u64,
}

// 获取.history文件夹路径
fn get_history_dir(pack_dir: &Path) -> PathBuf {
    pack_dir.join(".history")
}

// 获取文件的历史记录目录
fn get_file_history_dir(pack_dir: &Path, file_path: &str) -> PathBuf {
    let history_dir = get_history_dir(pack_dir);
    let file_history_path = history_dir.join(file_path);
    file_history_path
}

// 保存文件历史记录
#[command]
pub async fn save_file_history(
    pack_dir: String,
    file_path: String,
    content: String,
    file_type: String,
    max_count: u32,
) -> Result<String, String> {
    let pack_path = Path::new(&pack_dir);
    let file_history_dir = get_file_history_dir(pack_path, &file_path);
    
    // 创建历史记录目录
    fs::create_dir_all(&file_history_dir)
        .map_err(|e| format!("创建历史记录目录失败: {}", e))?;
    
    // 获取现有历史记录数量
    let entries = fs::read_dir(&file_history_dir)
        .map_err(|e| format!("读取历史记录目录失败: {}", e))?;
    
    let mut count = entries.count() as u32;
    
    // 如果超过限制删除最旧的记录
    if count >= max_count {
        let mut files: Vec<_> = fs::read_dir(&file_history_dir)
            .map_err(|e| format!("读取历史记录失败: {}", e))?
            .filter_map(|e| e.ok())
            .collect();
        
        files.sort_by_key(|f| f.file_name());
        
        if let Some(oldest) = files.first() {
            fs::remove_file(oldest.path())
                .map_err(|e| format!("删除旧历史记录失败: {}", e))?;
            count -= 1;
        }
    }
    
    // 创建新的历史记录
    let timestamp = chrono::Utc::now().to_rfc3339();
    let entry = HistoryEntry {
        timestamp: timestamp.clone(),
        content,
        file_type,
    };
    
    let history_file = file_history_dir.join(format!("{:03}.json", count + 1));
    let json = serde_json::to_string_pretty(&entry)
        .map_err(|e| format!("序列化历史记录失败: {}", e))?;
    
    fs::write(&history_file, json)
        .map_err(|e| format!("写入历史记录失败: {}", e))?;
    
    // 更新元数据
    update_metadata(pack_path, &file_path, count + 1, &timestamp)?;
    
    Ok("历史记录保存成功".to_string())
}

// 加载文件历史记录
#[command]
pub async fn load_file_history(
    pack_dir: String,
    file_path: String,
) -> Result<Vec<HistoryEntry>, String> {
    let pack_path = Path::new(&pack_dir);
    let file_history_dir = get_file_history_dir(pack_path, &file_path);
    
    if !file_history_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut entries = Vec::new();
    let dir_entries = fs::read_dir(&file_history_dir)
        .map_err(|e| format!("读取历史记录目录失败: {}", e))?;
    
    for entry in dir_entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)
                    .map_err(|e| format!("读取历史记录文件失败: {}", e))?;
                let history_entry: HistoryEntry = serde_json::from_str(&content)
                    .map_err(|e| format!("解析历史记录失败: {}", e))?;
                entries.push(history_entry);
            }
        }
    }
    
    // 按时间戳排序
    entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    
    Ok(entries)
}

// 获取历史记录统计信息
#[command]
pub async fn get_history_stats(pack_dir: String) -> Result<HistoryMetadata, String> {
    let pack_path = Path::new(&pack_dir);
    let meta_file = get_history_dir(pack_path).join("history_meta.json");
    
    if !meta_file.exists() {
        return Ok(HistoryMetadata {
            version: "1.0".to_string(),
            max_history_per_file: 30,
            files: HashMap::new(),
            total_size: 0,
        });
    }
    
    let content = fs::read_to_string(&meta_file)
        .map_err(|e| format!("读取元数据失败: {}", e))?;
    let metadata: HistoryMetadata = serde_json::from_str(&content)
        .map_err(|e| format!("解析元数据失败: {}", e))?;
    
    Ok(metadata)
}

// 清理指定文件的历史记录
#[command]
pub async fn clear_file_history(pack_dir: String, file_path: String) -> Result<String, String> {
    let pack_path = Path::new(&pack_dir);
    let file_history_dir = get_file_history_dir(pack_path, &file_path);
    
    if file_history_dir.exists() {
        fs::remove_dir_all(&file_history_dir)
            .map_err(|e| format!("删除历史记录失败: {}", e))?;
    }
    
    Ok("历史记录已清理".to_string())
}

// 清理所有历史记录
#[command]
pub async fn clear_all_history(pack_dir: String) -> Result<String, String> {
    let pack_path = Path::new(&pack_dir);
    let history_dir = get_history_dir(pack_path);
    
    if history_dir.exists() {
        fs::remove_dir_all(&history_dir)
            .map_err(|e| format!("删除所有历史记录失败: {}", e))?;
    }
    
    Ok("所有历史记录已清理".to_string())
}

// 获取材质包大小
#[command]
pub async fn get_pack_size(pack_dir: String) -> Result<u64, String> {
    let pack_path = Path::new(&pack_dir);
    calculate_dir_size(pack_path, true)
}

// 更新元数据
fn update_metadata(
    pack_path: &Path,
    file_path: &str,
    count: u32,
    timestamp: &str,
) -> Result<(), String> {
    let meta_file = get_history_dir(pack_path).join("history_meta.json");
    
    let mut metadata = if meta_file.exists() {
        let content = fs::read_to_string(&meta_file)
            .map_err(|e| format!("读取元数据失败: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("解析元数据失败: {}", e))?
    } else {
        HistoryMetadata {
            version: "1.0".to_string(),
            max_history_per_file: 30,
            files: HashMap::new(),
            total_size: 0,
        }
    };
    
    let file_history_dir = get_file_history_dir(pack_path, file_path);
    let size = calculate_dir_size(&file_history_dir, false)?;
    
    metadata.files.insert(
        file_path.to_string(),
        FileHistoryInfo {
            history_count: count,
            last_modified: timestamp.to_string(),
            size,
        },
    );
    
    // 重新计算总大小
    metadata.total_size = metadata.files.values().map(|f| f.size).sum();
    
    let json = serde_json::to_string_pretty(&metadata)
        .map_err(|e| format!("序列化元数据失败: {}", e))?;
    
    fs::write(&meta_file, json)
        .map_err(|e| format!("写入元数据失败: {}", e))?;
    
    Ok(())
}

// 计算目录大小
fn calculate_dir_size(path: &Path, exclude_history: bool) -> Result<u64, String> {
    let mut total_size = 0u64;
    
    if !path.exists() {
        return Ok(0);
    }
    
    if path.is_file() {
        return Ok(path.metadata()
            .map_err(|e| format!("获取文件大小失败: {}", e))?
            .len());
    }
    
    let entries = fs::read_dir(path)
        .map_err(|e| format!("读取目录失败: {}", e))?;
    
    for entry in entries {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            
            // 如果需要排除.history文件夹
            if exclude_history && entry_path.file_name().and_then(|s| s.to_str()) == Some(".history") {
                continue;
            }
            
            if entry_path.is_file() {
                total_size += entry_path.metadata()
                    .map_err(|e| format!("获取文件大小失败: {}", e))?
                    .len();
            } else if entry_path.is_dir() {
                total_size += calculate_dir_size(&entry_path, exclude_history)?;
            }
        }
    }
    
    Ok(total_size)
}