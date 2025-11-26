use serde::{Deserialize, Serialize};
use std::path::Path;

/// 版本清单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionManifest {
    pub latest: LatestVersions,
    pub versions: Vec<VersionInfo>,
}

/// 最新版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestVersions {
    pub release: String,
    pub snapshot: String,
}

/// 版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

/// 版本详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDetails {
    pub id: String,
    pub downloads: Downloads,
}

/// 下载信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Downloads {
    pub client: Option<DownloadInfo>,
    pub server: Option<DownloadInfo>,
}

/// 单个下载信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadInfo {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

const VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

/// 获取版本清单
pub async fn fetch_version_manifest() -> Result<VersionManifest, String> {
    let response = reqwest::get(VERSION_MANIFEST_URL)
        .await
        .map_err(|e| format!("Failed to fetch version manifest: {}", e))?;
    
    let manifest = response
        .json::<VersionManifest>()
        .await
        .map_err(|e| format!("Failed to parse version manifest: {}", e))?;
    
    Ok(manifest)
}

/// 获取版本详细信息
pub async fn fetch_version_details(version_url: &str) -> Result<VersionDetails, String> {
    let response = reqwest::get(version_url)
        .await
        .map_err(|e| format!("Failed to fetch version details: {}", e))?;
    
    let details = response
        .json::<VersionDetails>()
        .await
        .map_err(|e| format!("Failed to parse version details: {}", e))?;
    
    Ok(details)
}

/// 下载jar文件
pub async fn download_jar_with_progress(
    download_url: &str,
    output_path: &Path,
) -> Result<(), String> {
    use futures_util::StreamExt;
    use std::io::Write;
    
    // 确保输出目录存在
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    
    // 下载文件
    let response = reqwest::get(download_url)
        .await
        .map_err(|e| format!("Failed to download jar: {}", e))?;
    
    let total_size = response.content_length().unwrap_or(0);
    
    // 创建文件
    let mut file = std::fs::File::create(output_path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    // 流式下载
    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Failed to read chunk: {}", e))?;
        file.write_all(&chunk)
            .map_err(|e| format!("Failed to write chunk: {}", e))?;
        
        downloaded += chunk.len() as u64;
        
        // 进度
        if total_size > 0 {
            let progress = (downloaded as f64 / total_size as f64 * 100.0) as u32;
            println!("Download progress: {}%", progress);
        }
    }
    
    Ok(())
}

/// 获取最新的release版本并下载
pub async fn download_latest_release(output_dir: &Path) -> Result<String, String> {
    // 获取版本清单
    let manifest = fetch_version_manifest().await?;
    
    // 找到最新的release版本
    let latest_release = manifest.versions
        .iter()
        .find(|v| v.id == manifest.latest.release)
        .ok_or("Latest release version not found")?;
    
    // 获取版本详细信息
    let details = fetch_version_details(&latest_release.url).await?;
    
    // 获取客户端下载链接
    let client_download = details.downloads.client
        .ok_or("Client download not available")?;
    
    // 构建输出路径
    let output_path = output_dir.join(format!("{}.jar", details.id));
    
    // 检查文件是否已存在(缓存)
    if output_path.exists() {
        println!("Using cached jar file: {:?}", output_path);
        return Ok(details.id);
    }
    
    // 下载jar文件
    download_jar_with_progress(&client_download.url, &output_path).await?;
    
    Ok(details.id)
}
/// 下载指定版本
pub async fn download_version(
    version_id: &str,
    output_dir: &Path,
) -> Result<String, String> {
    // 获取版本清单
    let manifest = fetch_version_manifest().await?;
    
    // 找到指定版本
    let version = manifest.versions
        .iter()
        .find(|v| v.id == version_id)
        .ok_or(format!("Version {} not found", version_id))?;
    
    // 获取版本详细信息
    let details = fetch_version_details(&version.url).await?;
    
    // 获取客户端下载链接
    let client_download = details.downloads.client
        .ok_or("Client download not available")?;
    
    // 构建输出路径
    let output_path = output_dir.join(format!("{}.jar", details.id));
    
    // 检查文件是否已存在(缓存)
    if output_path.exists() {
        println!("Using cached jar file: {:?}", output_path);
        return Ok(output_path.to_string_lossy().to_string());
    }
    
    // 下载jar文件
    download_jar_with_progress(&client_download.url, &output_path).await?;
    
    Ok(output_path.to_string_lossy().to_string())
}

/// 从jar文件中提取assets文件夹
pub fn extract_assets_from_jar(jar_path: &Path, output_dir: &Path) -> Result<(), String> {
    use std::fs::File;
    use std::io::Read;
    use zip::ZipArchive;
    
    // 打开jar文件
    let file = File::open(jar_path)
        .map_err(|e| format!("Failed to open jar file: {}", e))?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read jar archive: {}", e))?;
    
    // 遍历所有文件
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to read file from archive: {}", e))?;
        
        let file_path = file.name().to_string();
        
        // 只提取assets目录下的文件
        if file_path.starts_with("assets/") {
            let output_path = output_dir.join(&file_path);
            
            if file.is_dir() {
                // 创建目录
                std::fs::create_dir_all(&output_path)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            } else {
                // 确保父目录存在
                if let Some(parent) = output_path.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create parent directory: {}", e))?;
                }
                
                // 写入文件
                let mut output_file = File::create(&output_path)
                    .map_err(|e| format!("Failed to create output file: {}", e))?;
                
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)
                    .map_err(|e| format!("Failed to read file content: {}", e))?;
                
                std::io::Write::write_all(&mut output_file, &buffer)
                    .map_err(|e| format!("Failed to write file: {}", e))?;
            }
        }
    }
    
    Ok(())
}

/// 下载版本并提取assets
pub async fn download_and_extract_version(
    version_id: &str,
    temp_dir: &Path,
    output_dir: &Path,
    keep_cache: bool,
) -> Result<String, String> {
    // 下载jar文件
    let jar_path = download_version(version_id, temp_dir).await?;
    
    // 提取assets
    extract_assets_from_jar(Path::new(&jar_path), output_dir)?;
    
    // 根据设置决定是否删除jar文件
    if !keep_cache {
        std::fs::remove_file(&jar_path).ok();
    }
    
    Ok(format!("Successfully extracted assets from version {}", version_id))
}

/// 清理缓存的jar文件
pub fn clear_template_cache(temp_dir: &Path) -> Result<(), String> {
    if !temp_dir.exists() {
        return Ok(());
    }
    
    let entries = std::fs::read_dir(temp_dir)
        .map_err(|e| format!("Failed to read temp directory: {}", e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("jar") {
            std::fs::remove_file(&path).ok();
        }
    }
    
    Ok(())
}