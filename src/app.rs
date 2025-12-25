use crate::utils;
use anyhow::Result;

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        // 同步检查版本更新（简化版）
        self.check_version_simple();

        match self.run_interactive_mode() {
            Ok(_) => {}
            Err(e) => {
                utils::print_error(&format!("程序运行失败: {}", e));
            }
        }
    }

    /// 简化版版本检查（同步方式）
    fn check_version_simple(&self) {
        // 使用简单的线程来执行异步任务
        std::thread::spawn(|| {
            // 创建临时的tokio运行时
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                if let Err(e) = crate::version::VersionChecker::check_update().await {
                    eprintln!("版本检查失败: {}", e);
                }
            });
        });
    }

    pub fn run_interactive_mode(&self) -> Result<()> {
        loop {
            Self::show_main_menu();

            let choice = match utils::get_user_input("请选择 (输入数字): ") {
                Ok(input) => input,
                Err(e) => {
                    utils::print_error(&format!("获取用户输入失败: {}", e));
                    continue;
                }
            };

            match choice.as_str() {
                "1" => {
                    self.run_batch_rename()?;
                }
                "2" => {
                    self.run_batch_search()?;
                }
                "3" => {
                    self.run_batch_compress()?;
                }
                "0" => {
                    utils::print_success("感谢使用HEKIT！");
                    break;
                }
                _ => {
                    utils::print_error("无效的选择，请重新输入");
                }
            }
        }
        Ok(())
    }

    fn run_batch_rename(&self) -> Result<()> {
        crate::features::rename::interface::run_interactive()
    }

    fn run_batch_search(&self) -> Result<()> {
        crate::features::search::interface::run_interactive()
    }

    fn run_batch_compress(&self) -> Result<()> {
        crate::features::compress::interface::run_interactive()
    }

    fn show_main_menu() {
        println!("=== HEKIT - 一个简单实用的命令行工具集合 ===");
        println!("当前版本: v{}", env!("CARGO_PKG_VERSION"));
        println!("1. 批量重命名工具");
        println!("2. 批量搜索工具");
        println!("3. 批量压缩工具");
        println!("0. 退出");
        println!("======================");
    }
}
