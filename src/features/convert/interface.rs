use anyhow::Result;

use crate::features::common;
use crate::features::common::ToolInterface;
use crate::features::convert::config::BatchConvertConfig;
use crate::features::convert::core::BatchConvertCore;

/// 批量转换工具接口
pub struct ConvertTool;

impl ToolInterface for ConvertTool {
    /// 工具名称
    fn tool_name() -> &'static str {
        "批量转换工具"
    }

    /// 显示使用说明
    fn show_usage() {
        use crate::utils;

        utils::print_title("批量转换工具 - 使用说明");
        utils::print_separator();

        utils::print_info("参数说明:");
        println!(
            "  -d, --path <源文件夹>     源文件所在文件夹（默认当前目录） 示例: -d \"F:\\photos\""
        );
        println!("  -m, --pattern <文件模式>  文件匹配模式（通配符 *） 示例: -m \"*.jpg\"");
        println!("  -f, --from <源格式>       源文件格式（jpg, png, pdf, txt等） 示例: -f jpg");
        println!("  -t, --to <目标格式>       目标文件格式（png, webp, txt等） 示例: -t png");
        println!("  -o, --output <输出目录>   输出目录（默认源目录） 示例: -o \"F:\\converted\"");
        println!("  -q, --quality <质量>      输出质量（1-100，图片格式有效） 示例: -q 90");
        println!(
            "  -r, --resize <尺寸>       调整尺寸（格式：宽x高，如：800x600） 示例: -r 1920x1080"
        );
        println!("  -v, --preview             预览模式（不实际转换） 示例: -v");
        println!("  -w, --overwrite           覆盖已存在文件 示例: -w");

        utils::print_separator();
        utils::print_info("支持格式:");
        println!("  图片格式: jpg, jpeg, png, webp, bmp, gif");
        println!("  文档格式: pdf, txt");

        utils::print_separator();
        utils::print_info("实用示例:");
        utils::print_success("将当前目录下所有jpg图片转换为png格式，并调整质量为90%");
        println!("  命令: -m \"*.jpg\" -f jpg -t png -q 90");

        utils::print_success(
            "将F:\\photos目录下所有png图片转换为webp，调整尺寸为800x600，预览效果",
        );
        println!("  命令: -d \"F:\\photos\" -m \"*.png\" -f png -t webp -r 800x600 -v");
        utils::print_separator();
    }

    /// 执行命令
    fn execute_command(input: &str) -> Result<()> {
        let matches = common::execute_common_command(
            input,
            "convert",
            BatchConvertConfig::build_clap_command,
            Self::show_usage,
        )?;

        // 具体处理逻辑
        let config = BatchConvertConfig::from_matches(&matches)?;
        let core = BatchConvertCore::new(config);
        core.execute()
    }
}

/// 运行交互式界面
pub fn run_interactive() -> Result<()> {
    common::run_interactive(
        ConvertTool::tool_name(),
        ConvertTool::execute_command,
        || {
            ConvertTool::show_usage();
        },
    )
}
