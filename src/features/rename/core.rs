use crate::error::{HekitError, HekitResult};
use crate::features::rename::config::BatchRenameConfig;
use crate::hekit_error; // 添加宏导入
use crate::progress::ProgressManager;
use crate::utils;
use glob::glob;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

/// 批量重命名核心逻辑
pub struct BatchRenameCore {
    pub config: BatchRenameConfig,
}

impl BatchRenameCore {
    /// 创建新的批量重命名实例
    pub fn new(config: BatchRenameConfig) -> Self {
        Self { config }
    }

    /// 执行批量重命名
    pub fn execute(&self) -> HekitResult<()> {
        let files = self.scan_files()?;
        let file_pairs: Vec<(PathBuf, PathBuf)> = files
            .iter()
            .enumerate()
            .map(|(i, file_path)| {
                let new_path = self.generate_new_filename(file_path, i + 1)?;
                Ok((file_path.clone(), new_path))
            })
            .collect::<HekitResult<Vec<_>>>()?;

        if self.config.preview {
            return self.execute_preview(&file_pairs);
        }

        // 删除备份选项，直接使用批量重命名
        self.execute_batch(&file_pairs)
    }

    /// 扫描匹配的文件
    fn scan_files(&self) -> HekitResult<Vec<PathBuf>> {
        let pattern = self.config.path.join(&self.config.match_pattern);
        let pattern = pattern.to_string_lossy().to_string();

        let mut files = Vec::new();

        match glob(&pattern) {
            Ok(paths) => {
                for entry in paths.flatten() {
                    if entry.is_file() {
                        files.push(entry);
                    }
                }
                files.sort();
                Ok(files)
            }
            Err(e) => hekit_error!(FileOperation, &format!("文件扫描失败: {}", e)),
        }
    }

    // 删除 generate_new_filenames 方法（第64-72行）

    /// 为单个文件生成新文件名
    fn generate_new_filename(&self, file_path: &Path, index: usize) -> HekitResult<PathBuf> {
        let parent_dir = file_path.parent().unwrap_or(Path::new("."));
        let file_stem = file_path
            .file_stem()
            .ok_or_else(|| HekitError::Rename("无法获取文件名".to_string()))?
            .to_string_lossy();
        let extension = file_path.extension().unwrap_or_default().to_string_lossy();

        let mut new_name = file_stem.to_string();

        if let Some(prefix) = &self.config.prefix {
            new_name = format!("{}{}", prefix, new_name);
        }

        if let Some(suffix) = &self.config.suffix {
            new_name = format!("{}{}", new_name, suffix);
        }

        // 修复：改进替换功能逻辑
        if let Some(replace_pattern) = &self.config.replace_pattern {
            if replace_pattern.starts_with('/') && replace_pattern.contains('/') {
                // 正则替换模式：/pattern/replacement/
                let parts: Vec<&str> = replace_pattern.splitn(3, '/').collect();
                if parts.len() == 3 && parts[0].is_empty() {
                    if let Ok(regex) = Regex::new(parts[1]) {
                        new_name = regex.replace_all(&new_name, parts[2]).to_string();
                    }
                }
            } else if replace_pattern.contains('=') {
                // 简单替换模式：old=new
                let parts: Vec<&str> = replace_pattern.splitn(2, '=').collect();
                if parts.len() == 2 {
                    new_name = new_name.replace(parts[0], parts[1]);
                }
            } else {
                // 向后兼容：只删除匹配内容
                new_name = new_name.replace(replace_pattern, "");
            }
        }

        // 修复：序号生成逻辑
        if self.config.number_start.is_some() {
            let start = self.config.number_start.unwrap_or(1);
            let current_number = start + index - 1;
            let number_str = format!("{:03}", current_number);
            new_name = format!("{}_{}", new_name, number_str);
        }

        let new_path = if let Some(new_ext) = &self.config.extension {
            if new_ext.is_empty() {
                parent_dir.join(new_name)
            } else {
                parent_dir.join(format!("{}.{}", new_name, new_ext))
            }
        } else if extension.is_empty() {
            parent_dir.join(new_name)
        } else {
            parent_dir.join(format!("{}.{}", new_name, extension))
        };

        Ok(new_path)
    }

    /// 执行预览模式
    fn execute_preview(&self, file_pairs: &[(PathBuf, PathBuf)]) -> HekitResult<()> {
        utils::print_info("预览结果:");

        for (old_path, new_path) in file_pairs {
            println!("  {} → {}", old_path.display(), new_path.display());
        }
        println!("总计: {} 个文件", file_pairs.len());
        Ok(())
    }

    // 删除 execute_with_backup 方法（从第147行到第180行）

    /// 执行批量重命名
    fn execute_batch(&self, file_pairs: &[(PathBuf, PathBuf)]) -> HekitResult<()> {
        let progress = ProgressManager::new(file_pairs.len() as u64, "批量重命名中...");
        let mut success_count = 0;
        let mut error_count = 0;

        for (_i, (old_path, new_path)) in file_pairs.iter().enumerate() {
            progress.set_message(&format!("重命名: {}", old_path.display()));

            match self.rename_file(old_path, new_path) {
                Ok(_) => {
                    println!("✓ {} → {}", old_path.display(), new_path.display());
                    success_count += 1;
                }
                Err(e) => {
                    eprintln!("✗ {} 失败: {}", old_path.display(), e);
                    error_count += 1;
                }
            }

            progress.inc(1);
        }

        progress.finish_with_message(&format!(
            "完成: 成功 {} 个, 失败 {} 个",
            success_count, error_count
        ));

        if error_count > 0 {
            hekit_error!(Rename, "部分文件重命名失败")
        } else {
            Ok(())
        }
    }

    /// 执行单个文件重命名
    fn rename_file(&self, old_path: &Path, new_path: &Path) -> HekitResult<()> {
        if old_path == new_path {
            return hekit_error!(Rename, "源文件和目标文件路径相同");
        }

        if new_path.exists() {
            if self.config.preview {
                return hekit_error!(Rename, &format!("目标文件已存在: {}", new_path.display()));
            }

            let mut counter = 1;
            let mut new_path_with_counter = new_path.to_path_buf();

            while new_path_with_counter.exists() {
                let stem = new_path.file_stem().unwrap_or_default();
                let ext = new_path.extension().unwrap_or_default();
                new_path_with_counter = new_path.with_file_name(format!(
                    "{}_{}.{}",
                    stem.to_string_lossy(),
                    counter,
                    ext.to_string_lossy()
                ));
                counter += 1;
            }

            fs::rename(old_path, &new_path_with_counter)
                .map_err(|e| HekitError::Rename(format!("文件重命名失败: {}", e)))?;

            println!("  自动重命名为: {}", new_path_with_counter.display());
        } else {
            fs::rename(old_path, new_path)
                .map_err(|e| HekitError::Rename(format!("文件重命名失败: {}", e)))?;
        }

        Ok(())
    }
}
