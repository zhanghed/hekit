use crate::utils;
use anyhow::Result;
use webbrowser;

/// 主应用程序结构体
pub struct App {}

impl App {
    /// 创建新的App实例
    pub fn new() -> Self {
        Self {}
    }

    /// 运行应用程序主循环
    pub fn run(&self) {
        match self.run_interactive_mode() {
            Ok(_) => {}
            Err(e) => {
                utils::print_error(&format!("程序运行失败: {}", e));
            }
        }
    }

    /// 运行交互式模式 - 显示主菜单并处理用户选择
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
                    self.run_check_update()?;
                }
                _ => {
                    utils::print_error("无效的选择，请重新输入");
                }
            }
        }
    }

    /// 运行批量重命名工具
    fn run_batch_rename(&self) -> Result<()> {
        crate::features::rename::interface::run_interactive()
    }

    /// 运行批量搜索工具
    fn run_batch_search(&self) -> Result<()> {
        crate::features::search::interface::run_interactive()
    }

    /// 运行批量压缩工具
    fn run_batch_compress(&self) -> Result<()> {
        crate::features::compress::interface::run_interactive()
    }

    /// 检查更新
    fn run_check_update(&self) -> Result<()> {
        println!("检查更新中...");
        if let Ok((has_update, latest_version)) =
            crate::version::VersionChecker::check_update_sync()
        {
            if has_update && !latest_version.is_empty() {
                Self::show_update_prompt(&latest_version);
                Self::open_download_page();
            } else {
                println!("已是最新版本");
            }
        } else {
            println!("检查更新失败");
        }
        Ok(())
    }

    /// 显示主菜单
    fn show_main_menu() {
        println!("=== HEKIT - 一个简单实用的命令行工具集合 ===");
        println!("1. 批量重命名工具");
        println!("2. 批量搜索工具");
        println!("3. 批量压缩工具");
        println!("0. 检查更新");
        println!("======================");
    }

    /// 显示更新提示
    fn show_update_prompt(latest_version: &str) {
        println!("发现新版本: {}", latest_version);
        println!("正在打开下载页面...");
    }

    /// 打开下载页面
    fn open_download_page() {
        let download_url = "https://gitee.com/zhanghed/hekit/releases";
        match webbrowser::open(download_url) {
            Ok(_) => {
                println!("已打开浏览器");
            }
            Err(_) => {
                println!("无法打开浏览器，请手动访问: {}", download_url);
            }
        }
    }
}
