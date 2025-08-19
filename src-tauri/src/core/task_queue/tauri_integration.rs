//! Tauri集成模块，提供与前端交互的命令和事件

use tauri::{Manager, AppHandle, Runtime};

pub fn init_task_queue<M: Manager<R>, R: Runtime>(app: M) -> Result<(), Box<dyn std::error::Error>> {
    use super::{TaskQueue, TaskTracker};
    use tokio::sync::mpsc;
    use tauri::Emitter;

    // 创建事件通道
    let (event_sender, mut event_receiver) = mpsc::channel(100);

    // 创建任务跟踪器
    let tracker = TaskTracker::new(event_sender);

    // 创建任务队列
    let (mut task_queue, queue_handle) = TaskQueue::new(10, tracker.clone());

    // 存储状态
    app.manage(queue_handle);
    app.manage(tracker.clone());

    // 队列后台执行
    tauri::async_runtime::spawn(async move {
        task_queue.run().await;
        println!("任务队列已退出");
    });

    // ⚠️ 注意：这里要 clone 出一个 `AppHandle<R>`
    let app_handle: AppHandle<R> = app.app_handle().clone();
    tauri::async_runtime::spawn(async move {
        while let Some(event) = event_receiver.recv().await {
            app_handle
                .emit_to("main", "task-event", event)
                .unwrap_or_else(|e| eprintln!("发送任务事件失败: {}", e));
        }
    });

    Ok(())
}