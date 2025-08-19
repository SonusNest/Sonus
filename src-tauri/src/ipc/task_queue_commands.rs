use tauri::{State, Window};
use crate::core::task_queue::{TaskQueueHandle, TaskStats, TaskTracker};
use tracing::info;

/// Tauri命令：开始扫描目录
#[tauri::command]
pub async fn start_directory_scan(
    path: String,
    queue_handle: State<'_, TaskQueueHandle>
) -> Result<(), String> {
    // 创建目录扫描任务
    info!("进入 start_directory_scan 方法体");
    info!("准备创建 DirectoryScanTask，path: {}", path);
    let scan_task = Box::new(super::super::core::library::scanner::DirectoryScanTask::new(path));
    info!("DirectoryScanTask 创建成功");
    info!("准备提交任务到队列");
    // 提交任务到队列
    queue_handle.submit_task(scan_task).await;
    info!("任务提交完成");
    info!("即将返回 Ok(())");
    Ok(())
}

/// Tauri命令：获取当前任务统计信息
#[tauri::command]
pub async fn get_task_stats(tracker: State<'_, TaskTracker>) -> Result<TaskStats, String> {
    Ok(tracker.get_stats().await)
}

/// Tauri命令：注册任务事件监听
#[tauri::command]
pub fn register_task_listener(window: Window) {
    // 实际应用中可以在这里做一些初始化工作
    println!("前端已注册任务事件监听");
}