use anyhow::Result;
use std::io::{self, Write};

/// 应用程序主结构体
pub struct App {}

impl App {
    /// 创建新的应用程序实例
    pub fn new() -> Self {
        Self {}
    }

    /// 运行应用程序
    pub fn run(&self) {
        // 运行交互式模式，处理可能的错误
        if let Err(e) = self.run_interactive_mode() {
            Self::print_error(&format!("程序运行失败: {}", e));
        }
    }

    /// 运行交互式模式
    pub fn run_interactive_mode(&self) -> Result<()> {
        loop {
            // 显示主菜单
            Self::show_main_menu();
            let choice = Self::get_user_input("请选择操作 (输入数字): ");

            match choice.as_str() {
                "1" => {
                    // 批量重命名文件功能
                    self.run_batch_rename()?;
                }
                "2" => {
                    // 文件搜索功能（预留）
                    Self::print_info("文件搜索功能开发中...");
                }
                "3" => {
                    // 文件复制功能（预留）
                    Self::print_info("文件复制功能开发中...");
                }
                "4" => {
                    // 退出程序
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

    /// 运行批量重命名功能
    fn run_batch_rename(&self) -> Result<()> {
        // 显示使用说明
        crate::features::batch_rename::show_usage();

        // 获取用户输入
        let input = Self::get_user_input("请输入批量重命名命令: ");

        // 执行命令
        crate::features::batch_rename::execute_command(&input)
    }

    // UI工具函数

    /// 显示主菜单
    fn show_main_menu() {
        println!("=== HEKIT 工具集合 ===");
        println!("1. 批量重命名文件");
        println!("2. 文件搜索");
        println!("3. 文件复制");
        println!("4. 退出");
        println!("======================");
    }

    /// 获取用户输入
    fn get_user_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    /// 打印错误信息
    fn print_error(msg: &str) {
        eprintln!("错误: {}", msg);
    }

    /// 打印成功信息
    fn print_success(msg: &str) {
        println!("成功: {}", msg);
    }

    /// 打印信息
    fn print_info(msg: &str) {
        println!("信息: {}", msg);
    }

    /// 分割文件路径为文件名和扩展名 - 改为公开方法
    pub fn split_file_path(path: &str) -> (String, String) {
        if let Some(pos) = path.rfind('.') {
            let name = path[..pos].to_string();
            let ext = path[pos + 1..].to_string();
            (name, ext)
        } else {
            (path.to_string(), String::new())
        }
    }
}
