<p align="center">
  <a href="https://github.com/SonusTeam/Sonus">
    <img src="https://file.lingke.ink/sonus/sonus-zh.webp" alt="Sonus 标志">
  </a>
</p>

<h1 align="center">Sonus</h1>

<p align="center">
  <a href="https://github.com/SonusTeam/Sonus/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/SonusTeam/Sonus/build.yml?style=flat-square" alt="构建状态">
  </a>
  <a href="https://github.com/SonusTeam/Sonus/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/SonusTeam/Sonus?style=flat-square" alt="许可证">
  </a>
  <a href="https://github.com/SonusTeam/Sonus/releases">
    <img src="https://img.shields.io/github/v/release/SonusTeam/Sonus?include_prereleases&style=flat-square" alt="最新版本">
  </a>
  <a href="https://github.com/SonusTeam/Sonus/stargazers">
    <img src="https://img.shields.io/github/stars/SonusTeam/Sonus?style=flat-square" alt="星标数">
  </a>
  <a href="https://github.com/SonusTeam/Sonus/issues">
    <img src="https://img.shields.io/github/issues/SonusTeam/Sonus?style=flat-square" alt="问题数">
  </a>
  <a href="https://discord.gg/Udq8xrruA3">
    <img src="https://img.shields.io/discord/yourserverid?style=flat-square&label=Discord" alt="Discord 社群">
  </a>
</p>

<p align="center">
  <a href="https://github.com/SonusTeam/Sonus/blob/master/README.md">English</a> | 简体中文
</p>

Sonus 是一款基于 Tauri + Rust 开发的**开源轻量型跨平台桌面应用**，作为私人音乐库管理播放器，它不仅能管理运行设备本地存储的音乐，还可通过 WebDAV、SMB 协议，对家庭私有云（NAS）或远程服务器中的音乐进行本地化管理。


## ✨ 核心功能

- **本地与网络音乐管理**
  - 整理并播放本地存储中的音乐
  - 连接 WebDAV、SMB 服务器，实现远程音乐管理
  - 自动提取并整理音乐元数据（如歌手、专辑信息等）

- **强大的播放功能**
  - 支持常见音频格式（MP3、FLAC、WAV 等）
  - 提供多种播放模式（循环、随机等）
  - 高品质音频输出

- **现代化界面设计**
  - 简洁直观的操作界面
  - 支持亮色/暗色模式切换
  - 可自定义主题
  - 适配不同窗口尺寸的响应式设计

- **进阶音乐库功能**
  - 按歌手、专辑、流派等维度搜索筛选
  - 创建并管理播放列表
  - 专辑封面显示与整理


## 📥 安装说明

### Windows 系统
1. 从 [发布页面](https://github.com/SonusTeam/Sonus/releases) 下载最新安装包
2. 运行安装包，按照屏幕提示完成安装

### macOS 系统
- 将于 1.0 正式版中新增支持

### Linux 系统
- 将于 1.0 正式版中新增支持


## 🚀 快速上手

1. 安装完成后启动 Sonus
2. 添加你的音乐库：
   - 点击「设置」>「音乐库」
   - 添加本地文件夹，或连接 WebDAV/SMB 服务器
3. 等待 Sonus 扫描并索引你的音乐集
4. 浏览音乐库、创建播放列表，开始享受音乐


## 🔧 开发指南

### 前置依赖
- [Rust](https://www.rust-lang.org/tools/install)（编程语言环境）
- [Node.js](https://nodejs.org/)（JavaScript 运行环境）
- [Tauri CLI](https://tauri.app/v2/guides/getting-started/prerequisites/)（Tauri 开发工具）

### 环境搭建
```bash
# 克隆代码仓库
git clone https://github.com/SonusTeam/Sonus.git
cd Sonus

# 安装依赖包
pnpm install

# 启动开发服务器
pnpm tauri dev
```

### 构建生产版本
```bash
# 构建生产环境安装包
pnpm tauri build
```


## 📋 开发路线图
目前项目处于初期开发阶段，重点规划功能如下：
- 完善 WebDAV 与 SMB 协议支持
- 新增进阶音频质量设置
- 支持快捷键自定义配置
- 增强主题自定义功能
- 实现 macOS、Linux 跨平台支持
- 增加更多元数据管理工具

如需查看当前开发任务详情，可参考项目 TODO 列表。


## 🤝 贡献指南
欢迎参与项目贡献！提交拉取请求（PR）前，请先阅读 [贡献指南](https://github.com/SonusTeam/Sonus/CONTRIBUTING.md)。

1. Fork 本项目仓库
2. 创建功能分支（`git checkout -b feature/AmazingFeature`）
3. 提交代码修改（`git commit -m 'Add some AmazingFeature'`）
4. 推送分支至远程仓库（`git push origin feature/AmazingFeature`）
5. 发起拉取请求（Pull Request）


## 🐛 问题反馈
如遇到任何问题，请前往 [问题追踪器](https://github.com/SonusTeam/Sonus/issues) 提交反馈。


## 📄 许可证
Sonus 基于 [GNU 通用公共许可证 v3.0](https://github.com/SonusTeam/Sonus/LICENSE) 授权开源。


## 💬 社群交流

- Discord - 加入社群聊天（链接见上方）
- QQ 群 - 755353142


## 🙏 鸣谢

Sonus 感谢 JetBrains 提供的 RustRover IDE 对开源项目的支持

<p align="center">
由 Sonus 团队及贡献者们 用心打造 ❤️。
</p>