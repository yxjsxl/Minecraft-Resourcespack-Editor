use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

/// 解压ZIP文件到指定目录
pub fn extract_zip(zip_path: &Path, extract_to: &Path) -> Result<(), String> {
    let file = File::open(zip_path)
        .map_err(|e| format!("Failed to open zip file: {}", e))?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read zip archive: {}", e))?;

    // 创建目标目录
    fs::create_dir_all(extract_to)
        .map_err(|e| format!("Failed to create extract directory: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to read file from archive: {}", e))?;
        
        let outpath = match file.enclosed_name() {
            Some(path) => extract_to.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            // 创建目录
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        } else {
            // 创建父目录
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directory: {}", e))?;
            }
            
            // 写入文件
            let mut outfile = File::create(&outpath)
                .map_err(|e| format!("Failed to create file: {}", e))?;
            
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .map_err(|e| format!("Failed to read file content: {}", e))?;
            
            outfile.write_all(&buffer)
                .map_err(|e| format!("Failed to write file: {}", e))?;
        }
    }

    Ok(())
}

/// 将目录打包为ZIP文件
pub fn create_zip(source_dir: &Path, output_path: &Path) -> Result<(), String> {
    let file = File::create(output_path)
        .map_err(|e| format!("Failed to create zip file: {}", e))?;
    
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::FileOptions::<()>::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let walkdir = walkdir::WalkDir::new(source_dir);
    let it = walkdir.into_iter().filter_map(|e| e.ok());

    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(source_dir)
            .map_err(|e| format!("Failed to strip prefix: {}", e))?;

        // 跳过根目录
        if name.as_os_str().is_empty() {
            continue;
        }

        let name_str = name.to_string_lossy().replace('\\', "/");

        if path.is_file() {
            zip.start_file(&name_str, options)
                .map_err(|e| format!("Failed to start file in zip: {}", e))?;
            
            let mut f = File::open(path)
                .map_err(|e| format!("Failed to open file: {}", e))?;
            
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)
                .map_err(|e| format!("Failed to read file: {}", e))?;
            
            zip.write_all(&buffer)
                .map_err(|e| format!("Failed to write to zip: {}", e))?;
        } else if path.is_dir() {
            zip.add_directory(&name_str, options)
                .map_err(|e| format!("Failed to add directory to zip: {}", e))?;
        }
    }

    zip.finish()
        .map_err(|e| format!("Failed to finish zip: {}", e))?;

    Ok(())
}

/// 验证是否为有效的材质包ZIP
pub fn validate_pack_zip(zip_path: &Path) -> Result<bool, String> {
    let file = File::open(zip_path)
        .map_err(|e| format!("Failed to open zip file: {}", e))?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read zip archive: {}", e))?;

    // 检查是否包含pack.mcmeta
    for i in 0..archive.len() {
        let file = archive.by_index(i)
            .map_err(|e| format!("Failed to read file from archive: {}", e))?;
        
        if file.name() == "pack.mcmeta" || file.name().ends_with("/pack.mcmeta") {
            return Ok(true);
        }
    }

    Ok(false)
}

/// 获取临时解压目录
pub fn get_temp_extract_dir() -> PathBuf {
    let temp_dir = std::env::temp_dir();
    temp_dir.join("minecraft_pack_editor")
}

/// 清理临时文件
pub fn cleanup_temp_files() -> Result<(), String> {
    let temp_dir = get_temp_extract_dir();
    
    let system_temp = std::env::temp_dir();
    
    if temp_dir.exists() && temp_dir.starts_with(&system_temp) {
        eprintln!("Cleaning up temp directory: {:?}", temp_dir);
        fs::remove_dir_all(&temp_dir)
            .map_err(|e| format!("Failed to cleanup temp files: {}", e))?;
    } else {
        eprintln!("Skipping cleanup: temp_dir is not in system temp or doesn't exist");
    }
    Ok(())
}