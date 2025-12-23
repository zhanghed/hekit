use super::config::BatchRenameConfig;
use super::generator::FilenameGenerator;
use super::scanner::scan_files;
use std::fs;
use std::io::{self, Write}; // 添加Write trait导入

/// 批量重命名执行器
pub struct BatchRenameExecutor;

impl BatchRenameExecutor {
    /// 执行批量重命名操作
    pub fn execute(&self, config: &BatchRenameConfig) -> Result<(), anyhow::Error> {
        // 扫描文件
        let dir_path = config.directory.to_string_lossy().to_string();
        let files = scan_files(&dir_path, config.recursive, None);

        if files.is_empty() {
            return Err(anyhow::anyhow!("没有找到匹配的文件"));
        }

        // 转换为字符串
        let file_strings: Vec<String> = files
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect();

        // 生成新文件名
        let new_names: Vec<String> = file_strings
            .iter()
            .enumerate()
            .map(|(i, file)| FilenameGenerator::generate_new_filename(file, &config.target, i + 1))
            .collect();

        // 预览模式
        if config.dry_run {
            self.show_preview(&file_strings, &new_names);
            return Ok(());
        }

        // 交互模式
        if config.interactive {
            return self.execute_interactive(&file_strings, &new_names);
        }

        // 批量执行
        self.execute_batch(&file_strings, &new_names)?;
        println!("成功重命名 {} 个文件", file_strings.len());
        Ok(())
    }

    /// 显示预览
    fn show_preview(&self, files: &[String], new_names: &[String]) {
        println!("预览重命名结果:");
        for (i, (old, new)) in files.iter().zip(new_names.iter()).enumerate() {
            println!("{}. {} -> {}", i + 1, old, new);
        }
        println!("总计: {} 个文件", files.len());
    }

    /// 交互模式执行
    fn execute_interactive(
        &self,
        files: &[String],
        new_names: &[String],
    ) -> Result<(), anyhow::Error> {
        let mut renamed_count = 0;

        for (i, (old, new)) in files.iter().zip(new_names.iter()).enumerate() {
            println!("{}. {} -> {}", i + 1, old, new);

            loop {
                let input = self
                    .get_user_input("确认重命名？(y/n/skip): ")
                    .to_lowercase();
                match input.as_str() {
                    "y" | "yes" => {
                        if let Err(e) = fs::rename(old, new) {
                            eprintln!("重命名失败: {}", e);
                        } else {
                            renamed_count += 1;
                        }
                        break;
                    }
                    "n" | "no" | "s" | "skip" => break,
                    _ => println!("请输入 y(是)/n(否)/s(跳过)"),
                }
            }
        }

        println!("成功重命名 {} 个文件", renamed_count);
        Ok(())
    }

    /// 批量执行
    fn execute_batch(&self, files: &[String], new_names: &[String]) -> Result<(), anyhow::Error> {
        for (old, new) in files.iter().zip(new_names.iter()) {
            fs::rename(old, new)?;
        }
        Ok(())
    }

    /// 获取用户输入
    fn get_user_input(&self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // 现在可以正常调用flush

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
