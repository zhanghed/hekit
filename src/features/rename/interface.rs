// 确保导入正确
use crate::error::HekitResult;
use crate::features::common::ToolInterface;
use crate::features::rename::config::BatchRenameConfig;
use crate::features::rename::core::BatchRenameCore;

/// 批量重命名工具接口
pub struct RenameTool;

impl ToolInterface for RenameTool {
    /// 工具名称
    fn tool_name() -> &'static str {
        "重命名"
    }

    /// 显示使用说明
    fn show_usage() {
        use crate::utils;

        utils::print_compact_tool_title("批量重命名");
        println!();

        println!("参数说明:");
        println!("  -d, --dir <路径>        指定要重命名的目录");
        println!("  -p, --pattern <模式>    文件名匹配模式");
        println!("  -r, --replace <替换>    替换字符串");
        println!("  -i, --interactive       交互式重命名");
        println!();

        println!("实用示例:");
        println!("  重命名当前目录文件: --dir . --pattern \"*.txt\" --replace \"new_\"");
        println!("  交互式重命名: --dir /path/to/dir --interactive");

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
            "rename",
            BatchRenameConfig::build_clap_command,
            Self::show_usage,
        )?;

        if input.trim() == "help" {
            return Ok(());
        }

        let config = BatchRenameConfig::from_matches(&matches)?;
        let core = BatchRenameCore::new(config);
        core.execute()
    }
}

/// 运行交互式界面
pub fn run_interactive() -> HekitResult<()> {
    crate::features::common::run_interactive(
        RenameTool::tool_name(),
        RenameTool::execute_command,
        || {
            RenameTool::show_usage();
        },
    )
}
