<p align="center">
  <a href="https://github.com/SonusTeam/Sonus">
    <img src="https://file.lingke.ink/sonus/sonus-en.webp" alt="Sonus Logo" width="200" height="200">
  </a>
</p>

<h1 align="center">Sonus</h1>

<p align="center">
  <a href="https://github.com/SonusTeam/Sonus/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/SonusTeam/Sonus/build.yml?style=flat-square" alt="Build Status">
  </a>
  <a href="https://github.com/SonusTeam/Sonus/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/SonusTeam/Sonus?style=flat-square" alt="License">
  </a>
  <a href="https://github.com/SonusTeam/Sonus/releases">
    <img src="https://img.shields.io/github/v/release/SonusTeam/Sonus?include_prereleases&style=flat-square" alt="Latest Release">
  </a>
  <a href="https://github.com/SonusTeam/Sonus/stargazers">
    <img src="https://img.shields.io/github/stars/SonusTeam/Sonus?style=flat-square" alt="Stars">
  </a>
  <a href="https://github.com/SonusTeam/Sonus/issues">
    <img src="https://img.shields.io/github/issues/SonusTeam/Sonus?style=flat-square" alt="Issues">
  </a>
  <a href="https://discord.gg/yourserver">
    <img src="https://img.shields.io/discord/yourserverid?style=flat-square&label=Discord" alt="Discord">
  </a>
</p>

<p align="center">
  English | <a href="https://github.com/SonusTeam/Sonus/blob/master/README-ZH.md">简体中文</a>
</p>

Sonus is an open-source, lightweight cross-platform desktop application developed with Tauri + Rust. As a private music library management player, it not only manages music stored locally on the running device but also enables localized management of music in home private clouds (NAS) or remote servers via WebDAV and SMB protocols.

## ✨ Features

- **Local & Network Music Management**
  - Organize and play music from local storage
  - Connect to WebDAV and SMB servers for remote music management
  - Automatic metadata extraction and organization

- **Powerful Playback**
  - Support for common audio formats (MP3, FLAC, WAV, etc.)
  - Multiple playback modes (repeat, shuffle, etc.)
  - High-quality audio output

- **Modern UI**
  - Clean and intuitive interface
  - Light/dark mode support
  - Customizable themes
  - Responsive design for different window sizes

- **Advanced Library Features**
  - Search and filter by artist, album, genre, etc.
  - Create and manage playlists
  - Album art display and organization

## 📥 Installation

### Windows
- Download the latest installer from the [releases page](https://github.com/SonusTeam/Sonus/releases)
- Run the installer and follow the on-screen instructions

### macOS
- Support will be added in version 1.0 stable release

### Linux
- Support will be added in version 1.0 stable release

## 🚀 Getting Started

1. Launch Sonus after installation
2. Add your music library:
  - Click on "Settings" > "Library"
  - Add local folders or connect to WebDAV/SMB servers
3. Let Sonus scan and index your music collection
4. Browse your library, create playlists, and enjoy your music

## 🔧 Development

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/)
- [Tauri CLI](https://tauri.app/v2/guides/getting-started/prerequisites/)

### Setup
```bash
# Clone the repository
git clone https://github.com/SonusTeam/Sonus.git
cd Sonus

# Install dependencies
pnpm install

# Start development server
pnpm tauri dev
```

### Building
```bash
# Build for production
pnpm tauri build
```

## 📋 Roadmap
The project is currently in the initial development phase. Key upcoming features include:
- Complete WebDAV and SMB support
- Advanced audio quality settings
- Shortcut key configuration
- Enhanced theme customization
- Cross-platform support for macOS and Linux
- Additional metadata management tools

For a detailed list of current development tasks, see our TODO list.

## 🤝 Contributing
Contributions are welcome! Please read our [Contributing Guide](https://github.com/SonusTeam/Sonus/CONTRIBUTING.md) before submitting a pull request.


1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 🐛 Issues
If you encounter any issues, please report them on our [issue tracker](https://github.com/SonusTeam/Sonus/issues).

## 📄 License
Sonus is licensed under the [GNU General Public License v3.0](https://github.com/SonusTeam/Sonus/LICENSE).

## 💬 Community

- Discord - Join our community chat
- QQ Group - 755353142

## 🙏 Acknowledgements

Sonus thanks JetBrains RustRover IDE for supporting open source projects

<p align="center">
Made with ❤️ by the Sonus Team and Contributors.
</p>

