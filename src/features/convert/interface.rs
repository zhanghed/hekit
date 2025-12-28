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

        utils::print_separator();
        println!("{:^30}", "批量转换工具");
        utils::print_separator();

        println!("参数说明:");
        println!("  -d, --path <源文件夹>     源文件所在文件夹（默认当前目录）");
        println!("  -m, --pattern <文件模式>  文件匹配模式（通配符 *）");
        println!("  -f, --from <源格式>       源文件格式（jpg, png, pdf, txt等）");
        println!("  -t, --to <目标格式>       目标文件格式（png, webp, txt等）");
        println!("  -o, --output <输出目录>   输出目录（默认源目录）");
        println!("  -q, --quality <质量>      输出质量（1-100，图片格式有效）");
        println!("  -r, --resize <尺寸>       调整尺寸（格式：宽x高）");
        println!("  -v, --preview             预览模式（不实际转换）");
        println!("  -w, --overwrite           覆盖已存在文件");

        println!("支持格式:");
        println!("  图片格式: jpg, jpeg, png, webp, bmp, gif");
        println!("  文档格式: pdf, txt");

        println!("实用示例:");
        println!("  将jpg转换为png: -m \"*.jpg\" -f jpg -t png -q 90");
        println!("  转换并调整尺寸: -d \"F:\\photos\" -m \"*.png\" -f png -t webp -r 800x600 -v");
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
