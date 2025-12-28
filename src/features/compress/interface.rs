use crate::error::HekitResult;
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
        use crate::utils;

        utils::print_separator();
        println!("{:^30}", "批量压缩工具");
        utils::print_separator();

        println!("参数说明:");
        println!("  -d, --path <目标文件夹>    目标文件夹（默认当前目录）");
        println!("  -m, --match <文件模式>     选文件（通配符 *）");
        println!("  -f, --format <压缩格式>    压缩格式（zip, tar.gz, tar.bz2）");
        println!("  -o, --output <输出路径>    输出文件路径（默认同目录）");
        println!("  -l, --level <压缩级别>     压缩级别 1-9（默认6）");
        println!("  -r, --recursive           递归处理子目录");
        println!("  -p, --preview             预览效果（不真压缩）");

        println!("实用示例:");
        println!("  压缩图片: -d \"F:\\photos\" -m \"*.jpg\" -f zip -l 9");
        println!("  预览效果: -m \"*.txt\" -p");
        utils::print_separator();
    }

    /// 执行压缩命令
    fn execute_command(input: &str) -> anyhow::Result<()> {
        // 使用公共命令执行函数处理参数解析
        let matches = common::execute_common_command_hekit(
            input,
            "compress",
            BatchCompressConfig::build_clap_command,
            Self::show_usage,
        )?;

        // 创建配置并执行压缩逻辑
        let config = BatchCompressConfig::from_matches(&matches)?;
        let core = BatchCompressCore::new(config);
        core.execute().map_err(|e| anyhow::anyhow!(e.to_string()))
    }
}

/// 运行交互式压缩界面
pub fn run_interactive() -> HekitResult<()> {
    common::run_interactive_hekit(
        CompressTool::tool_name(),
        |input| {
            let matches = common::execute_common_command_hekit(
                input,
                "compress",
                BatchCompressConfig::build_clap_command,
                CompressTool::show_usage,
            )?;
            let config = BatchCompressConfig::from_matches(&matches)?;
            let core = BatchCompressCore::new(config);
            core.execute()
        },
        || {
            CompressTool::show_usage();
        },
    )
}
