use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::core::task_queue::TaskStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    DirectoryScan,
    MetadataExtraction,
    ExtensionCheck,
    SqlGeneration,
    SqlExecution
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskType::DirectoryScan => {write!(f, "DirectoryScan")}
            TaskType::MetadataExtraction => {write!(f, "MetadataExtraction")}
            TaskType::ExtensionCheck => {write!(f, "ExtensionCheck")}
            TaskType::SqlGeneration => {write!(f, "SqlGeneration")}
            TaskType::SqlExecution => {write!(f, "SqlExecution")}
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskData {
    None,
    String(String),
    Path(String),
    PathList(Vec<String>),
    FileMetadata {
        title: Option<String>,
        album: Option<String>,
        artist: Option<Vec<String>>,
        album_artist: Option<String>,
        composer: Option<Vec<String>>,
        lyricist: Option<Vec<String>>,
        genre: Option<Vec<String>>,
        release_date: Option<DateTime<Utc>>,
        track_number: Option<u16>,
        disc_number: Option<u16>,
        disc_total: Option<u16>,
        bpm: Option<u16>,
        duration: u32,
        cover_art: Option<Vec<String>>,
        audio_format: Option<String>,
        audio_size: u64,
        bitrate: Option<u32>,
        sample_rate: Option<u32>,
        file_path: String,
        create_time: Option<DateTime<Utc>>,
        update_time: Option<DateTime<Utc>>,
        copyright: Option<String>,
        remark: Option<String>,
        path_type: u8,
        is_love: u8,
        lyrics: Option<String>,
        hash: String
    },
    SqlQuery(String),
}

#[derive(Debug, Clone)]
pub enum TaskResult {
    Success(TaskData),
    Failure(String),
    Continue(TaskData)
}

#[derive(Clone)]
pub struct TaskContext {
    queue_handle: super::queue::TaskQueueHandle,
    tracker: super::tracker::TaskTracker,
}

impl TaskContext {
    pub fn new(queue_handle: super::queue::TaskQueueHandle, tracker: super::tracker::TaskTracker) -> Self {
        Self {
            queue_handle,
            tracker,
        }
    }

    pub async fn submit_task(&self, task: Box<dyn Task>) {
        self.queue_handle.submit_task(task).await;
    }

    pub fn tracker(&self) -> &super::tracker::TaskTracker {
        &self.tracker
    }
}

pub trait Task: Send + Sync + 'static {
    fn id(&self) -> &str;

    fn task_type(&self) -> TaskType;

    fn path(&self) -> Option<&str>;

    fn execute(&mut self, context: &TaskContext) -> tokio::task::JoinHandle<TaskResult>;

    fn status(&self) -> TaskStatus;

    fn set_status(&mut self, status: TaskStatus);
    fn clone_box(&self) -> Box<dyn Task>;
}

impl Clone for Box<dyn Task> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug)]
pub struct BaseTask {
    id: String,
    task_type: TaskType,
    path: Option<String>,
    status: TaskStatus,
}

impl BaseTask {
    pub fn new(task_type: TaskType, path: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            task_type,
            path,
            status: TaskStatus::Pending,
        }
    }

    fn clone_box(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}

impl Clone for BaseTask {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            task_type: self.task_type.clone(),
            path: self.path.clone(),
            status: self.status.clone(),
        }
    }
}

impl Task for BaseTask {
    fn id(&self) -> &str {
        &self.id
    }

    fn task_type(&self) -> TaskType {
        self.task_type.clone()
    }

    fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    fn execute(&mut self, _context: &TaskContext) -> tokio::task::JoinHandle<TaskResult> {
        self.set_status(TaskStatus::InProgress);
        tokio::spawn(async {
            TaskResult::Success(TaskData::None)
        })
    }

    fn status(&self) -> TaskStatus {
        self.status.clone()
    }

    fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    fn clone_box(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}