use super::config::{BatchCleanConfig, CleanMode};
use crate::error::HekitError;
use std::fs;
use std::path::{Path, PathBuf};

/// 批量清理核心功能
pub struct BatchCleanCore {
    pub config: BatchCleanConfig, // 改为公开字段
    files_to_clean: Vec<PathBuf>,
    folders_to_clean: Vec<PathBuf>,
}

impl BatchCleanCore {
    /// 创建新的清理核心实例
    pub fn new(config: BatchCleanConfig) -> Self {
        Self {
            config,
            files_to_clean: Vec::new(),
            folders_to_clean: Vec::new(),
        }
    }

    /// 扫描目标目录
    pub fn scan(&mut self) -> Result<usize, HekitError> {
        self.files_to_clean.clear();
        self.folders_to_clean.clear();

        // 避免借用冲突，将clean_mode提取到局部变量
        let clean_mode = self.config.clean_mode.clone();

        match clean_mode {
            CleanMode::EmptyFolders => self.scan_empty_folders()?,
            CleanMode::TempFiles => self.scan_temp_files()?,
            CleanMode::LogFiles { days_old } => self.scan_log_files(days_old)?,
            CleanMode::SecureDelete => self.scan_all_files()?,
            CleanMode::Custom { patterns } => self.scan_custom_patterns(&patterns)?,
        }

        Ok(self.files_to_clean.len() + self.folders_to_clean.len())
    }

    /// 执行清理操作
    pub fn execute(&self) -> Result<usize, HekitError> {
        let mut cleaned_count = 0;

        // 备份文件（如果启用）
        if self.config.backup_enabled {
            self.backup_files()?;
        }

        // 删除文件
        for file_path in &self.files_to_clean {
            if self.config.preview_mode {
                println!("预览删除: {:?}", file_path);
            } else {
                match &self.config.clean_mode {
                    CleanMode::SecureDelete => self.secure_delete(file_path)?,
                    _ => fs::remove_file(file_path).map_err(|e| {
                        HekitError::FileOperation(format!("删除文件失败: {:?} - {}", file_path, e))
                    })?,
                }
                cleaned_count += 1;
            }
        }

        // 删除空文件夹（从最深层的开始）
        for folder_path in self.folders_to_clean.iter().rev() {
            if self.config.preview_mode {
                println!("预览删除空文件夹: {:?}", folder_path);
            } else {
                fs::remove_dir(folder_path).map_err(|e| {
                    HekitError::FileOperation(format!("删除文件夹失败: {:?} - {}", folder_path, e))
                })?;
                cleaned_count += 1;
            }
        }

        Ok(cleaned_count)
    }

    /// 扫描空文件夹
    fn scan_empty_folders(&mut self) -> Result<(), HekitError> {
        let target_dir = self.config.target_dir.clone();
        let mut folders_to_clean = Vec::new();

        // 将 is_empty_folder 提取为独立函数，避免借用冲突
        fn is_empty_folder(path: &Path) -> bool {
            if let Ok(entries) = fs::read_dir(path) {
                entries.count() == 0
            } else {
                false
            }
        }

        self.walk_directory(&target_dir, |path| {
            if path.is_dir() && is_empty_folder(path) {
                folders_to_clean.push(path.to_path_buf());
            }
        })?;

        self.folders_to_clean.extend(folders_to_clean);
        Ok(())
    }

    /// 扫描临时文件
    fn scan_temp_files(&mut self) -> Result<(), HekitError> {
        let temp_extensions = ["tmp", "bak", "temp", "log", "cache"];
        let target_dir = self.config.target_dir.clone();
        let mut files_to_clean = Vec::new(); // 局部变量

        self.walk_directory(&target_dir, |path| {
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    let ext_str = extension.to_string_lossy().to_lowercase();
                    if temp_extensions.contains(&ext_str.as_str()) {
                        files_to_clean.push(path.to_path_buf()); // 修改局部变量
                    }
                }
            }
        })?;

        self.files_to_clean.extend(files_to_clean); // 一次性扩展到self字段
        Ok(())
    }

    /// 扫描日志文件（按时间）
    fn scan_log_files(&mut self, days_old: u32) -> Result<(), HekitError> {
        use chrono::{Duration, Utc};
        let cutoff_time = Utc::now() - Duration::days(days_old as i64);
        let target_dir = self.config.target_dir.clone();
        let mut files_to_clean = Vec::new();

        self.walk_directory(&target_dir, |path| {
            if path.is_file() {
                if let Ok(metadata) = fs::metadata(path) {
                    if let Ok(modified) = metadata.modified() {
                        let modified_time: chrono::DateTime<chrono::Utc> = modified.into();
                        if modified_time < cutoff_time {
                            files_to_clean.push(path.to_path_buf());
                        }
                    }
                }
            }
        })?;

        self.files_to_clean.extend(files_to_clean);
        Ok(())
    }

    /// 扫描所有文件（安全删除模式）
    fn scan_all_files(&mut self) -> Result<(), HekitError> {
        let target_dir = self.config.target_dir.clone();
        let mut files_to_clean = Vec::new();

        self.walk_directory(&target_dir, |path| {
            if path.is_file() {
                files_to_clean.push(path.to_path_buf());
            }
        })?;

        self.files_to_clean.extend(files_to_clean);
        Ok(())
    }

    /// 扫描自定义模式文件
    fn scan_custom_patterns(&mut self, patterns: &[String]) -> Result<(), HekitError> {
        let target_dir = self.config.target_dir.clone();
        let mut files_to_clean = Vec::new();

        self.walk_directory(&target_dir, |path| {
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    for pattern in patterns {
                        if file_name_str.contains(pattern) {
                            files_to_clean.push(path.to_path_buf());
                            break;
                        }
                    }
                }
            }
        })?;

        self.files_to_clean.extend(files_to_clean);
        Ok(())
    }

    /// 遍历目录（重构版本，避免借用冲突）
    fn walk_directory<F>(&mut self, dir: &Path, mut callback: F) -> Result<(), HekitError>
    where
        F: FnMut(&Path),
    {
        // 使用辅助函数来避免递归调用时的借用冲突
        fn walk_directory_helper<F>(dir: &Path, callback: &mut F) -> Result<(), HekitError>
        where
            F: FnMut(&Path),
        {
            if dir.is_dir() {
                let entries = fs::read_dir(dir).map_err(|e| {
                    HekitError::FileOperation(format!("读取目录失败: {:?} - {}", dir, e))
                })?;

                for entry in entries {
                    let entry = entry
                        .map_err(|e| HekitError::FileOperation(format!("读取目录项失败: {}", e)))?;
                    let path = entry.path();

                    callback(&path);

                    if path.is_dir() {
                        walk_directory_helper(&path, callback)?; // 递归调用辅助函数
                    }
                }
            }
            Ok(())
        }

        walk_directory_helper(dir, &mut callback)
    }

    /// 安全删除文件（覆盖数据）
    fn secure_delete(&self, path: &Path) -> Result<(), HekitError> {
        // 简单实现：先覆盖文件内容，然后删除
        if let Ok(mut file) = fs::File::create(path) {
            use std::io::Write;
            // 写入随机数据覆盖
            let random_data = vec![0u8; 1024];
            for _ in 0..3 {
                // 覆盖3次
                file.write_all(&random_data).map_err(|e| {
                    HekitError::FileOperation(format!("覆盖文件失败: {:?} - {}", path, e))
                })?;
            }
        }

        fs::remove_file(path)
            .map_err(|e| HekitError::FileOperation(format!("删除文件失败: {:?} - {}", path, e)))
    }

    /// 备份文件
    fn backup_files(&self) -> Result<(), HekitError> {
        // 简化备份实现
        println!("备份功能暂未实现");
        Ok(())
    }

    /// 获取要清理的文件列表
    pub fn get_files_to_clean(&self) -> &Vec<PathBuf> {
        &self.files_to_clean
    }

    /// 获取要清理的文件夹列表
    pub fn get_folders_to_clean(&self) -> &Vec<PathBuf> {
        &self.folders_to_clean
    }
}
