use anyhow::Result;

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
        "批量搜索工具"
    }

    /// 显示使用说明
    fn show_usage() {
        println!("批量搜索工具");
        println!("参数说明:");
        println!("  -d, --path <搜索路径>     搜索路径（默认当前目录） 示例: -d \"F:\\documents\"");
        println!("  -n, --name <文件名模式>   文件名匹配模式（支持通配符 *） 示例: -n \"*.txt\"");
        println!("  -t, --type <文件类型>     按文件类型筛选 示例: -t \"jpg\"");
        println!("  --min-size <最小大小>     最小文件大小（字节） 示例: --min-size 1024");
        println!("  --max-size <最大大小>     最大文件大小（字节） 示例: --max-size 1048576");
        println!("  -r, --recursive           递归搜索子目录 示例: -r");
        println!("  -c, --case                不区分大小写匹配 示例: -c");
        println!("实用示例:");
        println!("  搜索F:\\documents目录下所有txt文件，递归搜索子目录");
        println!("    -d \"F:\\documents\" -n \"*.txt\" -r");
        println!("  搜索当前目录下大于1MB的PDF文件");
        println!("    -n \"*.pdf\" --min-size 1048576");
    }

    /// 执行命令
    fn execute_command(input: &str) -> Result<()> {
        let matches = common::execute_common_command(
            input,
            "search",
            BatchSearchConfig::build_clap_command,
            Self::show_usage,
        )?;

        // 具体处理逻辑
        let config = BatchSearchConfig::from_matches(&matches)?;
        let results = BatchSearchCore::search_files(&config)?;
        if results.is_empty() {
            utils::print_info("未找到匹配的文件");
        } else {
            utils::print_success_format(
                "找到 {count} 个匹配的文件：",
                &[("count", &results.len())],
            );
            for (i, path) in results.iter().enumerate() {
                println!("{}. {}", i + 1, path.display());
            }
        }
        Ok(())
    }
}

/// 运行交互式界面
pub fn run_interactive() -> Result<()> {
    common::run_interactive(SearchTool::tool_name(), SearchTool::execute_command)
}
