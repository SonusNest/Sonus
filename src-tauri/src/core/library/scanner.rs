/*
* Scanner
* This module is responsible for scanning the device, WebDAV and NAS for new tracks.
*/
use super::super::task_queue::task::{Task, TaskType, TaskResult, TaskData, TaskContext, BaseTask};
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{info, debug};
use async_recursion::async_recursion;
use crate::core::task_queue::TaskStatus;
use base64::engine::{general_purpose, Engine as _};

#[derive(Debug)]
pub struct DirectoryScanTask {
    base: BaseTask,
}

impl DirectoryScanTask {
    /// 创建新的目录扫描任务
    pub fn new(path: String) -> Self {
        Self {
            base: BaseTask::new(TaskType::DirectoryScan, Some(path)),
        }
    }

    /// 递归扫描目录
    #[async_recursion]
    async fn scan_directory(path: &str) -> Vec<String> {
        info!("准备扫描目录: {:?}", path);
        let mut result = Vec::new();
        let path_buf = PathBuf::from(path);

        if !path_buf.exists() {
            info!("Path not exists: {}", path);
            return result;
        }
        info!("Path exists: {}", path);
        info!("准备读取目录内容");
        // 读取目录内容（返回ReadDir，它实现了Stream）
        match fs::read_dir(&path_buf).await {
            Ok(mut entries) => {
                info!("目录内容读取成功");
                // 异步遍历：使用StreamExt的next()方法
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let entry_path = entry.path();
                    info!("准备获取元数据: {:?}", entry_path);
                    let metadata = match entry.metadata().await {
                        Ok(m) => {
                            info!("元数据获取成功: {:?}", m);
                            m
                        },
                        Err(e) => {
                            eprintln!("获取元数据失败: {:?}, 错误: {}", entry_path, e);
                            continue;
                        }
                    };

                    let path_str = entry_path.to_string_lossy().to_string();
                    if metadata.is_dir() {
                        // 递归扫描子目录
                        info!("准备递归扫描子目录: {:?}", path_str);
                        let sub_files = DirectoryScanTask::scan_directory(&path_str).await;
                        info!("子目录扫描完成，共发现 {} 个文件", sub_files.len());
                        result.extend(sub_files);
                    } else if metadata.is_file() {
                        info!("发现文件: {:?}", path_str);
                        result.push(path_str);
                    }
                }
            }
            Err(e) => eprintln!("读取目录失败: {}, 错误: {}", path, e),
        }

        result
    }
}

impl Task for DirectoryScanTask {
    fn id(&self) -> &str {
        self.base.id()
    }

    fn task_type(&self) -> TaskType {
        self.base.task_type()
    }

    fn path(&self) -> Option<&str> {
        self.base.path()
    }

    fn execute(&mut self, context: &TaskContext) -> tokio::task::JoinHandle<TaskResult> {
        self.set_status(TaskStatus::InProgress);
        info!("目录扫描任务开始执行");
        let path = self.base.path().unwrap_or("").to_string();
        let context = context.clone();
        // let self_clone = self.clone();
        info!("目录扫描任务路径: {:?}", path);
        tokio::spawn(async move {
            // 执行目录扫描
            let files = DirectoryScanTask::scan_directory(&path).await;
            info!("目录扫描任务完成，共发现 {} 个文件", files.len());
            // 克隆一份用于循环，避免所有权转移
            let files_clone = files.clone();

            // 使用克隆体进行循环（转移克隆体的所有权）
            for file_path in files {
                info!("准备提交扩展名检查任务: {:?}", file_path);
                let ext_check_task = Box::new(ExtensionCheckTask::new(file_path));

                info!("扩展名检查任务创建成功");
                context.submit_task(ext_check_task).await;
                info!("扩展名检查任务提交成功");
            }

            // 原始files未被移动，可正常使用
            TaskResult::Success(TaskData::PathList(files_clone))
        })
    }

    fn status(&self) -> TaskStatus {
        self.base.status()
    }

    fn set_status(&mut self, status: TaskStatus) {
        self.base.set_status(status);
    }

    fn clone_box(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}

impl Clone for DirectoryScanTask {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone()
        }
    }
}

/// 扩展名检查任务：根据扩展名初步判断文件类型
#[derive(Debug)]
pub struct ExtensionCheckTask {
    base: BaseTask,
}

impl ExtensionCheckTask {
    /// 创建新的扩展名检查任务
    pub fn new(path: String) -> Self {
        Self {
            base: BaseTask::new(TaskType::ExtensionCheck, Some(path)),
        }
    }

    /// 检查文件扩展名是否为支持的类型
    fn is_supported_extension(path: &str) -> bool {
        info!("准备检查文件扩展名: {:?}", path);
        let supported_extensions = ["mp3", "flac", "wav", "aac", "m4a", "ogg"];
        let ext = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        info!("文件扩展名: {:?}", ext);
        supported_extensions.contains(&ext.as_str())
    }
}

impl Task for ExtensionCheckTask {
    fn id(&self) -> &str {
        self.base.id()
    }

    fn task_type(&self) -> TaskType {
        self.base.task_type()
    }

    fn path(&self) -> Option<&str> {
        self.base.path()
    }

    fn execute(&mut self, context: &TaskContext) -> tokio::task::JoinHandle<TaskResult> {
        self.set_status(TaskStatus::InProgress);
        let path = self.base.path().unwrap_or("").to_string();
        let context = context.clone();

        tokio::spawn(async move {
            // 检查文件扩展名
            let is_supported = Self::is_supported_extension(&path);
            info!("扩展名检查结果: {:?}", is_supported);
            if is_supported {
                // 如果扩展名有效，创建文件头检查任务
                let header_check_task = Box::new(MetadataExtractionTask::new(path.clone()));
                context.submit_task(header_check_task).await;

                TaskResult::Continue(TaskData::Path(path))
            } else {
                TaskResult::Success(TaskData::String(format!("不支持的文件类型: {}", path)))
            }
        })
    }

    fn status(&self) -> TaskStatus {
        self.base.status()
    }

    fn set_status(&mut self, status: TaskStatus) {
        self.base.set_status(status);
    }

    fn clone_box(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}

impl Clone for ExtensionCheckTask {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone()
        }
    }
}

/// 元数据提取任务：获取音频文件的元数据
#[derive(Debug)]
pub struct MetadataExtractionTask {
    base: BaseTask,
}

impl MetadataExtractionTask {
    /// 创建新的元数据提取任务
    pub fn new(path: String) -> Self {
        Self {
            base: BaseTask::new(TaskType::MetadataExtraction, Some(path)),
        }
    }

    /// 提取音频文件的元数据
    async fn extract_metadata(path: &str) -> TaskResult {
        use lofty::prelude::*;
        use lofty::probe::Probe;
        use md5;
        use std::path::Path;

        let path_str = Path::new(&path);

        assert!(path_str.is_file(), "ERROR: Path is not a file!");

        let tagged_file = Probe::open(path)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => {
                let err_msg = format!("文件没有任何标签: {}", path);
                info!("{}", err_msg);
                return TaskResult::Failure(err_msg);
            },
        };

        let title = tag.title().map(|s| s.to_string());
        let album = tag.album().map(|s| s.to_string());
        let artist = {
            // 优先获取 TrackArtists 列表
            let v: Vec<String> = tag.get_strings(&ItemKey::TrackArtists)
                .map(|s| s.to_string())
                .collect();
            if !v.is_empty() {
                Some(v)
            } else {
                // 其次获取 TrackArtists 列表（这里可能和v是同一来源，根据实际情况确认）
                let v1: Vec<String> = tag.get_strings(&ItemKey::TrackArtists)
                    .map(|s| s.to_string())
                    .collect();
                if !v1.is_empty() {
                    Some(v1)
                } else {
                    // 最后获取 TrackArtist 单个值
                    let v2: String = tag.get_string(&ItemKey::TrackArtist)
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                    if !v2.is_empty() {
                        Some(vec![v2])  // 统一返回Vec<String>类型
                    } else {
                        None
                    }
                }
            }
        };
        let album_artist = {
            let v: String = tag.get_string(&ItemKey::AlbumArtist).map(|s| s.to_string()).unwrap_or_default();
            if v.is_empty() { None } else { Some(v) }
        };
        let composer = {
            let v: Vec<String> = tag.get_strings(&ItemKey::Composer).map(|s| s.to_string()).collect();
            if v.is_empty() { None } else { Some(v) }
        };
        let lyricist = {
            let v: Vec<String> = tag.get_strings(&ItemKey::Lyricist).map(|s| s.to_string()).collect();
            if v.is_empty() { None } else { Some(v) }
        };
        let genre = {
            let v: Vec<String> = tag.get_strings(&ItemKey::Genre).map(|s| s.to_string()).collect();
            if v.is_empty() { None } else { Some(v) }
        };
        let release_date = tag.get_string(&ItemKey::ReleaseDate).map(|s| s.to_string());
        let track_number = tag.get_string(&ItemKey::TrackNumber).map(|s| s.to_string().parse::<u16>().unwrap());
        let disc_number = tag.get_string(&ItemKey::DiscNumber).map(|s| s.to_string().parse().unwrap());
        let disc_total = tag.get_string(&ItemKey::DiscTotal).map(|s| s.to_string().parse().unwrap());
        let bpm = tag.get_string(&ItemKey::Bpm).map(|s| s.to_string().parse::<u16>().unwrap());
        let duration = tagged_file.properties().duration().as_secs() as u32;
        let cover_art = {
            let pictures = tag.pictures();
            if pictures.is_empty() {
                None
            } else {
                let mut s = String::from("data:image/png;base64,");
                s.push_str(&bytes_to_base64(pictures[0].data()));
                let test = String::from("test");
                Some(vec!(test))
            }
        };
        let audio_format = Option::from(get_extension_from_filename(path.to_string()));
        let audio_size = get_file_size(path);
        let bitrate = tagged_file.properties().audio_bitrate();
        let sample_rate = tagged_file.properties().sample_rate();
        let file_path = path.to_string();
        let create_time = None;
        let update_time = None;
        let copyright = tag.get_string(&ItemKey::CopyrightMessage).map(|s| s.to_string());
        let remark = tag.get_string(&ItemKey::Comment).map(|s| s.to_string());
        let path_type = 0;
        let is_love = 0;
        let lyrics = tag.get_string(&ItemKey::Lyrics).map(|s| s.to_string());
        let hash = format!("{:x}", md5::compute(&std::fs::read(path).unwrap()));

        let metadata = TaskData::FileMetadata {
            title,
            album,
            artist,
            album_artist,
            composer,
            lyricist,
            genre,
            release_date,
            track_number,
            disc_number,
            disc_total,
            bpm,
            duration,
            cover_art,
            audio_format,
            audio_size,
            bitrate,
            sample_rate,
            file_path,
            create_time,
            update_time,
            copyright,
            remark,
            path_type,
            is_love,
            lyrics,
            hash,
        };

        info!("Metadata: {:?}", metadata);

        TaskResult::Success(metadata)
    }
}

fn bytes_to_base64(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}

fn get_file_size(filepath: &str) -> u64 {
    let metadata = std::fs::metadata(filepath);
    metadata.unwrap().len()
}

fn get_extension_from_filename(filename: String) -> String {

    //Change it to a canonical file path.
    let path = Path::new(&filename).canonicalize().expect(
        "Expecting an existing filename",
    );

    let filepath = path.to_str();
    let name = filepath.unwrap().split('/');
    let names: Vec<&str> = name.collect();
    let extension = names.last().expect("File extension can not be read.");
    let extension1: Vec<&str> = extension.split(".").collect();

    extension1[1..(extension1.len())].join(".").to_string()
}

impl Task for MetadataExtractionTask {
    fn id(&self) -> &str {
        self.base.id()
    }

    fn task_type(&self) -> TaskType {
        self.base.task_type()
    }

    fn path(&self) -> Option<&str> {
        self.base.path()
    }

    fn execute(&mut self, context: &TaskContext) -> tokio::task::JoinHandle<TaskResult> {
        self.set_status(TaskStatus::InProgress);
        let path = self.base.path().unwrap_or("").to_string();
        let context = context.clone();

        tokio::spawn(async move {
            // 提取元数据
            match Self::extract_metadata(&path).await {
                TaskResult::Success(metadata) => {
                    // 提取成功，创建SQL生成任务
                    let sql_task = Box::new(SqlGenerationTask::new(path, metadata.clone()));
                    context.submit_task(sql_task).await;
                    TaskResult::Continue(metadata)
                }
                TaskResult::Failure(err) => {
                    // 提取失败，直接返回错误
                    TaskResult::Failure(err)
                }
                TaskResult::Continue(_) => {
                    // 理论上不会走到这里，保持完整性
                    TaskResult::Failure("Unexpected Continue result".to_string())
                }
            }
        })
    }

    fn status(&self) -> TaskStatus {
        self.base.status()
    }

    fn set_status(&mut self, status: TaskStatus) {
        self.base.set_status(status);
    }

    fn clone_box(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}

impl Clone for MetadataExtractionTask {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone()
        }
    }
}

/// SQL生成任务：根据元数据生成SQL语句
#[derive(Debug)]
pub struct SqlGenerationTask {
    base: BaseTask,
    metadata: TaskData,
}

impl SqlGenerationTask {
    /// 创建新的SQL生成任务
    pub fn new(path: String, metadata: TaskData) -> Self {
        Self {
            base: BaseTask::new(TaskType::SqlGeneration, Some(path)),
            metadata,
        }
    }

    /// 根据元数据生成SQL插入语句
    fn generate_sql(metadata: &TaskData) -> Option<String> {
        if let TaskData::FileMetadata {
            title,
            album,
            artist,
            album_artist,
            composer,
            lyricist,
            genre,
            release_date,
            track_number,
            disc_number,
            disc_total,
            bpm,
            duration,
            cover_art,
            audio_format,
            audio_size,
            bitrate,
            sample_rate,
            file_path,
            create_time,
            update_time,
            copyright,
            remark,
            path_type,
            is_love,
            lyrics,
            hash
        } = metadata
        {
            Some(format!(
                "INSERT INTO music (title, album, artist, album_artist, composer, lyricist, genre, release_date, track_number, disc_number, bpm, duration, cover_art, audio_format, audio_size, bitrate, sample_rate, file_path, create_time, update_time, copyright, remark, path_type, is_love, hash, disc_total, lyrics)
                 VALUES ( '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', {}, {}, {}, {}, '{}', '{}', {}, {}, {}, '{}', {}, {}, '{}', '{}', {}, {}, '{}', {}, '{}')",
                escape_sql_string(title.as_deref().unwrap_or("unknown")),
                escape_sql_string(album.as_deref().unwrap_or("unknown")),
                escape_sql_string(&artist.as_ref().map(|a| a.join(", ")).unwrap_or_else(|| "unknown".to_string())),
                escape_sql_string(album_artist.as_deref().unwrap_or("unknown")),
                escape_sql_string(composer.as_ref().map(|a| a.join(", ")).as_deref().unwrap_or("unknown")),
                escape_sql_string(lyricist.as_ref().map(|a| a.join(", ")).as_deref().unwrap_or("none")),
                escape_sql_string(genre.as_ref().map(|a| a.join(", ")).as_deref().unwrap_or("unknown")),
                escape_sql_string(release_date.as_deref().unwrap_or("1970-01-01")),
                track_number.unwrap_or(0),
                disc_number.unwrap_or(0),
                bpm.unwrap_or(0),
                duration,
                escape_sql_string(cover_art.as_ref().map(|a| a.join(", ")).as_deref().unwrap_or("default")),
                escape_sql_string(audio_format.as_deref().unwrap_or("")),
                audio_size,
                bitrate.unwrap_or(0),
                sample_rate.unwrap_or(0),
                escape_sql_string(file_path),
                create_time.unwrap_or(0),
                update_time.unwrap_or(0),
                escape_sql_string(copyright.as_deref().unwrap_or("unknown")),
                escape_sql_string(remark.as_deref().unwrap_or("")),
                path_type,
                is_love,
                escape_sql_string(hash),
                disc_total.unwrap_or(0),
                escape_sql_string(lyrics.as_deref().unwrap_or("")),
            ))
        } else {
            None
        }
    }
}

fn escape_sql_string(input: &str) -> String {
    input.replace("'", "''")
}

impl Task for SqlGenerationTask {
    fn id(&self) -> &str {
        self.base.id()
    }

    fn task_type(&self) -> TaskType {
        self.base.task_type()
    }

    fn path(&self) -> Option<&str> {
        self.base.path()
    }

    fn execute(&mut self, context: &TaskContext) -> tokio::task::JoinHandle<TaskResult> {
        self.set_status(TaskStatus::InProgress);
        let metadata = self.metadata.clone();
        let context = context.clone();

        tokio::spawn(async move {
            // 生成SQL语句
            match Self::generate_sql(&metadata) {
                Some(sql) => {
                    // 创建SQL执行任务
                    let path = if let TaskData::FileMetadata { file_path, .. } = &metadata {
                        file_path.clone()
                    } else {
                        "未知路径".to_string()
                    };

                    let sql_task = Box::new(SqlExecutionTask::new(path, sql.clone()));
                    context.submit_task(sql_task).await;

                    TaskResult::Continue(TaskData::SqlQuery(sql))
                }
                None => {
                    TaskResult::Failure("生成SQL语句失败".to_string())
                }
            }
        })
    }

    fn status(&self) -> TaskStatus {
        self.base.status()
    }

    fn set_status(&mut self, status: TaskStatus) {
        self.base.set_status(status);
    }

    fn clone_box(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}

impl Clone for SqlGenerationTask {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

/// SQL执行任务：执行SQL插入语句
#[derive(Debug)]
pub struct SqlExecutionTask {
    base: BaseTask,
    sql: String,
}

impl SqlExecutionTask {
    /// 创建新的SQL执行任务
    pub fn new(path: String, sql: String) -> Self {
        Self {
            base: BaseTask::new(TaskType::SqlExecution, Some(path)),
            sql,
        }
    }

    /// 执行SQL语句
    async fn execute_sql(sql: &str) -> Result<usize, String> {
        use super::super::super::app::database::{connection, execute};
        let conn = connection();
        execute(&conn, sql).map_err(|e| e.to_string())
    }
}

impl Task for SqlExecutionTask {
    fn id(&self) -> &str {
        self.base.id()
    }

    fn task_type(&self) -> TaskType {
        self.base.task_type()
    }

    fn path(&self) -> Option<&str> {
        self.base.path()
    }

    fn execute(&mut self, _context: &TaskContext) -> tokio::task::JoinHandle<TaskResult> {
        self.set_status(TaskStatus::InProgress);
        let sql = self.sql.clone();
        let path = self.base.path().unwrap_or("").to_string();

        tokio::spawn(async move {
            // 执行SQL
            match Self::execute_sql(&sql).await {
                Ok(rows_affected) => {
                    TaskResult::Success(TaskData::String(format!(
                        "已成功为 {} 创建索引，影响行数: {}", path, rows_affected
                    )))
                }
                Err(e) => {
                    TaskResult::Failure(format!("执行SQL失败: {}", e))
                }
            }
        })
    }

    fn status(&self) -> TaskStatus {
        self.base.status()
    }

    fn set_status(&mut self, status: TaskStatus) {
        self.base.set_status(status);
    }

    fn clone_box(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}

impl Clone for SqlExecutionTask {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            sql: self.sql.clone(),
        }
    }
}
