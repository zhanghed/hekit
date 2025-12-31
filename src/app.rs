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
            let choice = match utils::get_user_input(&format!(
                "{} 请选择功能 (输入数字): ",
                utils::get_compatible_icon("arrow")
            )) {
                Ok(input) => input,
                Err(e) => {
                    utils::print_compatible_error(&format!("获取用户输入失败: {}", e));
                    continue;
                }
            };

            // 修改进入工具时的提示信息
            match choice.as_str() {
                "1" => {
                    utils::print_compatible_success("进入批量重命名");
                    self.run_batch_rename()?;
                }
                "2" => {
                    utils::print_compatible_success("进入批量搜索");
                    self.run_batch_search()?;
                }
                "3" => {
                    utils::print_compatible_success("进入批量压缩");
                    self.run_batch_compress()?;
                }
                "4" => {
                    utils::print_compatible_success("进入批量转换");
                    self.run_batch_convert()?;
                }
                "5" => {
                    utils::print_compatible_success("进入批量清理");
                    self.run_batch_clean()?;
                }
                "6" => {
                    utils::print_compatible_success("进入系统信息");
                    self.run_sysinfo()?;
                }
                "0" => {
                    self.show_about_info()?;
                }
                _ => {
                    utils::print_compatible_warning("无效的选择，请重新输入");
                }
            }
        }
    }

    /// 显示程序标题（使用超紧凑格式）
    fn show_program_title() {
        let version = env!("CARGO_PKG_VERSION");
        utils::print_super_compact_program_title("HEKIT", &format!("v{} - 工具集合", version));
    }

    // 修改主菜单中的显示
    fn show_main_menu() {
        utils::print_super_compact_program_title("HEKIT", "主菜单");

        // 菜单项数据 - 统一工具名称格式
        let menu_items = vec![
            ("1", "批量重命名", "多种重命名规则，预览模式"),
            ("2", "批量搜索", "文件名模式搜索，文件类型过滤"),
            ("3", "批量压缩", "支持ZIP/TAR格式，可调压缩级别"),
            ("4", "批量转换", "图片格式转换，文档格式转换"),
            ("5", "批量清理", "清理空文件夹、临时文件"),
            ("6", "系统信息", "CPU/内存/磁盘/网络监控"),
            ("0", "关于/更新", "查看程序信息，检查更新"),
        ];

        // 紧凑的菜单显示
        for (number, name, description) in menu_items {
            utils::print_compact_menu_item(number, name, description);
        }

        println!();
        utils::print_compact_separator();
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

    /// 显示关于信息（使用紧凑格式）
    fn show_about_info(&self) -> Result<()> {
        let description = env!("CARGO_PKG_DESCRIPTION");
        let version = env!("CARGO_PKG_VERSION");

        utils::print_super_compact_program_title("HEKIT", &format!("v{} - 工具集合", version));

        utils::print_compatible_info(&format!("项目描述: {}", description));
        utils::print_compatible_info(&format!("作者: zhanghed"));
        utils::print_compatible_info(&format!("版本: {}", version));
        println!();

        utils::print_compatible_info("项目地址:");
        utils::print_clickable_link(
            "https://gitee.com/zhanghed/hekit",
            "https://gitee.com/zhanghed/hekit",
        );

        utils::print_compatible_info("下载地址:");
        utils::print_clickable_link(
            "https://gitee.com/zhanghed/hekit/releases",
            "https://gitee.com/zhanghed/hekit/releases",
        );
        println!();

        // 检查更新
        utils::print_compatible_info("检查更新中...");
        if let Ok((has_update, latest_version)) =
            crate::version::VersionChecker::check_update_sync()
        {
            if has_update && !latest_version.is_empty() {
                utils::print_compatible_success(&format!("发现新版本: {}", latest_version));
                utils::print_compatible_info("请访问下载地址获取最新版本");
            } else {
                utils::print_compatible_success("已是最新版本");
            }
        } else {
            utils::print_compatible_error("检查更新失败");
        }

        utils::print_compact_separator();
        Ok(())
    }
}
