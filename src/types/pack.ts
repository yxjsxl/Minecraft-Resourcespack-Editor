// 版本枚举
export enum MinecraftVersion {
  Legacy = "Legacy",
  Flattening = "Flattening",
  Components = "Components",
  NewModel = "NewModel",
  ItemsFolder = "ItemsFolder",
}

// 资源类型
export enum ResourceType {
  Texture = "Texture",
  Model = "Model",
  ItemModel = "ItemModel",
  BlockState = "BlockState",
  Sound = "Sound",
  Language = "Language",
  Font = "Font",
  Shader = "Shader",
  Other = "Other",
}

// 资源文件信息
export interface ResourceFile {
  path: string;
  relative_path: string;
  resource_type: ResourceType;
  namespace: string;
  name: string;
  size: number;
}

// 材质包信息
export interface PackInfo {
  name: string;
  version: MinecraftVersion;
  pack_format: number;
  description: string;
  resources: Record<ResourceType, ResourceFile[]>;
  namespaces: string[];
}

// 图片信息
export interface ImageInfo {
  width: number;
  height: number;
  format: string;
  size_bytes: number;
  is_valid_texture: boolean;
}

// 版本描述映射
export const VERSION_DESCRIPTIONS: Record<MinecraftVersion, string> = {
  [MinecraftVersion.Legacy]: "1.6-1.12 (Legacy)",
  [MinecraftVersion.Flattening]: "1.13-1.19.3 (Flattening)",
  [MinecraftVersion.Components]: "1.19.4-1.20.4 (Components)",
  [MinecraftVersion.NewModel]: "1.20.5-1.21.3 (New Components)",
  [MinecraftVersion.ItemsFolder]: "1.21.4+ (Items Folder)",
};

// 资源类型显示名称
export const RESOURCE_TYPE_NAMES: Record<ResourceType, string> = {
  [ResourceType.Texture]: "纹理",
  [ResourceType.Model]: "模型",
  [ResourceType.ItemModel]: "物品模型 (1.21.4+)",
  [ResourceType.BlockState]: "方块状态",
  [ResourceType.Sound]: "音效",
  [ResourceType.Language]: "语言文件",
  [ResourceType.Font]: "字体",
  [ResourceType.Shader]: "着色器",
  [ResourceType.Other]: "其他",
};