use super::config::BatchRenameConfig;
use super::generator::FilenameGenerator;
use super::scanner::scan_files;
use crate::app::App;
use std::fs;

/// 批量重命名执行器 - 负责执行批量重命名操作
pub struct BatchRenameExecutor;

impl BatchRenameExecutor {
    /// 执行批量重命名操作
    ///
    /// # 参数
    /// - config: 批量重命名配置
    ///
    /// # 返回值
    /// 操作结果，成功返回 Ok(()), 失败返回 Err
    pub fn execute(&self, config: &BatchRenameConfig) -> Result<(), anyhow::Error> {
        // 扫描匹配的文件
        let dir_path = config.directory.to_string_lossy().to_string();
        let files = scan_files(&dir_path, config.recursive, None);

        // 检查是否找到文件
        if files.is_empty() {
            return Err(anyhow::anyhow!("没有找到匹配的文件"));
        }

        // 将 PathBuf 转换为字符串
        let file_strings: Vec<String> = files
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect();

        // 生成新的文件名
        let new_names: Vec<String> = file_strings
            .iter()
            .enumerate()
            .map(|(i, file)| FilenameGenerator::generate_new_filename(file, &config.target, i + 1))
            .collect();

        // 预览模式：只显示重命名结果，不实际执行
        if config.dry_run {
            self.show_preview(&file_strings, &new_names);
            return Ok(());
        }

        // 交互模式：逐个确认重命名
        if config.interactive {
            return self.execute_interactive(&file_strings, &new_names);
        }

        // 批量执行重命名
        self.execute_batch(&file_strings, &new_names)?;

        App::print_success(&format!("成功重命名 {} 个文件", file_strings.len()));
        Ok(())
    }

    /// 显示重命名预览
    ///
    /// # 参数
    /// - files: 原始文件列表
    /// - new_names: 新文件名列表
    fn show_preview(&self, files: &[String], new_names: &[String]) {
        println!("预览重命名结果:");
        for (i, (old, new)) in files.iter().zip(new_names.iter()).enumerate() {
            println!("{}. {} -> {}", i + 1, old, new);
        }
        println!("总计: {} 个文件", files.len());
    }

    /// 交互模式执行重命名
    ///
    /// # 参数
    /// - files: 原始文件列表
    /// - new_names: 新文件名列表
    ///
    /// # 返回值
    /// 操作结果
    fn execute_interactive(
        &self,
        files: &[String],
        new_names: &[String],
    ) -> Result<(), anyhow::Error> {
        let mut renamed_count = 0;

        // 逐个文件处理
        for (i, (old, new)) in files.iter().zip(new_names.iter()).enumerate() {
            println!("{}. {} -> {}", i + 1, old, new);

            // 循环等待用户确认
            loop {
                let input = App::get_user_input("确认重命名？(y/n/skip): ").to_lowercase();
                match input.as_str() {
                    "y" | "yes" => {
                        // 执行重命名
                        if let Err(e) = fs::rename(old, new) {
                            App::print_error(&format!("重命名失败: {}", e));
                        } else {
                            renamed_count += 1;
                        }
                        break;
                    }
                    "n" | "no" => break,   // 跳过当前文件
                    "s" | "skip" => break, // 跳过当前文件
                    _ => println!("请输入 y(是)/n(否)/s(跳过)"),
                }
            }
        }

        App::print_success(&format!("成功重命名 {} 个文件", renamed_count));
        Ok(())
    }

    /// 批量执行重命名
    ///
    /// # 参数
    /// - files: 原始文件列表
    /// - new_names: 新文件名列表
    ///
    /// # 返回值
    /// 操作结果
    fn execute_batch(&self, files: &[String], new_names: &[String]) -> Result<(), anyhow::Error> {
        // 逐个文件重命名
        for (old, new) in files.iter().zip(new_names.iter()) {
            if let Err(e) = fs::rename(old, new) {
                return Err(anyhow::anyhow!("重命名失败: {} -> {}: {}", old, new, e));
            }
        }
        Ok(())
    }
}
