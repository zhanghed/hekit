use crate::utils;
use anyhow::Result;
use std::env;

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
                    println!("获取用户输入失败: {}", e);
                    continue;
                }
            };
            match choice.as_str() {
                "1" => {
                    println!("进入批量重命名工具");
                    self.run_batch_rename()?;
                }
                "2" => {
                    println!("进入批量搜索工具");
                    self.run_batch_search()?;
                }
                "3" => {
                    println!("进入批量压缩工具");
                    self.run_batch_compress()?;
                }
                "0" => {
                    self.show_about_info()?;
                }
                _ => {
                    println!("无效的选择，请重新输入");
                }
            }
            // 移除空行，使用空格分隔
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

    /// 显示关于信息（包含检查更新）
    fn show_about_info(&self) -> Result<()> {
        utils::print_chapter_title("HEKIT - 关于");
        println!("版本: {}", env!("CARGO_PKG_VERSION"));
        println!("作者: zhanghed");
        println!("项目地址: https://github.com/zhanghed/hekit");
        println!("下载地址: https://gitee.com/zhanghed/hekit/releases");
        utils::print_simple_separator();

        // 检查更新但不主动跳转
        println!("检查更新中...");
        if let Ok((has_update, latest_version)) =
            crate::version::VersionChecker::check_update_sync()
        {
            if has_update && !latest_version.is_empty() {
                println!("发现新版本: {}", latest_version);
                println!("请访问下载地址获取最新版本");
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
        utils::print_title("HEKIT - 一个简单实用的命令行工具集合");
        utils::print_menu_item("1", "批量重命名工具");
        utils::print_menu_item("2", "批量搜索工具");
        utils::print_menu_item("3", "批量压缩工具");
        utils::print_menu_item("0", "关于HEKIT");
        utils::print_separator();
    }
}
