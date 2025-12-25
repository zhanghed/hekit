use crate::features::compress::config::BatchCompressConfig;
use crate::features::compress::core::BatchCompressCore;
use crate::utils;
use anyhow::{anyhow, Result};
use clap::error::ErrorKind;
use shlex::split;
use std::io::{self, Write};

/// 显示使用说明
pub fn show_usage() {
    println!("批量压缩工具");
    println!("参数说明:");
    println!("  -d, --path <目标文件夹>    目标文件夹（默认当前目录） 示例: -d \"F:\\documents\"");
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

/// 执行命令行命令
pub fn execute_command(input: &str) -> Result<()> {
    // 检查是否为help命令
    if input.trim() == "help" {
        show_usage();
        return Ok(());
    }

    // 添加虚拟的 "compress" 命令前缀
    let full_command = format!("compress {}", input);
    let args = match split(&full_command) {
        Some(args) => args,
        None => return Err(anyhow!("命令行参数解析失败")),
    };

    // 使用 try_get_matches_from 来捕获错误
    match BatchCompressConfig::build_clap_command().try_get_matches_from(&args) {
        Ok(matches) => {
            let config = BatchCompressConfig::from_matches(&matches)?;
            let core = BatchCompressCore::new(config);
            core.execute()
        }
        Err(e) => match e.kind() {
            ErrorKind::DisplayHelp => {
                show_usage();
                Ok(())
            }
            ErrorKind::DisplayVersion => {
                println!("批量压缩工具 v1.0.0");
                Ok(())
            }
            ErrorKind::MissingRequiredArgument => {
                if e.to_string().contains("--match") {
                    Err(anyhow!(
                        "缺少必要参数：必须指定 --match 参数来选择要压缩的文件"
                    ))
                } else {
                    Err(anyhow!("参数解析失败: {}", e))
                }
            }
            _ => Err(anyhow!("参数解析失败: {}", e)),
        },
    }
}

/// 获取用户输入
fn get_user_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

/// 运行交互式界面
pub fn run_interactive() -> Result<()> {
    println!("批量压缩工具");
    println!("输入 help 查看详细说明，back 返回上一级");

    loop {
        let input = get_user_input("请输入命令: ")?;

        match input.as_str() {
            "back" => {
                println!("返回主菜单");
                break;
            }
            "help" => {
                show_usage();
            }
            "" => {
                // 空输入，继续循环
                continue;
            }
            _ => {
                if let Err(e) = execute_command(&input) {
                    utils::print_error(&format!("命令执行失败: {}", e));
                    println!("请输入 'help' 查看正确的命令格式");
                }
            }
        }
    }

    Ok(())
}
