use anyhow::Result;

use crate::features::common;
use crate::features::common::ToolInterface;
use crate::features::compress::config::BatchCompressConfig;
use crate::features::compress::core::BatchCompressCore;

/// 批量压缩工具接口实现
pub struct CompressTool;

impl ToolInterface for CompressTool {
    /// 工具名称
    fn tool_name() -> &'static str {
        "批量压缩工具"
    }

    /// 显示使用说明
    fn show_usage() {
        println!("批量压缩工具");
        println!("参数说明:");
        println!(
            "  -d, --path <目标文件夹>    目标文件夹（默认当前目录） 示例: -d \"F:\\documents\""
        );
        println!("  -m, --match <文件模式>     选文件（通配符 *） 示例: -m \"*.jpg\"");
        println!("  -f, --format <压缩格式>    压缩格式（zip, tar.gz, tar.bz2） 示例: -f zip");
        println!("  -o, --output <输出路径>    输出文件路径（默认同目录） 示例: -o \"F:\\backup\"");
        println!("  -l, --level <压缩级别>     压缩级别 1-9（默认6） 示例: -l 9");
        println!("  -r, --recursive           递归处理子目录 示例: -r");
        println!("  -p, --preview             预览效果（不真压缩） 示例: -p");
        println!("实用示例:");
        println!("  压缩F:\\photos目录下所有jpg图片为zip格式，最高压缩级别");
        println!("    -d \"F:\\photos\" -m \"*.jpg\" -f zip -l 9");
        println!("  预览当前目录下所有txt文件的压缩效果");
        println!("    -m \"*.txt\" -p");
    }

    /// 执行压缩命令
    fn execute_command(input: &str) -> Result<()> {
        // 使用公共命令执行函数处理参数解析
        let matches = common::execute_common_command(
            input,
            "compress",
            BatchCompressConfig::build_clap_command,
            Self::show_usage,
        )?;

        // 创建配置并执行压缩逻辑
        let config = BatchCompressConfig::from_matches(&matches)?;
        let core = BatchCompressCore::new(config);
        core.execute()
    }
}

/// 运行交互式压缩界面
pub fn run_interactive() -> Result<()> {
    common::run_interactive(
        CompressTool::tool_name(),
        CompressTool::execute_command,
        || {
            CompressTool::show_usage();
        },
    )
}
