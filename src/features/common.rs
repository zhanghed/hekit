//! 公共工具接口模块 - 提供统一的工具接口和通用功能
use crate::utils;
use anyhow::{anyhow, Result};
use clap::error::ErrorKind;
use shlex::split;

/// 工具接口特征 - 所有工具模块必须实现此特征
pub trait ToolInterface {
    /// 返回工具名称
    fn tool_name() -> &'static str;

    /// 显示工具使用说明
    fn show_usage();

    /// 执行命令 - 输入为命令行字符串，返回执行结果
    fn execute_command(input: &str) -> Result<()>;
}

/// 通用的交互式运行函数
/// # 参数
/// - `tool_name`: 工具名称，用于界面显示
/// - `execute_fn`: 命令执行函数
/// - `show_usage_fn`: 显示使用说明的函数
pub fn run_interactive<F, G>(tool_name: &str, execute_fn: F, show_usage_fn: G) -> Result<()>
where
    F: Fn(&str) -> Result<()>,
    G: Fn(),
{
    utils::print_chapter_title(&format!("{}", tool_name));
    println!("输入 help 查看详细说明，back 返回上一级");

    loop {
        let input = utils::get_user_input("请输入命令: ")?;
        match input.as_str() {
            "back" => {
                println!("返回主菜单");
                break;
            }
            "help" => {
                show_usage_fn();
                // 移除空行，使用空格分隔
            }
            "" => continue, // 空输入，继续循环
            _ => {
                match execute_fn(&input) {
                    Ok(_) => {
                        println!("命令执行完成");
                        // 移除空行，使用空格分隔
                    }
                    Err(e) => {
                        let error_msg = e.to_string();
                        if error_msg == "显示帮助信息" || error_msg == "显示版本信息" {
                            // 帮助信息和版本信息不是真正的错误
                        } else {
                            println!("命令执行失败: {e}");
                            println!("请输入 'help' 查看正确的命令格式");
                            // 移除空行，使用空格分隔
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// 通用的命令执行函数 - 处理命令行参数解析和错误处理
/// # 参数
/// - `input`: 用户输入的命令行字符串
/// - `command_prefix`: 命令前缀（如"compress"、"rename"）
/// - `build_command`: 构建CLAP命令的函数
/// - `show_usage_fn`: 显示使用说明的函数
pub fn execute_common_command<F>(
    input: &str,
    command_prefix: &str,
    build_command: F,
    show_usage_fn: fn(),
) -> Result<clap::ArgMatches>
where
    F: Fn() -> clap::Command,
{
    // 处理help命令
    if input.trim() == "help" {
        show_usage_fn();
        return Ok(clap::ArgMatches::default()); // 返回空的匹配结果，表示正常显示帮助
    }

    // 解析命令行参数
    let full_command = format!("{} {}", command_prefix, input);
    let args = match split(&full_command) {
        Some(args) => args,
        None => return Err(anyhow!("命令行参数解析失败")),
    };

    // 执行命令并处理结果
    match build_command().try_get_matches_from(&args) {
        Ok(matches) => Ok(matches),
        Err(e) => match e.kind() {
            ErrorKind::DisplayHelp => {
                show_usage_fn();
                Ok(clap::ArgMatches::default()) // 返回空的匹配结果，表示正常显示帮助
            }
            ErrorKind::DisplayVersion => {
                utils::print_info(&format!("{} v1.0.0", command_prefix));
                Ok(clap::ArgMatches::default()) // 返回空的匹配结果，表示正常显示版本
            }
            _ => Err(anyhow!("参数解析失败: {e}")),
        },
    }
}
