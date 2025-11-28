# Minecraft Resourcespack Editor

<div align="center">
  <img src="src/assets/grass-block.png" alt="Minecraft" width="100"/>
  <h2>一个功能强大的 Minecraft 资源包编辑器</h2>
  <h3>支持最新版本的材质包格式，让您轻松创建和编辑资源包</h3>
  <p>此文档由Ai生成(未经过审查)</p>
</div>

## ✨ 功能特性

- 🎨 **可视化编辑** - 直观的图形界面，支持实时预览
- 📦 **多种导入方式** - 支持从 ZIP 文件或文件夹导入资源包
- 🔧 **完整编辑功能** - 编辑材质、模型、音效等资源文件
- 🎯 **元数据管理** - 轻松编辑 pack.mcmeta 配置
- 🖼️ **图像编辑器** - 内置图像查看和编辑工具
- 📝 **代码编辑器** - 支持 JSON、MCMETA 等文件的语法高亮
- 🌐 **Web 服务** - 可选的局域网/公网访问功能
- 🎨 **主题切换** - 支持亮色/暗色主题
- 💾 **历史记录** - 支持编辑历史的撤销/重做
- 🔄 **模板缓存** - 智能缓存 Minecraft 版本文件

## 🚀 快速开始

### 环境要求

- Node.js 16+
- Rust 1.70+
- Windows 10/11 (当前版本)

### 安装依赖

```bash
# 安装前端依赖
npm install

# 安装 Tauri CLI (如果尚未安装)
npm install -g @tauri-apps/cli
```

### 开发模式

```bash
# 启动开发服务器
npm run dev

# 或者只启动 Vite 开发服务器
npm run dev:vite
```

### 构建应用

```bash
# 构建生产版本
npm run build

# 预览构建结果
npm run preview
```

## 📖 使用指南

### 导入资源包

1. **从文件夹导入** - 选择包含资源包文件的文件夹
2. **从 ZIP 导入** - 选择 .zip 格式的资源包文件
3. **从零创建** - 创建全新的资源包项目

### 编辑资源

- 在左侧文件树中浏览资源文件
- 双击文件进行编辑
- 支持的文件类型：
  - 图像文件 (.png, .jpg, .jpeg)
  - JSON 文件 (.json, .mcmeta)
  - 文本文件 (.txt, .md)

### 导出资源包

编辑完成后，点击"导出"按钮将资源包保存为 ZIP 文件。

## 🛠️ 技术栈

### 前端
- **React 19** - UI 框架
- **TypeScript** - 类型安全
- **Vite** - 构建工具

### 后端
- **Tauri 2** - 桌面应用框架
- **Rust** - 系统级编程语言
- **Axum** - Web 服务器框架 (可选)

### 主要依赖
- `@tauri-apps/api` - Tauri API
- `@tauri-apps/plugin-dialog` - 文件对话框
- `@tauri-apps/plugin-shell` - Shell 命令
- `image` - 图像处理
- `zip` - ZIP 文件处理
- `serde` - 序列化/反序列化

## 📁 项目结构

```
minecraft-pack-editor/
├── src/                    # 前端源代码
│   ├── components/         # React 组件
│   ├── types/             # TypeScript 类型定义
│   ├── utils/             # 工具函数
│   └── assets/            # 静态资源
├── src-tauri/             # Tauri 后端代码
│   ├── src/               # Rust 源代码
│   │   ├── commands.rs    # Tauri 命令
│   │   ├── pack_parser.rs # 资源包解析
│   │   ├── image_handler.rs # 图像处理
│   │   └── ...
│   └── Cargo.toml         # Rust 依赖配置
├── public/                # 公共资源
└── package.json           # Node.js 依赖配置
```

## ⚙️ 配置选项

### 主题设置
- 跟随系统
- 亮色模式
- 暗色模式

### 窗口效果
- 亚克力效果 (Windows 11)

### 编辑历史
- 启用/禁用历史记录
- 配置历史记录数量 (10-50)

### Web 服务
- 关闭
- 仅局域网
- 全部网络

### 模板缓存
- 启用/禁用 Minecraft 版本文件缓存

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

本项目采用 GPLv3 许可证。

## 👤 作者

**Little_100**

- Bilibili: [@Little_100](https://space.bilibili.com/1492647738)
- GitHub: [@little100](https://github.com/little100)

---

<div align="center">
  Made with ❤️ by Little_100
</div>