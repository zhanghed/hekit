use crate::error::HekitResult;
use crate::features::common;
use crate::features::common::ToolInterface;
use crate::features::sysinfo::config::SysInfoConfig;
use crate::features::sysinfo::core::SysInfoCore;

/// 系统信息工具接口
pub struct SysInfoTool;

impl ToolInterface for SysInfoTool {
    /// 工具名称
    fn tool_name() -> &'static str {
        "系统信息"
    }

    /// 显示使用说明
    fn show_usage() {
        use crate::utils;

        utils::print_compact_tool_title("系统信息");
        println!();

        println!("参数说明:");
        println!("  -a, --all        显示所有信息");
        println!("  -b, --basic      显示基本系统信息");
        println!("  -c, --cpu        显示CPU信息");
        println!("  -m, --memory     显示内存信息");
        println!("  -d, --disk       显示磁盘信息");
        println!("  -n, --network    显示网络信息");
        println!("  -p, --processes  显示进程信息");
        println!("  -r, --refresh    刷新系统信息");
        println!();

        println!("实用示例:");
        println!("  显示完整系统信息: --all");
        println!("  显示CPU信息: --cpu");
        println!("  显示内存信息: --memory");

        utils::print_compact_separator();
    }

    /// 执行命令
    fn execute_command(input: &str) -> HekitResult<()> {
        if input.trim().is_empty() {
            Self::show_usage();
            return Ok(());
        }

        let matches = common::execute_common_command(
            input,
            "sysinfo",
            SysInfoConfig::build_clap_command,
            Self::show_usage,
        )?;

        if input.trim() == "help" {
            return Ok(());
        }

        let config = SysInfoConfig::from_matches(&matches)?;
        let core = SysInfoCore::new(config);
        core.execute()
    }
}

/// 运行交互式界面
pub fn run_interactive() -> HekitResult<()> {
    common::run_interactive_hekit(
        SysInfoTool::tool_name(),
        SysInfoTool::execute_command,
        || {
            SysInfoTool::show_usage();
        },
    )
}
