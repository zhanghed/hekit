use anyhow::Result;

use crate::features::common;
use crate::features::common::ToolInterface;
use crate::features::rename::config::BatchRenameConfig;
use crate::features::rename::core::BatchRenameCore;

/// 批量重命名工具接口
pub struct RenameTool;

impl ToolInterface for RenameTool {
    /// 工具名称
    fn tool_name() -> &'static str {
        "批量重命名工具"
    }

    /// 显示使用说明
    fn show_usage() {
        use crate::utils;

        utils::print_title("批量重命名工具 - 使用说明");
        utils::print_separator();

        utils::print_info("参数说明:");
        println!(
            "  -d, --path <目标文件夹>     目标文件夹（默认当前目录） 示例: -d \"F:\\hekit\\test\""
        );
        println!(
            "  -m, --match <文件模式>     选文件（通配符 *） 示例: -m \"*.jpg\" 或 -m \"笔记*\""
        );
        println!("  -p, --prefix <前缀>        加前缀 示例: -p \"2025_\"");
        println!("  -s, --suffix <后缀>        加后缀（扩展名前） 示例: -s \"_备份\"");
        println!("  -r, --replace <替换规则>   替换文字 示例: -r \"旧=新\" 或 -r \"/旧/新/\"");
        println!("  -n, --number <起始序号>    加序号（默认1开始，3位补零） 示例: -n 1");
        println!("  -e, --ext <扩展名>         改扩展名 示例: -e \"md\" 或 -e \"\"（删除扩展名）");
        println!("  -v, --preview              预览效果（不真改名） 示例: -v");
        println!("  -b, --backup               备份原文件（自动加.bak后缀） 示例: -b");
        println!("  -c, --case                 不区分大小写匹配 示例: -c");

        utils::print_separator();
        utils::print_info("实用示例:");
        utils::print_success(
            "为F:\\hekit\\test目录下的所有jpg照片添加2024_前缀和从1开始的序号，并预览效果",
        );
        println!("  命令: -d \"F:\\hekit\\test\" -m \"*.jpg\" -p \"2024_\" -n 1 -v");
        utils::print_separator();
    }

    /// 执行命令
    fn execute_command(input: &str) -> Result<()> {
        let matches = common::execute_common_command(
            input,
            "rename",
            BatchRenameConfig::build_clap_command,
            Self::show_usage,
        )?;

        // 具体处理逻辑
        let config = BatchRenameConfig::from_matches(&matches)?;
        let core = BatchRenameCore::new(config);
        core.execute()
    }
}

/// 运行交互式界面
pub fn run_interactive() -> Result<()> {
    common::run_interactive(RenameTool::tool_name(), RenameTool::execute_command, || {
        RenameTool::show_usage();
    })
}
