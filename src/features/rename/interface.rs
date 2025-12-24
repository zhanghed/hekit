use crate::features::rename::config::BatchRenameConfig;
use crate::features::rename::core::BatchRenameCore;
use anyhow::{anyhow, Result};
use shlex;
use std::io::{self, Write};

/// 显示使用说明 - 批量重命名工具
fn show_usage() {
    println!();
    println!("=== 批量重命名工具 ===");
    println!();

    println!("命令格式: rename [选项]");
    println!();

    println!("类型    参数（长 / 短） 作用说明（一句话讲清）  简单示例");
    println!("必选基础    --path / -d 目标文件夹（默认当前目录）  -d 我的图片");
    println!("           --match / -m 选文件（通配符 *）        -m \"*.jpg\" / -m \"笔记*\"");
    println!("常用功能    --prefix / -p 加前缀（如加 \"2025_\")  --prefix \"2025_\"");
    println!("           --suffix / -s 加后缀（扩展名前）       --suffix \"_备份\"");
    println!(
        "           --replace / -r 替换文字（支持正则）     -r \"旧=新\" / -r \"/\\d+/=数字\""
    );
    println!("           --number / -n 加序号（3位补零）        -n（默认）/ -n 5（从5开始）");
    println!("           --ext / -e 改扩展名（空值删除）        -e md / -e \"\"");
    println!("安全保障    --preview / -v 预览效果（不真改名）    -v（必用！）");
    println!("           --backup / -b 备份原文件（加.bak）     -b（一键备份）");
    println!("           --help / -h 查看帮助                  -h");
    println!();

    println!("实用示例:");
    println!("1. 为所有jpg照片添加日期前缀和序号:");
    println!("   rename -m \"*.jpg\" -p \"2024_\" -n -v");
    println!();

    println!("2. 将txt文件转为md格式:");
    println!("   rename -m \"*.txt\" -e md -v");
    println!();

    println!("3. 替换文件名中的数字:");
    println!("   rename -m \"*\" -r \"/\\d+/=数字\" -v");
    println!();

    println!("输入 help 查看详细说明，back 返回上一级");
    println!();
}

/// 执行命令
fn execute_command(command: &str) -> Result<()> {
    if command.trim().is_empty() {
        return Ok(());
    }

    // 预处理命令：如果命令不以"rename"开头，自动添加
    let processed_command = if !command.trim().starts_with("rename") {
        format!("rename {}", command.trim())
    } else {
        command.trim().to_string()
    };

    // 解析命令行参数
    let args: Vec<String> =
        shlex::split(&processed_command).ok_or_else(|| anyhow!("命令解析失败"))?;

    let matches = BatchRenameConfig::build_clap_command()
        .try_get_matches_from(&args)
        .map_err(|e| anyhow!("参数解析失败: {}", e))?;

    let config = BatchRenameConfig::from_matches(&matches)?;
    let renamer = BatchRenameCore::new(config);

    renamer.execute()
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
    println!("=== 批量重命名工具 ===");
    println!("输入 help 查看详细说明，back 返回上一级");
    println!("命令格式: rename [选项]");
    println!("示例: rename -m \"*.txt\" -p \"test_\" -v");
    println!();

    loop {
        let input = get_user_input("请输入命令: ")?;

        match input.as_str() {
            "back" | "返回" | "0" => {
                println!("返回主菜单");
                break;
            }
            "help" | "?" => {
                show_usage();
            }
            "" => {
                // 空输入，继续循环
                continue;
            }
            _ => {
                if let Err(e) = execute_command(&input) {
                    eprintln!("命令执行失败: {}", e);
                    eprintln!("请输入 'help' 查看正确的命令格式");
                    eprintln!("示例: rename -m \"*.txt\" -p \"test_\" -v");
                }
            }
        }
    }

    Ok(())
}
