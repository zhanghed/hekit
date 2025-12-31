use crate::error::HekitResult;
use crate::features::clean::config::BatchCleanConfig;
use crate::features::clean::core::BatchCleanCore;
use crate::features::common::ToolInterface;

/// 清理工具接口
pub struct CleanTool;

impl ToolInterface for CleanTool {
    /// 工具名称
    fn tool_name() -> &'static str {
        "批量清理"
    }

    /// 显示使用说明
    fn show_usage() {
        use crate::utils;

        utils::print_compact_tool_title("批量清理工具");
        println!();

        println!("参数说明:");
        println!("  -d, --path       目标文件夹（默认当前目录）");
        println!("  -m, --mode       清理模式: empty(空文件夹), temp(临时文件), log(日志文件), secure(安全删除), custom(自定义)");
        println!("  --days           清理多少天前的日志文件");
        println!("  --patterns       自定义文件模式（用逗号分隔）");
        println!("  -v, --preview    预览模式（不实际删除）");
        println!("  -b, --backup     启用备份功能");
        println!("  --backup-dir     备份目录路径");
        println!();

        println!("实用示例:");
        println!("  清理空文件夹: --mode empty");
        println!("  清理临时文件: --mode temp");
        println!("  清理7天前的日志: --mode log --days 7");

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
            "clean",
            BatchCleanConfig::build_clap_command,
            Self::show_usage,
        )?;

        if input.trim() == "help" {
            return Ok(());
        }

        let config = BatchCleanConfig::from_matches(&matches)
            .map_err(|e| crate::error::HekitError::UserInput(format!("配置错误: {}", e)))?;
        let mut core = BatchCleanCore::new(config);
        let count = core.scan()?;

        if core.config.preview_mode {
            println!("预览模式：找到 {} 个待清理项目", count);
        } else {
            let cleaned = core.execute()?;
            println!("成功清理 {} 个项目", cleaned);
        }

        Ok(())
    }
}

/// 运行交互式界面
pub fn run_interactive() -> HekitResult<()> {
    crate::features::common::run_interactive(
        "批量清理",
        CleanTool::execute_command,
        CleanTool::show_usage,
    )
}
