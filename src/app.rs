use anyhow::Result;
use std::io::{self, Write};

use crate::features::batch_rename::{
    show_batch_rename_usage, show_interactive_menu, BatchRenameConfig, BatchRenameExecutor,
};

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
            show_interactive_menu();
            let choice = Self::get_user_input("请选择操作 (输入数字): ");

            match choice.as_str() {
                "1" => {
                    // 批量重命名文件功能
                    show_batch_rename_usage();
                    let input = Self::get_user_input("请输入批量重命名命令: ");
                    self.execute_command(input)?;
                }
                "2" => {
                    // 退出程序
                    Self::print_success("感谢使用！");
                    break;
                }
                _ => {
                    Self::print_error("无效的选择，请重新输入");
                }
            }
        }

        Ok(())
    }

    /// 执行命令（用于交互式模式）
    ///
    /// # 参数
    /// - input: 用户输入的命令字符串
    ///
    /// # 返回值
    /// 执行结果
    pub fn execute_command(&self, input: String) -> Result<()> {
        // 将输入转换为命令行参数
        let mut args = vec!["hekit".to_string()];
        if let Some(split_args) = shlex::split(&input) {
            args.extend(split_args);
        } else {
            anyhow::bail!("命令解析失败");
        }

        // 处理批量重命名命令
        if args.len() >= 2 && args[1] == "batch-rename" {
            return self.execute_batch_rename(&args[2..]);
        }

        println!("未知命令: {}", input);
        Ok(())
    }

    /// 执行批量重命名命令
    ///
    /// # 参数
    /// - args: 命令行参数数组
    ///
    /// # 返回值
    /// 执行结果
    fn execute_batch_rename(&self, args: &[String]) -> Result<()> {
        // 解析命令行参数
        let command = BatchRenameConfig::build_clap_command();
        let matches = command.get_matches_from(args);

        // 创建配置和执行器
        let config = BatchRenameConfig::from_matches(&matches)?;
        let executor = BatchRenameExecutor;

        // 执行批量重命名
        executor.execute(&config)?;

        Ok(())
    }

    // UI工具函数 - 改为公共的，让其他模块可以调用

    /// 获取用户输入
    ///
    /// # 参数
    /// - prompt: 提示信息
    ///
    /// # 返回值
    /// 用户输入的字符串
    pub fn get_user_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    /// 打印错误信息
    ///
    /// # 参数
    /// - msg: 错误消息
    pub fn print_error(msg: &str) {
        eprintln!("错误: {}", msg);
    }

    /// 打印成功信息
    ///
    /// # 参数
    /// - msg: 成功消息
    pub fn print_success(msg: &str) {
        println!("成功: {}", msg);
    }

    /// 打印预览信息
    ///
    /// # 参数
    /// - msg: 预览消息
    pub fn print_preview(msg: &str) {
        println!("预览: {}", msg);
    }

    /// 分割文件路径为文件名和扩展名
    ///
    /// # 参数
    /// - path: 文件路径
    ///
    /// # 返回值
    /// (文件名, 扩展名) 元组
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
