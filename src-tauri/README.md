```text
src/
├── main.rs                  # 程序入口，初始化Tauri应用、注册IPC指令
├── ipc/                     # IPC通信层
│   ├── mod.rs               # 导出IPC相关模块和接口
│   ├── commands.rs          # 定义前端可调用的IPC指令（如播放/暂停等）
│   ├── events.rs            # 定义后端主动推送的事件（如播放状态变化）
│   └── types.rs             # 跨端数据结构定义（使用serde序列化）
│
├── app/                     # 应用控制层
│   ├── mod.rs               # 导出应用控制相关模块
│   ├── window.rs            # 窗口样式与行为控制
│   └── tray.rs              # 系统托盘相关（最小化到托盘等）
│
├── core/                    # 核心业务逻辑层
│   ├── mod.rs               # 导出核心模块
│   ├── player/              # 播放控制模块
│   │   ├── mod.rs
│   │   ├── controller.rs    # 播放/暂停/进度调节等控制逻辑
│   │   ├── audio_backend.rs # 音频库适配（rodio/symphonia集成）
│   │   └── state.rs         # 播放状态管理（当前进度、音量等）
│   │
│   ├── library/             # 音乐库管理模块
│   │   ├── mod.rs
│   │   ├── scanner.rs       # 本地文件扫描逻辑
│   │   ├── metadata.rs      # 音频元数据解析
│   │   ├── index.rs         # 音乐库索引与缓存管理
│   │   └── search.rs        # 音乐检索功能
│   │
│   ├── playlist/            # 播放列表模块
│   │   ├── mod.rs
│   │   ├── manager.rs       # 播放列表CRUD操作
│   │   ├── play_mode.rs     # 播放模式（顺序/循环/随机）实现
│   │   └── persistence.rs   # 播放列表持久化存储
│   │
│   ├── state/               # 全局状态管理
│   │   ├── mod.rs
│   │   ├── global.rs        # 全局状态定义与访问接口
│   │   └── observer.rs      # 状态变更通知机制（观察者模式）
│   │
│   └── task_queue/          # 异步任务队列模块
│       ├── mod.rs
│       ├── queue.rs         # 任务队列实现
│       ├── task.rs          # 任务定义与执行
│       ├── tauri_integration.rs        # Tauri与任务队列集成
│       └── tracker.rs       # 任务执行状态跟踪
├── utils/                   # 工具与支撑层
│   ├── mod.rs
│   ├── config.rs            # 应用配置管理
│   ├── error.rs             # 自定义错误类型与处理
│   ├── logger.rs            # 日志初始化与配置
│   ├── fs.rs                # 文件系统工具（跨平台路径处理等）
│   └── lifecycle.rs         # 应用生命周期管理
│
└── assets/                  # 静态资源（可选）
    └── icons/               # 应用图标等资源
```

more theme
```text
            "micaLight",
            "micaDark",
            "tabbed",
            "tabbedLight",
            "tabbedDark",
            "blur",
            "acrylic"
```