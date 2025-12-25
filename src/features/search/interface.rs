use crate::features::search::config::BatchSearchConfig;
use crate::features::search::core::BatchSearchCore;
use crate::utils;
use anyhow::{anyhow, Result};
use clap::error::ErrorKind;
use shlex::split;
use std::io::{self, Write};

/// 显示使用说明
pub fn show_usage() {
    println!("批量搜索工具");
    println!("参数说明:");
    println!("  -d, --path <搜索路径>     搜索路径（默认当前目录） 示例: -d \"F:\\documents\"");
    println!("  -n, --name <文件名模式>   文件名匹配模式（支持通配符 *） 示例: -n \"*.txt\"");
    println!("  -t, --type <文件类型>     按文件类型筛选 示例: -t \"jpg\"");
    println!("  --min-size <最小大小>     最小文件大小（字节） 示例: --min-size 1024");
    println!("  --max-size <最大大小>     最大文件大小（字节） 示例: --max-size 1048576");
    println!("  -r, --recursive           递归搜索子目录 示例: -r");
    println!("  -c, --case                不区分大小写匹配 示例: -c");
    println!("实用示例:");
    println!("  搜索F:\\documents目录下所有txt文件，递归搜索子目录");
    println!("    -d \"F:\\documents\" -n \"*.txt\" -r");
    println!("  搜索当前目录下大于1MB的PDF文件");
    println!("    -n \"*.pdf\" --min-size 1048576");
}

/// 执行命令行命令
pub fn execute_command(input: &str) -> Result<()> {
    // 检查是否为help命令
    if input.trim() == "help" {
        show_usage();
        return Ok(());
    }

    // 添加虚拟的 "search" 命令前缀
    let full_command = format!("search {}", input);
    let args = match split(&full_command) {
        Some(args) => args,
        None => return Err(anyhow!("命令行参数解析失败")),
    };

    // 使用 try_get_matches_from 来捕获错误
    match BatchSearchConfig::build_clap_command().try_get_matches_from(&args) {
        Ok(matches) => {
            let config = BatchSearchConfig::from_matches(&matches)?;
            let results = BatchSearchCore::search_files(&config)?;

            // 显示搜索结果
            if results.is_empty() {
                utils::print_info("未找到匹配的文件");
            } else {
                utils::print_success(&format!("找到 {} 个匹配的文件：", results.len()));
                for (i, path) in results.iter().enumerate() {
                    println!("{}. {}", i + 1, path.display());
                }
            }
            Ok(())
        }
        Err(e) => match e.kind() {
            ErrorKind::DisplayHelp => {
                show_usage();
                Ok(())
            }
            ErrorKind::DisplayVersion => {
                println!("批量搜索工具 v1.0.0");
                Ok(())
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
    println!("批量搜索工具");
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
