use anyhow::Result;
use std::io::{self, Write};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        match self.run_interactive_mode() {
            Ok(_) => {}
            Err(e) => {
                Self::print_error(&format!("程序运行失败: {}", e));
            }
        }
    }

    pub fn run_interactive_mode(&self) -> Result<()> {
        loop {
            Self::show_main_menu();

            let choice = match Self::get_user_input("请选择 (输入数字): ") {
                Ok(input) => input,
                Err(e) => {
                    Self::print_error(&format!("获取用户输入失败: {}", e));
                    continue;
                }
            };

            match choice.as_str() {
                "1" => {
                    self.run_batch_rename()?;
                }
                "2" => {
                    Self::print_info("功能开发中...");
                }
                "0" => {
                    Self::print_success("感谢使用HEKIT！");
                    break;
                }
                _ => {
                    Self::print_error("无效的选择，请重新输入");
                }
            }
        }
        Ok(())
    }

    fn run_batch_rename(&self) -> Result<()> {
        crate::features::rename::interface::run_interactive()
    }

    fn show_main_menu() {
        println!("=== HEKIT ===");
        println!("1. 批量重命名工具");
        println!("2. ...");
        println!("0. 退出");
        println!("======================");
    }

    fn get_user_input(prompt: &str) -> Result<String, anyhow::Error> {
        print!("{}", prompt);
        match io::stdout().flush() {
            Ok(_) => {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => Ok(input.trim().to_string()),
                    Err(e) => Err(anyhow::anyhow!("读取输入失败: {}", e)),
                }
            }
            Err(e) => Err(anyhow::anyhow!("刷新输出缓冲区失败: {}", e)),
        }
    }

    fn print_error(msg: &str) {
        eprintln!("错误: {}", msg);
    }

    fn print_success(msg: &str) {
        println!("成功: {}", msg);
    }

    fn print_info(msg: &str) {
        println!("信息: {}", msg);
    }
}
