use crate::error::HekitResult;
use crate::features::common::ToolInterface;
use crate::features::convert::config::BatchConvertConfig;
use crate::features::convert::core::BatchConvertCore;

/// 转换工具接口
pub struct ConvertTool;

impl ToolInterface for ConvertTool {
    /// 工具名称
    fn tool_name() -> &'static str {
        "批量转换"
    }

    /// 显示使用说明
    fn show_usage() {
        use crate::utils;

        utils::print_compact_tool_title("批量文件格式转换工具");
        println!();

        println!("参数说明:");
        println!("  -d, --path       源文件所在文件夹（默认当前目录）");
        println!("  -m, --pattern    文件匹配模式（通配符 *）");
        println!("  -f, --from       源文件格式（如：jpg, png, pdf）");
        println!("  -t, --to         目标文件格式（如：png, webp, txt）");
        println!("  -o, --output     输出目录（默认源目录）");
        println!("  -q, --quality    输出质量（1-100，仅图片格式有效）");
        println!("  -r, --resize     调整尺寸（格式：宽x高，如：800x600）");
        println!("  -v, --preview    预览模式（不实际转换）");
        println!("  -w, --overwrite  覆盖已存在文件");
        println!();

        println!("实用示例:");
        println!("  JPG转PNG: --from jpg --to png");
        println!("  调整图片质量: --quality 80");
        println!("  调整图片尺寸: --resize 800x600");

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
            "convert",
            BatchConvertConfig::build_clap_command,
            Self::show_usage,
        )?;

        if input.trim() == "help" {
            return Ok(());
        }

        let config = BatchConvertConfig::from_matches(&matches)
            .map_err(|e| crate::error::HekitError::UserInput(format!("配置错误: {}", e)))?;
        let core = BatchConvertCore::new(config);

        // 修复：使用find_files方法而不是scan
        let files = core.find_files()?;
        let count = files.len();

        if core.config.preview {
            println!("预览模式：找到 {} 个待转换文件", count);
        } else {
            // 修复：execute方法返回HekitResult<()>，不需要接收返回值
            core.execute()?;
            println!("成功转换 {} 个文件", count);
        }

        Ok(())
    }
}

/// 运行交互式界面
pub fn run_interactive() -> HekitResult<()> {
    crate::features::common::run_interactive(
        "批量转换",
        ConvertTool::execute_command,
        ConvertTool::show_usage,
    )
}
