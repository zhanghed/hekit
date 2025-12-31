use crate::error::HekitResult;
use crate::features::common;
use crate::features::common::ToolInterface;
use crate::features::search::config::BatchSearchConfig;
use crate::features::search::core::BatchSearchCore;
use crate::utils;

/// 批量搜索工具接口
pub struct SearchTool;

impl ToolInterface for SearchTool {
    /// 工具名称
    fn tool_name() -> &'static str {
        "批量搜索"
    }

    /// 显示使用说明
    fn show_usage() {
        utils::print_compact_tool_title("批量搜索");
        println!();

        println!("参数说明:");
        println!("  -d, --path <搜索路径>     搜索路径（默认当前目录）");
        println!("  -n, --name <文件名模式>   文件名匹配模式（支持通配符 *）");
        println!("  -t, --type <文件类型>     按文件类型筛选");
        println!("  --min-size <最小大小>     最小文件大小（字节）");
        println!("  --max-size <最大大小>     最大文件大小（字节）");
        println!("  -r, --recursive           递归搜索子目录");
        println!("  -c, --case                不区分大小写匹配");
        println!();

        println!("实用示例:");
        println!("  搜索jpg文件: -d \"C:\\\" -n \"*.jpg\" -r");
        println!("  搜索大文件: -n \"*.pdf\" --min-size 1048576");

        utils::print_compact_separator();
    }

    /// 执行命令
    fn execute_command(input: &str) -> HekitResult<()> {
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

        if input.trim() == "help" {
            return Ok(());
        }

        if !matches.contains_id("path") && !matches.contains_id("name") {
            return Ok(());
        }

        let config = BatchSearchConfig::from_matches(&matches)?;
        utils::print_info("高性能搜索工具启动");
        BatchSearchCore::search_files(&config)?;

        Ok(())
    }
}

/// 运行交互式界面
pub fn run_interactive() -> HekitResult<()> {
    common::run_interactive_hekit(SearchTool::tool_name(), SearchTool::execute_command, || {
        SearchTool::show_usage();
    })
}
