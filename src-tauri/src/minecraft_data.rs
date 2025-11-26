use serde::{Deserialize, Serialize};

/// 物品/方块ID数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftItem {
    pub id: String,
    pub name: String,
    pub category: ItemCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ItemCategory {
    Block,
    Item,
    Tool,
    Weapon,
    Armor,
    Food,
    Decoration,
    Redstone,
    Transportation,
    Misc,
}

/// 获取所有物品/方块ID
pub fn get_all_items() -> Vec<MinecraftItem> {
    vec![
        // 方块
        MinecraftItem { id: "stone".to_string(), name: "石头".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "granite".to_string(), name: "花岗岩".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "polished_granite".to_string(), name: "磨制花岗岩".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "diorite".to_string(), name: "闪长岩".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "polished_diorite".to_string(), name: "磨制闪长岩".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "andesite".to_string(), name: "安山岩".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "polished_andesite".to_string(), name: "磨制安山岩".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "grass_block".to_string(), name: "草方块".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "dirt".to_string(), name: "泥土".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "coarse_dirt".to_string(), name: "砂土".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "podzol".to_string(), name: "灰化土".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "cobblestone".to_string(), name: "圆石".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "oak_planks".to_string(), name: "橡木木板".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "spruce_planks".to_string(), name: "云杉木板".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "birch_planks".to_string(), name: "白桦木板".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "jungle_planks".to_string(), name: "丛林木板".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "acacia_planks".to_string(), name: "金合欢木板".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "dark_oak_planks".to_string(), name: "深色橡木木板".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "crimson_planks".to_string(), name: "绯红木板".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "warped_planks".to_string(), name: "诡异木板".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "bedrock".to_string(), name: "基岩".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "sand".to_string(), name: "沙子".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "red_sand".to_string(), name: "红沙".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "gravel".to_string(), name: "沙砾".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "gold_ore".to_string(), name: "金矿石".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "deepslate_gold_ore".to_string(), name: "深层金矿石".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "iron_ore".to_string(), name: "铁矿石".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "deepslate_iron_ore".to_string(), name: "深层铁矿石".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "coal_ore".to_string(), name: "煤矿石".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "deepslate_coal_ore".to_string(), name: "深层煤矿石".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "oak_log".to_string(), name: "橡木原木".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "spruce_log".to_string(), name: "云杉原木".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "birch_log".to_string(), name: "白桦原木".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "jungle_log".to_string(), name: "丛林原木".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "acacia_log".to_string(), name: "金合欢原木".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "dark_oak_log".to_string(), name: "深色橡木原木".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "glass".to_string(), name: "玻璃".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "lapis_ore".to_string(), name: "青金石矿石".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "deepslate_lapis_ore".to_string(), name: "深层青金石矿石".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "sandstone".to_string(), name: "砂岩".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "wool".to_string(), name: "白色羊毛".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "gold_block".to_string(), name: "金块".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "iron_block".to_string(), name: "铁块".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "diamond_block".to_string(), name: "钻石块".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "emerald_block".to_string(), name: "绿宝石块".to_string(), category: ItemCategory::Block },
        MinecraftItem { id: "netherite_block".to_string(), name: "下界合金块".to_string(), category: ItemCategory::Block },
        
        // 工具
        MinecraftItem { id: "wooden_pickaxe".to_string(), name: "木镐".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "stone_pickaxe".to_string(), name: "石镐".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "iron_pickaxe".to_string(), name: "铁镐".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "golden_pickaxe".to_string(), name: "金镐".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "diamond_pickaxe".to_string(), name: "钻石镐".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "netherite_pickaxe".to_string(), name: "下界合金镐".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "wooden_axe".to_string(), name: "木斧".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "stone_axe".to_string(), name: "石斧".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "iron_axe".to_string(), name: "铁斧".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "golden_axe".to_string(), name: "金斧".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "diamond_axe".to_string(), name: "钻石斧".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "netherite_axe".to_string(), name: "下界合金斧".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "wooden_shovel".to_string(), name: "木锹".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "stone_shovel".to_string(), name: "石锹".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "iron_shovel".to_string(), name: "铁锹".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "golden_shovel".to_string(), name: "金锹".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "diamond_shovel".to_string(), name: "钻石锹".to_string(), category: ItemCategory::Tool },
        MinecraftItem { id: "netherite_shovel".to_string(), name: "下界合金锹".to_string(), category: ItemCategory::Tool },
        
        // 武器
        MinecraftItem { id: "wooden_sword".to_string(), name: "木剑".to_string(), category: ItemCategory::Weapon },
        MinecraftItem { id: "stone_sword".to_string(), name: "石剑".to_string(), category: ItemCategory::Weapon },
        MinecraftItem { id: "iron_sword".to_string(), name: "铁剑".to_string(), category: ItemCategory::Weapon },
        MinecraftItem { id: "golden_sword".to_string(), name: "金剑".to_string(), category: ItemCategory::Weapon },
        MinecraftItem { id: "diamond_sword".to_string(), name: "钻石剑".to_string(), category: ItemCategory::Weapon },
        MinecraftItem { id: "netherite_sword".to_string(), name: "下界合金剑".to_string(), category: ItemCategory::Weapon },
        MinecraftItem { id: "bow".to_string(), name: "弓".to_string(), category: ItemCategory::Weapon },
        MinecraftItem { id: "crossbow".to_string(), name: "弩".to_string(), category: ItemCategory::Weapon },
        MinecraftItem { id: "trident".to_string(), name: "三叉戟".to_string(), category: ItemCategory::Weapon },
        
        // 盔甲
        MinecraftItem { id: "leather_helmet".to_string(), name: "皮革帽子".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "leather_chestplate".to_string(), name: "皮革外套".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "leather_leggings".to_string(), name: "皮革裤子".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "leather_boots".to_string(), name: "皮革靴子".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "iron_helmet".to_string(), name: "铁头盔".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "iron_chestplate".to_string(), name: "铁胸甲".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "iron_leggings".to_string(), name: "铁护腿".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "iron_boots".to_string(), name: "铁靴子".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "diamond_helmet".to_string(), name: "钻石头盔".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "diamond_chestplate".to_string(), name: "钻石胸甲".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "diamond_leggings".to_string(), name: "钻石护腿".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "diamond_boots".to_string(), name: "钻石靴子".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "netherite_helmet".to_string(), name: "下界合金头盔".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "netherite_chestplate".to_string(), name: "下界合金胸甲".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "netherite_leggings".to_string(), name: "下界合金护腿".to_string(), category: ItemCategory::Armor },
        MinecraftItem { id: "netherite_boots".to_string(), name: "下界合金靴子".to_string(), category: ItemCategory::Armor },
        
        // 食物
        MinecraftItem { id: "apple".to_string(), name: "苹果".to_string(), category: ItemCategory::Food },
        MinecraftItem { id: "golden_apple".to_string(), name: "金苹果".to_string(), category: ItemCategory::Food },
        MinecraftItem { id: "bread".to_string(), name: "面包".to_string(), category: ItemCategory::Food },
        MinecraftItem { id: "cooked_beef".to_string(), name: "熟牛肉".to_string(), category: ItemCategory::Food },
        MinecraftItem { id: "cooked_porkchop".to_string(), name: "熟猪排".to_string(), category: ItemCategory::Food },
        MinecraftItem { id: "cooked_chicken".to_string(), name: "熟鸡肉".to_string(), category: ItemCategory::Food },
        MinecraftItem { id: "cooked_mutton".to_string(), name: "熟羊肉".to_string(), category: ItemCategory::Food },
        MinecraftItem { id: "cooked_rabbit".to_string(), name: "熟兔肉".to_string(), category: ItemCategory::Food },
        MinecraftItem { id: "cooked_cod".to_string(), name: "熟鳕鱼".to_string(), category: ItemCategory::Food },
        MinecraftItem { id: "cooked_salmon".to_string(), name: "熟鲑鱼".to_string(), category: ItemCategory::Food },
        
        // 物品
        MinecraftItem { id: "coal".to_string(), name: "煤炭".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "charcoal".to_string(), name: "木炭".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "diamond".to_string(), name: "钻石".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "emerald".to_string(), name: "绿宝石".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "iron_ingot".to_string(), name: "铁锭".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "gold_ingot".to_string(), name: "金锭".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "netherite_ingot".to_string(), name: "下界合金锭".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "stick".to_string(), name: "木棍".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "string".to_string(), name: "线".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "feather".to_string(), name: "羽毛".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "gunpowder".to_string(), name: "火药".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "wheat".to_string(), name: "小麦".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "wheat_seeds".to_string(), name: "小麦种子".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "ender_pearl".to_string(), name: "末影珍珠".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "blaze_rod".to_string(), name: "烈焰棒".to_string(), category: ItemCategory::Item },
        MinecraftItem { id: "nether_star".to_string(), name: "下界之星".to_string(), category: ItemCategory::Item },
        
        // 红石
        MinecraftItem { id: "redstone".to_string(), name: "红石粉".to_string(), category: ItemCategory::Redstone },
        MinecraftItem { id: "redstone_torch".to_string(), name: "红石火把".to_string(), category: ItemCategory::Redstone },
        MinecraftItem { id: "repeater".to_string(), name: "红石中继器".to_string(), category: ItemCategory::Redstone },
        MinecraftItem { id: "comparator".to_string(), name: "红石比较器".to_string(), category: ItemCategory::Redstone },
        MinecraftItem { id: "piston".to_string(), name: "活塞".to_string(), category: ItemCategory::Redstone },
        MinecraftItem { id: "sticky_piston".to_string(), name: "粘性活塞".to_string(), category: ItemCategory::Redstone },
        MinecraftItem { id: "dispenser".to_string(), name: "发射器".to_string(), category: ItemCategory::Redstone },
        MinecraftItem { id: "dropper".to_string(), name: "投掷器".to_string(), category: ItemCategory::Redstone },
        MinecraftItem { id: "hopper".to_string(), name: "漏斗".to_string(), category: ItemCategory::Redstone },
        
        // 交通
        MinecraftItem { id: "minecart".to_string(), name: "矿车".to_string(), category: ItemCategory::Transportation },
        MinecraftItem { id: "oak_boat".to_string(), name: "橡木船".to_string(), category: ItemCategory::Transportation },
        MinecraftItem { id: "elytra".to_string(), name: "鞘翅".to_string(), category: ItemCategory::Transportation },
        MinecraftItem { id: "saddle".to_string(), name: "鞍".to_string(), category: ItemCategory::Transportation },
    ]
}

/// 按类别获取物品
pub fn get_items_by_category(category: ItemCategory) -> Vec<MinecraftItem> {
    get_all_items()
        .into_iter()
        .filter(|item| item.category == category)
        .collect()
}

/// 搜索物品
pub fn search_items(query: &str) -> Vec<MinecraftItem> {
    let query_lower = query.to_lowercase();
    get_all_items()
        .into_iter()
        .filter(|item| {
            item.id.to_lowercase().contains(&query_lower)
                || item.name.to_lowercase().contains(&query_lower)
        })
        .collect()
}