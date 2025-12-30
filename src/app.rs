use crate::utils;
use anyhow::Result; // 移除未使用的 Context 导入
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
        Self::show_program_title();

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
                "4" => {
                    println!("进入批量转换工具");
                    self.run_batch_convert()?;
                }
                "5" => {
                    println!("进入批量清理工具");
                    self.run_batch_clean()?;
                }
                "6" => {
                    println!("进入系统信息工具");
                    self.run_sysinfo()?;
                }
                "0" => {
                    self.show_about_info()?;
                }
                _ => {
                    println!("无效的选择，请重新输入");
                }
            }
        }
    }

    /// 显示程序标题
    fn show_program_title() {
        // 使用简洁标题设计（显示名称和简介，正常显示）
        utils::print_large_simple_title("HEKIT", "      一个工具集合");
    }

    /// 运行批量重命名工具
    fn run_batch_rename(&self) -> Result<()> {
        crate::features::rename::interface::run_interactive()
            .map_err(|e| anyhow::anyhow!("重命名工具执行失败: {}", e))
    }

    /// 运行批量搜索工具
    fn run_batch_search(&self) -> Result<()> {
        crate::features::search::interface::run_interactive()
            .map_err(|e| anyhow::anyhow!("搜索工具执行失败: {}", e))
    }

    /// 运行批量压缩工具
    fn run_batch_compress(&self) -> Result<()> {
        crate::features::compress::interface::run_interactive()
            .map_err(|e| anyhow::anyhow!("压缩工具执行失败: {}", e))
    }

    /// 运行批量转换工具 - 新增方法
    fn run_batch_convert(&self) -> Result<()> {
        crate::features::convert::interface::run_interactive()
            .map_err(|e| anyhow::anyhow!("转换工具执行失败: {}", e))
    }

    /// 运行批量清理工具 - 新增方法
    fn run_batch_clean(&self) -> Result<()> {
        crate::features::clean::interface::run_interactive()
            .map_err(|e| anyhow::anyhow!("清理工具执行失败: {}", e))
    }
    /// 运行系统信息工具
    fn run_sysinfo(&self) -> Result<()> {
        crate::features::sysinfo::interface::run_interactive()
            .map_err(|e| anyhow::anyhow!("系统信息工具执行失败: {}", e))
    }

    /// 显示关于信息（包含检查更新）
    fn show_about_info(&self) -> Result<()> {
        utils::print_separator();
        utils::print_chapter_title("HEKIT - 关于");
        println!("版本: {}", env!("CARGO_PKG_VERSION"));
        println!("作者: zhanghed");

        println!("项目地址: {}", "https://gitee.com/zhanghed/hekit");
        println!("下载地址: {}", "https://gitee.com/zhanghed/hekit/releases");

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
        utils::print_separator();
        Ok(())
    }

    /// 显示主菜单（单列显示，每项间隔空行）
    fn show_main_menu() {
        utils::print_separator();
        println!("HEKIT - 主菜单"); // 取消居中显示，改为左对齐
        utils::print_separator();
        println!(); // 添加空行

        // 菜单项数据
        let menu_items = vec![
            ("1", "批量重命名"),
            ("2", "批量搜索"),
            ("3", "批量压缩"),
            ("4", "批量转换"),
            ("5", "批量清理"),
            ("6", "系统信息工具"),
            ("0", "关于 HEKIT"),
        ];

        // 单列显示，每项一行，项之间添加空行
        for (i, (num, text)) in menu_items.iter().enumerate() {
            println!("      {:>2}. {}", num, text);

            // 在每项后添加空行（除了最后一项）
            if i < menu_items.len() - 1 {
                println!();
            }
        }

        println!(); // 添加空行
        utils::print_separator();
    }
}
