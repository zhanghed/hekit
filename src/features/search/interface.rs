use crate::error::HekitResult;
use crate::features::common::ToolInterface;
use crate::features::search::config::BatchSearchConfig;

/// 搜索工具接口
pub struct SearchTool;

impl ToolInterface for SearchTool {
    /// 工具名称
    fn tool_name() -> &'static str {
        "文件搜索"
    }

    /// 显示使用说明
    fn show_usage() {
        use crate::utils;

        utils::print_compact_tool_title("文件搜索工具");
        println!();

        println!("参数说明:");
        println!("  -d, --path       搜索目录（默认当前目录）");
        println!("  -n, --name       文件名模式（支持通配符 *）");
        println!("  -t, --type       文件类型（扩展名，如：txt, jpg）");
        println!("  -s, --size       文件大小范围（格式：min-max，如：1K-10M）");
        println!("  -r, --recursive  递归搜索子目录");
        println!("  -i, --ignore-case 忽略大小写");
        println!("  -c, --content    搜索文件内容（暂不支持）");
        println!();

        println!("实用示例:");
        println!("  搜索所有txt文件: --name *.txt");
        println!("  搜索图片文件: --type jpg --type png");
        println!("  搜索大文件: --size 10M-100M");

        utils::print_compact_separator();
    }

    /// 执行命令
    fn execute_command(input: &str) -> HekitResult<()> {
        if input.trim().is_empty() {
            Self::show_usage();
            return Ok(());
        }

        let matches = crate::features::common::execute_common_command(
            input,
            "search",
            BatchSearchConfig::build_clap_command,
            Self::show_usage,
        )?;

        if input.trim() == "help" {
            return Ok(());
        }

        let config = BatchSearchConfig::from_matches(&matches)
            .map_err(|e| crate::error::HekitError::UserInput(format!("配置错误: {}", e)))?;

        // 修复：使用静态方法而非new方法
        let (results, skipped) =
            crate::features::search::core::BatchSearchCore::search_files(&config)?;

        println!("搜索完成！找到 {} 个文件", results.len());
        if skipped > 0 {
            println!("跳过 {} 个无法访问的目录", skipped);
        }

        Ok(())
    }
}

/// 运行交互式界面
pub fn run_interactive() -> HekitResult<()> {
    crate::features::common::run_interactive(
        "文件搜索",
        SearchTool::execute_command,
        SearchTool::show_usage,
    )
}
