//! 任务队列管理器，负责接收、调度和执行任务
//! 支持控制并发数量和动态添加任务

use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use super::task::{Task, TaskContext, TaskResult};
use super::tracker::TaskTracker;
use tracing::info;

/// 任务队列句柄，用于向队列提交新任务
#[derive(Clone)]
pub struct TaskQueueHandle {
    sender: mpsc::Sender<Box<dyn Task>>,
}

impl TaskQueueHandle {
    /// 向任务队列提交新任务
    pub async fn submit_task(&self, task: Box<dyn Task>) {
        // 忽略发送错误（通常是队列已关闭）
        let _ = self.sender.send(task).await;
    }
}

/// 任务队列管理器
pub struct TaskQueue {
    receiver: mpsc::Receiver<Box<dyn Task>>,
    tracker: TaskTracker,
    max_concurrent_tasks: usize,
    sender: mpsc::Sender<Box<dyn Task>>,
}

impl TaskQueue {
    /// 创建新的任务队列
    pub fn new(max_concurrent_tasks: usize, tracker: TaskTracker) -> (Self, TaskQueueHandle) {
        // 创建通道，缓冲区大小为1000
        let (sender, receiver) = mpsc::channel(1000);

        let queue = Self {
            receiver,
            tracker,
            max_concurrent_tasks,
            sender: sender.clone(),
        };

        info!("任务队列已创建");
        (queue, TaskQueueHandle { sender })
    }

    /// 启动任务队列，开始处理任务
    pub async fn run(&mut self) {
        // 创建信号量控制并发任务数量
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent_tasks));

        info!("任务队列已启动，最大并发数: {}", self.max_concurrent_tasks);


        // 持续接收并处理任务
        while let Some(mut task) = self.receiver.recv().await {
            let task_id = task.id().to_string();
            let _task_type = task.task_type();
            let tracker = self.tracker.clone();

            info!("开始处理任务: {:?}", task_id);

            // 创建任务上下文
            let queue_handle = self.create_handle();
            let context = TaskContext::new(queue_handle, tracker.clone());
            info!("任务上下文已创建");

            // 获取信号量许可，控制并发
            let permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();
            info!("获取信号量许可");

            // 添加任务到跟踪器
            tracker.add_task(task.clone()).await;
            info!("任务已添加到跟踪器");

            // 通知任务状态为等待中
            tracker.update_status(&task_id, super::tracker::TaskStatus::Pending).await;
            info!("任务状态已更新为等待中");

            // 启动任务执行
            tokio::spawn(async move {
                // 信号量许可会在任务完成后自动释放
                let _permit = permit;

                // 更新任务状态为进行中
                tracker.update_status(&task_id, super::tracker::TaskStatus::InProgress).await;

                info!("任务开始执行");
                // 执行任务
                let result = task.execute(&context).await;

                // 根据结果更新任务状态
                match result {
                    Ok(task_result) => {
                        // 根据任务结果更新状态
                        match task_result {
                            TaskResult::Success(_) | TaskResult::Continue(_) => {
                                tracker.update_status(&task_id, super::tracker::TaskStatus::Completed).await;
                            }
                            TaskResult::Failure(err) => {
                                tracker.update_status(&task_id, super::tracker::TaskStatus::Failed(err)).await;
                            }
                        }
                    }
                    Err(e) => {
                        // 处理任务执行被中断的情况（如 panic 或被取消）
                        tracker.update_status(
                            &task_id,
                            super::tracker::TaskStatus::Failed(format!("任务执行中断: {}", e)),
                        ).await;
                    }
                }
            });
            info!("任务执行已启动");
        }

        println!("任务队列已停止");
    }

    /// 创建任务队列句柄
    fn create_handle(&self) -> TaskQueueHandle {
        TaskQueueHandle { sender: self.sender.clone() }
    }
}
