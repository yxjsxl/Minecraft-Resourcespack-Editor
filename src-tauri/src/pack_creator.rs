use std::fs;
use std::path::Path;
use serde_json::json;

/// 创建新的材质包
pub fn create_new_pack(
    output_path: &Path,
    _pack_name: &str,
    pack_format: i32,
    description: &str,
) -> Result<(), String> {
    // 创建主目录
    fs::create_dir_all(output_path)
        .map_err(|e| format!("Failed to create pack directory: {}", e))?;

    // 创建 pack.mcmeta
    let pack_mcmeta = json!({
        "pack": {
            "pack_format": pack_format,
            "description": description
        }
    });

    let mcmeta_path = output_path.join("pack.mcmeta");
    fs::write(
        mcmeta_path,
        serde_json::to_string_pretty(&pack_mcmeta)
            .map_err(|e| format!("Failed to serialize pack.mcmeta: {}", e))?,
    )
    .map_err(|e| format!("Failed to write pack.mcmeta: {}", e))?;

    // 创建 assets/minecraft 目录结构
    let assets_path = output_path.join("assets").join("minecraft");
    
    // 创建标准目录
    let directories = vec![
        "textures/block",
        "textures/item",
        "textures/entity",
        "textures/gui",
        "models/block",
        "models/item",
        "blockstates",
        "sounds",
        "lang",
        "font",
        "shaders",
    ];

    // 如果是1.21.4+版本,创建items目录
    if pack_format >= 35 {
        fs::create_dir_all(assets_path.join("items"))
            .map_err(|e| format!("Failed to create items directory: {}", e))?;
    }

    for dir in directories {
        fs::create_dir_all(assets_path.join(dir))
            .map_err(|e| format!("Failed to create directory {}: {}", dir, e))?;
    }

    // 创建 pack.png (可选的图标)
    create_default_pack_icon(output_path)?;

    Ok(())
}

/// 创建默认的材质包图标
fn create_default_pack_icon(output_path: &Path) -> Result<(), String> {
    // 创建一个简单的64x64像素的图标
    use image::{ImageBuffer, Rgba};

    let img = ImageBuffer::from_fn(64, 64, |x, y| {
        // 创建一个渐变图标的麦块需要的pack.png
        let r = ((x as f32 / 64.0) * 255.0) as u8;
        let g = ((y as f32 / 64.0) * 255.0) as u8;
        let b = 128;
        Rgba([r, g, b, 255])
    });

    let icon_path = output_path.join("pack.png");
    img.save(&icon_path)
        .map_err(|e| format!("Failed to save pack icon: {}", e))?;

    Ok(())
}

/// 为指定物品创建默认模型文件
pub fn create_item_model(
    pack_path: &Path,
    item_id: &str,
    pack_format: i32,
) -> Result<(), String> {
    let assets_path = pack_path.join("assets").join("minecraft");

    if pack_format >= 35 {
        // 1.21.4+ 使用 items/ 文件夹
        let items_path = assets_path.join("items");
        fs::create_dir_all(&items_path)
            .map_err(|e| format!("Failed to create items directory: {}", e))?;

        let model_content = json!({
            "model": {
                "type": "minecraft:model",
                "model": format!("minecraft:item/{}", item_id)
            }
        });

        let model_path = items_path.join(format!("{}.json", item_id));
        fs::write(
            model_path,
            serde_json::to_string_pretty(&model_content)
                .map_err(|e| format!("Failed to serialize item model: {}", e))?,
        )
        .map_err(|e| format!("Failed to write item model: {}", e))?;
    } else {
        // 旧版本使用 models/item/ 文件夹
        let models_path = assets_path.join("models").join("item");
        fs::create_dir_all(&models_path)
            .map_err(|e| format!("Failed to create models directory: {}", e))?;

        let model_content = json!({
            "parent": "item/generated",
            "textures": {
                "layer0": format!("minecraft:item/{}", item_id)
            }
        });

        let model_path = models_path.join(format!("{}.json", item_id));
        fs::write(
            model_path,
            serde_json::to_string_pretty(&model_content)
                .map_err(|e| format!("Failed to serialize item model: {}", e))?,
        )
        .map_err(|e| format!("Failed to write item model: {}", e))?;
    }

    Ok(())
}

/// 为指定方块创建默认模型和方块状态文件
pub fn create_block_model(
    pack_path: &Path,
    block_id: &str,
) -> Result<(), String> {
    let assets_path = pack_path.join("assets").join("minecraft");

    // 创建方块状态文件
    let blockstates_path = assets_path.join("blockstates");
    fs::create_dir_all(&blockstates_path)
        .map_err(|e| format!("Failed to create blockstates directory: {}", e))?;

    let blockstate_content = json!({
        "variants": {
            "": {
                "model": format!("minecraft:block/{}", block_id)
            }
        }
    });

    let blockstate_path = blockstates_path.join(format!("{}.json", block_id));
    fs::write(
        blockstate_path,
        serde_json::to_string_pretty(&blockstate_content)
            .map_err(|e| format!("Failed to serialize blockstate: {}", e))?,
    )
    .map_err(|e| format!("Failed to write blockstate: {}", e))?;

    // 创建方块模型文件
    let models_path = assets_path.join("models").join("block");
    fs::create_dir_all(&models_path)
        .map_err(|e| format!("Failed to create models directory: {}", e))?;

    let model_content = json!({
        "parent": "block/cube_all",
        "textures": {
            "all": format!("minecraft:block/{}", block_id)
        }
    });

    let model_path = models_path.join(format!("{}.json", block_id));
    fs::write(
        model_path,
        serde_json::to_string_pretty(&model_content)
            .map_err(|e| format!("Failed to serialize block model: {}", e))?,
    )
    .map_err(|e| format!("Failed to write block model: {}", e))?;

    // 创建物品模型(方块的物品形式)
    let item_models_path = assets_path.join("models").join("item");
    fs::create_dir_all(&item_models_path)
        .map_err(|e| format!("Failed to create item models directory: {}", e))?;

    let item_model_content = json!({
        "parent": format!("minecraft:block/{}", block_id)
    });

    let item_model_path = item_models_path.join(format!("{}.json", block_id));
    fs::write(
        item_model_path,
        serde_json::to_string_pretty(&item_model_content)
            .map_err(|e| format!("Failed to serialize item model: {}", e))?,
    )
    .map_err(|e| format!("Failed to write item model: {}", e))?;

    Ok(())
}

/// 批量创建物品模型
pub fn create_multiple_item_models(
    pack_path: &Path,
    item_ids: &[String],
    pack_format: i32,
) -> Result<Vec<String>, String> {
    let mut created = Vec::new();
    let mut errors = Vec::new();

    for item_id in item_ids {
        match create_item_model(pack_path, item_id, pack_format) {
            Ok(_) => created.push(item_id.clone()),
            Err(e) => errors.push(format!("{}: {}", item_id, e)),
        }
    }

    if !errors.is_empty() {
        return Err(format!("Failed to create some models: {}", errors.join(", ")));
    }

    Ok(created)
}

/// 批量创建方块模型
pub fn create_multiple_block_models(
    pack_path: &Path,
    block_ids: &[String],
) -> Result<Vec<String>, String> {
    let mut created = Vec::new();
    let mut errors = Vec::new();

    for block_id in block_ids {
        match create_block_model(pack_path, block_id) {
            Ok(_) => created.push(block_id.clone()),
            Err(e) => errors.push(format!("{}: {}", block_id, e)),
        }
    }

    if !errors.is_empty() {
        return Err(format!("Failed to create some models: {}", errors.join(", ")));
    }

    Ok(created)
}