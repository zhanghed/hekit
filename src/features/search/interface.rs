use anyhow::Result;

use crate::features::common;
use crate::features::common::ToolInterface;
use crate::features::search::config::BatchSearchConfig;
use crate::features::search::core::BatchSearchCore;

/// 批量搜索工具接口
pub struct SearchTool;

impl ToolInterface for SearchTool {
    /// 工具名称
    fn tool_name() -> &'static str {
        "批量搜索工具"
    }

    /// 显示使用说明
    fn show_usage() {
        use crate::utils;

        utils::print_separator();
        println!("{:^30}", "批量搜索工具");
        utils::print_separator();

        println!("参数说明:");
        println!("  -d, --path <搜索路径>     搜索路径（默认当前目录）");
        println!("  -n, --name <文件名模式>   文件名匹配模式（支持通配符 *）");
        println!("  -t, --type <文件类型>     按文件类型筛选");
        println!("  --min-size <最小大小>     最小文件大小（字节）");
        println!("  --max-size <最大大小>     最大文件大小（字节）");
        println!("  -r, --recursive           递归搜索子目录");
        println!("  -c, --case                不区分大小写匹配");

        println!("实用示例:");
        println!("  搜索jpg文件: -d \"C:\\\" -n \"*.jpg\" -r");
        println!("  搜索大文件: -n \"*.pdf\" --min-size 1048576");
        utils::print_separator();
    }

    /// 执行命令
    fn execute_command(input: &str) -> Result<()> {
        // 检查输入是否为空
        if input.trim().is_empty() {
            Self::show_usage();
            return Ok(());
        }

        let matches = common::execute_common_command(
            input,
            "search",
            BatchSearchConfig::build_clap_command,
            Self::show_usage,
        )?;

        // 方法1：检查是否显示了帮助信息（通过检查输入是否为"help"）
        if input.trim() == "help" {
            return Ok(());
        }

        // 方法2：检查是否有任何参数被设置
        if !matches.contains_id("path") && !matches.contains_id("name") {
            // 如果没有设置主要参数，可能是显示了帮助信息
            return Ok(());
        }

        // 具体处理逻辑
        let config = BatchSearchConfig::from_matches(&matches)?;

        // 显示搜索进度提示
        println!("高性能搜索工具启动");

        // 核心函数已经处理了所有显示逻辑，这里直接调用即可
        let _ = BatchSearchCore::search_files(&config)?;

        Ok(())
    }
}

/// 运行交互式界面
pub fn run_interactive() -> Result<()> {
    common::run_interactive(SearchTool::tool_name(), SearchTool::execute_command, || {
        SearchTool::show_usage();
    })
}
