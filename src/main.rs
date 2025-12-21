use clap::{Parser, Subcommand}; // 移除未使用的 anyhow::Result

mod core;
mod features;

use core::utils;
use features::batch_renamer;

/// hekit - 轻量级 Rust 命令行工具
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// 支持的命令
#[derive(Subcommand)]
enum Commands {
    /// 批量重命名文件
    BatchRename {
        /// 要处理的目录路径
        #[arg(short = 'd', long, default_value = ".")] // 使用 'd' 作为短选项
        dir: String,

        /// 是否递归处理子目录
        #[arg(short = 'R', long, default_value_t = false)] // 使用 'R' 作为短选项（大写）
        recursive: bool,

        /// 要筛选的文件扩展名
        #[arg(short = 'e', long)] // 使用 'e' 作为短选项
        extension: Option<String>,

        /// 添加前缀
        #[arg(short = 'p', long)] // 使用 'p' 作为短选项
        prefix: Option<String>,

        /// 添加后缀
        #[arg(short = 's', long)] // 使用 's' 作为短选项
        suffix: Option<String>,

        /// 替换文字（格式：old=new）
        #[arg(short = 'r', long)] // 使用 'r' 作为短选项
        replace: Option<String>,

        /// 使用序号重命名
        #[arg(short = 'i', long, default_value_t = false)]
        // 使用 'i' 作为短选项（表示 index/number）
        serial: bool,

        /// 序号起始值
        #[arg(long, default_value_t = 1)] // 不使用短选项，只使用长选项
        start: u32,

        /// 新的文件扩展名
        #[arg(short = 'x', long)] // 使用 'x' 作为短选项
        new_ext: Option<String>,

        /// 干运行（预览不执行）
        #[arg(short = 'n', long, default_value_t = false)] // 使用 'n' 作为短选项
        dry_run: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::BatchRename {
            dir,
            recursive,
            extension,
            prefix,
            suffix,
            replace,
            serial,
            start,
            new_ext,
            dry_run,
        } => {
            // 解析替换参数
            let replace_pair = if let Some(replace_str) = replace {
                let parts: Vec<&str> = replace_str.split('=').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    utils::print_error("替换参数格式错误，应为：old=new");
                    return;
                }
            } else {
                None
            };

            // 创建配置
            let config = batch_renamer::BatchRenameConfig {
                dir_path: dir,
                recursive,
                extension,
                prefix,
                suffix,
                replace: replace_pair,
                serial_number: serial,
                start_number: start,
                new_extension: new_ext,
                dry_run,
            };

            // 执行重命名
            if let Err(e) = batch_renamer::run(&config) {
                utils::print_error(&format!("执行失败：{}", e));
            }
        }
    }
}
