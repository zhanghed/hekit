use anyhow::{anyhow, Result};
use glob::glob;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

/// 批量重命名核心逻辑
pub struct BatchRenameCore {
    config: crate::features::rename::config::BatchRenameConfig,
}

impl BatchRenameCore {
    /// 创建新的重命名核心实例
    pub fn new(config: crate::features::rename::config::BatchRenameConfig) -> Self {
        Self { config }
    }

    /// 执行重命名操作
    pub fn execute(&self) -> Result<()> {
        println!("开始重命名...");

        // 扫描文件
        let files = self.scan_files()?;
        if files.is_empty() {
            println!("未找到匹配的文件");
            return Ok(());
        }

        println!("找到 {} 个文件", files.len());

        // 生成新文件名
        let file_pairs = self.generate_new_filenames(&files)?;

        if self.config.preview {
            self.execute_preview(&file_pairs)
        } else if self.config.backup {
            self.execute_with_backup(&file_pairs)
        } else {
            self.execute_batch(&file_pairs)
        }
    }

    /// 扫描匹配的文件
    fn scan_files(&self) -> Result<Vec<PathBuf>> {
        let pattern = format!(
            "{}/{}",
            self.config.path.display(),
            self.config.match_pattern
        );

        let mut files = Vec::new();

        match glob(&pattern) {
            Ok(paths) => {
                for entry in paths.flatten() {
                    if entry.is_file() {
                        files.push(entry);
                    }
                }
                files.sort(); // 确保顺序一致
                Ok(files)
            }
            Err(e) => Err(anyhow!("文件扫描失败: {}", e)),
        }
    }

    /// 生成新文件名
    fn generate_new_filenames(&self, files: &[PathBuf]) -> Result<Vec<(PathBuf, PathBuf)>> {
        let mut file_pairs = Vec::new();

        for (index, file_path) in files.iter().enumerate() {
            let new_filename = self.generate_new_filename(file_path, index + 1)?;
            file_pairs.push((file_path.clone(), new_filename));
        }

        Ok(file_pairs)
    }

    /// 为单个文件生成新文件名
    fn generate_new_filename(&self, file_path: &Path, index: usize) -> Result<PathBuf> {
        let parent_dir = file_path.parent().unwrap_or(Path::new("."));
        let file_stem = file_path.file_stem().unwrap_or_default().to_string_lossy();
        let extension = file_path.extension().unwrap_or_default().to_string_lossy();

        let mut new_name = file_stem.to_string();

        // 应用前缀
        if let Some(prefix) = &self.config.prefix {
            new_name = format!("{}{}", prefix, new_name);
        }

        // 应用后缀
        if let Some(suffix) = &self.config.suffix {
            new_name = format!("{}{}", new_name, suffix);
        }

        // 执行文本替换
        if let Some(replace_pattern) = &self.config.replace_pattern {
            if replace_pattern.starts_with('/') && replace_pattern.contains('/') {
                // 正则表达式替换格式: /pattern/replacement/
                let parts: Vec<&str> = replace_pattern.splitn(3, '/').collect();
                if parts.len() == 3 && parts[0].is_empty() {
                    if let Ok(regex) = Regex::new(parts[1]) {
                        new_name = regex.replace_all(&new_name, parts[2]).to_string();
                    }
                }
            } else if let Some((old, new)) = replace_pattern.split_once('=') {
                // 简单替换格式: old=new
                new_name = new_name.replace(old, new);
            }
        }
        // 添加序号
        if let Some(start) = self.config.number_start {
            let current_number = start + index - 1;
            let number_str = format!("{:03}", current_number); // 3位补零
            new_name = format!("{}_{}", new_name, number_str);
        }

        // 构建完整路径
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
    fn execute_preview(&self, file_pairs: &[(PathBuf, PathBuf)]) -> Result<()> {
        println!("预览结果:");

        for (old_path, new_path) in file_pairs {
            println!("  {} → {}", old_path.display(), new_path.display());
        }

        println!("\n总计: {} 个文件", file_pairs.len());
        println!("使用 -v 选项预览完成");
        Ok(())
    }

    /// 执行带备份的重命名
    fn execute_with_backup(&self, file_pairs: &[(PathBuf, PathBuf)]) -> Result<()> {
        println!("开始备份重命名...");
        let mut success_count = 0;
        let mut error_count = 0;

        for (old_path, new_path) in file_pairs {
            // 先备份原文件
            let backup_path = old_path.with_extension(format!(
                "{}.bak",
                old_path.extension().unwrap_or_default().to_string_lossy()
            ));

            match fs::copy(old_path, &backup_path) {
                Ok(_) => match fs::rename(old_path, new_path) {
                    Ok(_) => {
                        println!(
                            "✓ {} → {} (备份: {})",
                            old_path.display(),
                            new_path.display(),
                            backup_path.display()
                        );
                        success_count += 1;
                    }
                    Err(e) => {
                        eprintln!("✗ {} 重命名失败: {}", old_path.display(), e);
                        error_count += 1;
                    }
                },
                Err(e) => {
                    eprintln!("✗ {} 备份失败: {}", old_path.display(), e);
                    error_count += 1;
                }
            }
        }

        println!("\n完成: 成功 {} 个, 失败 {} 个", success_count, error_count);

        if error_count > 0 {
            Err(anyhow!("部分文件重命名失败"))
        } else {
            Ok(())
        }
    }

    /// 执行批量重命名
    fn execute_batch(&self, file_pairs: &[(PathBuf, PathBuf)]) -> Result<()> {
        let mut success_count = 0;
        let mut error_count = 0;

        for (old_path, new_path) in file_pairs {
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
        }

        println!("\n完成: 成功 {} 个, 失败 {} 个", success_count, error_count);

        if error_count > 0 {
            Err(anyhow!("部分文件重命名失败"))
        } else {
            Ok(())
        }
    }

    /// 执行单个文件重命名
    fn rename_file(&self, old_path: &Path, new_path: &Path) -> Result<()> {
        if old_path == new_path {
            return Err(anyhow!("源文件和目标文件路径相同"));
        }

        // 检查目标文件是否存在，如果存在则自动处理
        if new_path.exists() {
            if self.config.preview {
                return Err(anyhow!("目标文件已存在: {}", new_path.display()));
            }

            // 自动生成不冲突的文件名
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
                .map_err(|e| anyhow!("文件重命名失败: {}", e))?;

            println!("  自动重命名为: {}", new_path_with_counter.display());
        } else {
            fs::rename(old_path, new_path).map_err(|e| anyhow!("文件重命名失败: {}", e))?;
        }

        Ok(())
    }
}
