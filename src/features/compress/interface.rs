use crate::error::HekitResult;
use crate::features::common::ToolInterface;
use crate::features::compress::config::BatchCompressConfig;
use crate::features::compress::core::BatchCompressCore;

/// 压缩工具接口
pub struct CompressTool;

impl ToolInterface for CompressTool {
    /// 工具名称
    fn tool_name() -> &'static str {
        "批量压缩"
    }

    /// 显示使用说明
    fn show_usage() {
        use crate::utils;

        utils::print_compact_tool_title("批量压缩工具");
        println!();

        println!("参数说明:");
        println!("  -d, --path       目标文件夹（默认当前目录）");
        println!("  -m, --match      文件匹配模式（通配符 *）");
        println!("  -f, --format     压缩格式（zip, tar.gz, tar.bz2）");
        println!("  -o, --output     输出文件路径");
        println!("  -l, --level      压缩级别 1-9（默认6）");
        println!("  -r, --recursive  递归处理子目录");
        println!("  -p, --preview    预览效果（不真压缩）");
        println!();

        println!("实用示例:");
        println!("  压缩所有txt文件: --match *.txt");
        println!("  压缩图片到tar.gz: --match *.jpg --format tar.gz");
        println!("  高压缩级别: --level 9");

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
            "compress",
            BatchCompressConfig::build_clap_command,
            Self::show_usage,
        )?;

        if input.trim() == "help" {
            return Ok(());
        }

        let config = BatchCompressConfig::from_matches(&matches)
            .map_err(|e| crate::error::HekitError::UserInput(format!("配置错误: {}", e)))?;
        let core = BatchCompressCore::new(config);

        // 修复：使用scan_files方法而不是scan
        let files = core.scan_files()?;
        let count = files.len();

        if core.config.preview {
            println!("预览模式：找到 {} 个待压缩文件", count);
        } else {
            // 修复：execute方法返回HekitResult<()>，不需要接收返回值
            core.execute()?;
            println!("成功压缩 {} 个文件", count);
        }

        Ok(())
    }
}

/// 运行交互式界面
pub fn run_interactive() -> HekitResult<()> {
    crate::features::common::run_interactive(
        "批量压缩",
        CompressTool::execute_command,
        CompressTool::show_usage,
    )
}
