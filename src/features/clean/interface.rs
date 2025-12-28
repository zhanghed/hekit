use crate::features::clean::config::BatchCleanConfig;
use crate::features::clean::core::BatchCleanCore;
use crate::features::common;
use crate::features::common::ToolInterface;
use anyhow::Result;

/// 批量清理工具接口
pub struct CleanTool;

impl ToolInterface for CleanTool {
    /// 工具名称
    fn tool_name() -> &'static str {
        "批量清理工具"
    }

    /// 显示使用说明
    fn show_usage() {
        use crate::utils;

        utils::print_separator();
        println!("{}", "批量清理工具"); // 取消居中：{:^30} -> {}
        utils::print_separator();

        println!("参数说明:");
        println!("  -d, --path <目标文件夹>     目标文件夹（默认当前目录）");
        println!("  -m, --mode <清理模式>       清理模式: empty(空文件夹), temp(临时文件), log(日志文件), secure(安全删除), custom(自定义)");
        println!("  --days <天数>               清理多少天前的日志文件");
        println!("  --patterns <模式>           自定义文件模式（用逗号分隔）");
        println!("  -v, --preview               预览模式（不实际删除）");
        println!("  -b, --backup                启用备份功能");
        println!("  --backup-dir <备份目录>     备份目录路径");

        println!("实用示例:");
        println!("  清理空文件夹: -d \"F:\\test\" -m empty -v");
        println!("  清理临时文件: -m temp -b");
        println!("  清理7天前日志: -m log --days 7");
        utils::print_separator();
    }

    /// 执行命令
    fn execute_command(input: &str) -> Result<()> {
        let matches = common::execute_common_command(
            input,
            "clean",
            BatchCleanConfig::build_clap_command,
            Self::show_usage,
        )?;

        // 如果显示了帮助信息或没有有效参数，直接返回
        if input.trim() == "help" {
            return Ok(());
        }

        // 检查是否有有效参数（通过检查是否有目标目录或其他必需参数）
        if matches.get_one::<String>("path").is_none()
            && matches.get_one::<String>("mode").is_none()
        {
            // 如果没有提供必需参数，显示使用说明
            Self::show_usage();
            return Ok(());
        }

        // 具体处理逻辑
        let config =
            BatchCleanConfig::from_matches(&matches).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let mut core = BatchCleanCore::new(config);

        println!("开始扫描目标目录...");
        let found_count = core.scan().map_err(|e| anyhow::anyhow!(e.to_string()))?;

        if found_count == 0 {
            println!("未找到需要清理的文件或文件夹。");
            return Ok(());
        }

        println!("找到 {} 个需要清理的项目:", found_count);

        // 显示找到的文件
        let files = core.get_files_to_clean();
        let folders = core.get_folders_to_clean();

        if !files.is_empty() {
            println!("\n文件列表:");
            for file in files {
                println!("  - {:?}", file);
            }
        }

        if !folders.is_empty() {
            println!("\n文件夹列表:");
            for folder in folders {
                println!("  - {:?}", folder);
            }
        }

        // 确认执行
        if core.config.preview_mode {
            println!("\n当前为预览模式，不会实际删除文件。");
        } else {
            print!("\n确认执行清理操作？(y/n): ");
            let mut confirm_input = String::new();
            std::io::stdin()
                .read_line(&mut confirm_input)
                .map_err(|e| anyhow::anyhow!(format!("读取输入失败: {}", e)))?;

            if !confirm_input.trim().eq_ignore_ascii_case("y") {
                println!("操作已取消。");
                return Ok(());
            }
        }

        // 执行清理
        let cleaned_count = core.execute().map_err(|e| anyhow::anyhow!(e.to_string()))?;

        if core.config.preview_mode {
            println!("预览完成，将清理 {} 个项目。", cleaned_count);
        } else {
            println!("清理完成，已清理 {} 个项目。", cleaned_count);
        }

        Ok(())
    }
}

/// 运行交互式界面
pub fn run_interactive() -> Result<()> {
    common::run_interactive(
        CleanTool::tool_name(),
        |input| CleanTool::execute_command(input),
        || {
            CleanTool::show_usage();
        },
    )
}
