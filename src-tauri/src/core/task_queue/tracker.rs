//! 任务跟踪器，负责记录和更新所有任务的状态
//! 提供任务统计信息和事件通知

use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use serde::{Serialize, Deserialize};
use super::task::{TaskType, Task};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,      // 等待中
    InProgress,   // 进行中
    Completed,    // 已完成
    Failed(String), // 失败，包含错误信息
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "等待中"),
            TaskStatus::InProgress => write!(f, "进行中"),
            TaskStatus::Completed => write!(f, "已完成"),
            TaskStatus::Failed(msg) => write!(f, "失败: {}", msg),
        }
    }
}

/// 任务事件，用于通知前端任务状态变化
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEvent {
    pub task_id: String,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub path: Option<String>,
    pub timestamp: u64,
}

/// 任务统计信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskStats {
    pub total_tasks: usize,
    pub pending_tasks: usize,
    pub in_progress_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
}

/// 任务跟踪器
#[derive(Clone)]
pub struct TaskTracker {
    inner: Arc<TaskTrackerInner>,
}

/// 任务跟踪器内部数据结构
struct TaskTrackerInner {
    tasks: Mutex<Vec<Box<dyn Task>>>,
    stats: Mutex<TaskStats>,
    event_sender: mpsc::Sender<TaskEvent>,
}

impl TaskTracker {
    /// 创建新的任务跟踪器
    pub fn new(event_sender: mpsc::Sender<TaskEvent>) -> Self {
        Self {
            inner: Arc::new(TaskTrackerInner {
                tasks: Mutex::new(Vec::new()),
                stats: Mutex::new(TaskStats::default()),
                event_sender,
            }),
        }
    }

    /// 添加任务到跟踪器
    pub async fn add_task(&self, task: Box<dyn Task>) {
        let mut tasks = self.inner.tasks.lock().await;
        let mut stats = self.inner.stats.lock().await;

        tasks.push(task);
        stats.total_tasks += 1;
        stats.pending_tasks += 1;
    }

    /// 更新任务状态
    pub async fn update_status(&self, task_id: &str, new_status: TaskStatus) {
        let mut tasks = self.inner.tasks.lock().await;
        let mut stats = self.inner.stats.lock().await;
        info!("更新任务状态: {:?}", new_status);
        // 查找任务并更新状态
        for task in tasks.iter_mut() {
            if task.id() == task_id {
                info!("任务类型: {:?}", task.task_type());
                // 获取旧状态用于更新统计
                // let old_status = self.get_task_status(task_id).await;
                let old_status = task.status();
                info!("旧状态已获取");

                task.set_status(new_status.clone());
                // 更新统计信息
                self.update_stats(&mut stats, &old_status, &new_status);
                info!("统计信息已更新");
                // 发送事件通知
                self.send_event(task, &new_status).await;
                info!("事件已发送");
                break;
            }
        }
    }


    /// 更新统计信息
    fn update_stats(&self, stats: &mut TaskStats, old_status: &TaskStatus, new_status: &TaskStatus) {
        // 减少旧状态的计数
        match old_status {
            TaskStatus::Pending => stats.pending_tasks -= 1,
            TaskStatus::InProgress => stats.in_progress_tasks -= 1,
            TaskStatus::Completed => stats.completed_tasks -= 1,
            TaskStatus::Failed(_) => stats.failed_tasks -= 1,
        }

        // 增加新状态的计数
        match new_status {
            TaskStatus::Pending => stats.pending_tasks += 1,
            TaskStatus::InProgress => stats.in_progress_tasks += 1,
            TaskStatus::Completed => stats.completed_tasks += 1,
            TaskStatus::Failed(_) => stats.failed_tasks += 1,
        }
    }

    /// 发送任务事件
    async fn send_event(&self, task: &Box<dyn Task>, status: &TaskStatus) {
        let event = TaskEvent {
            task_id: task.id().to_string(),
            task_type: task.task_type(),
            status: status.clone(),
            path: task.path().map(|s| s.to_string()),
            timestamp: Self::current_timestamp(),
        };

        // 发送事件，忽略发送错误
        let _ = self.inner.event_sender.send(event).await;
    }

    /// 获取当前任务统计信息
    pub async fn get_stats(&self) -> TaskStats {
        self.inner.stats.lock().await.clone()
    }

    /// 获取当前时间戳（秒）
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("获取时间失败")
            .as_secs()
    }
}

use std::fmt;
