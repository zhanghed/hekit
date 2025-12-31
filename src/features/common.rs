//! 公共工具接口模块 - 提供统一的工具接口和通用功能
use crate::error::{handle_error, HekitError, HekitResult};
use crate::utils;
use clap::error::ErrorKind;
use shlex::split;

/// 工具接口特征 - 所有工具模块必须实现此特征
pub trait ToolInterface {
    /// 返回工具名称
    fn tool_name() -> &'static str;

    /// 显示工具使用说明
    fn show_usage();

    /// 执行命令 - 输入为命令行字符串，返回执行结果
    fn execute_command(input: &str) -> HekitResult<()>;
}

/// 通用的交互式运行函数（支持 HekitResult）
pub fn run_interactive_hekit<F, G>(
    tool_name: &str,
    execute_fn: F,
    show_usage_fn: G,
) -> HekitResult<()>
where
    F: Fn(&str) -> HekitResult<()>,
    G: Fn(),
{
    utils::print_compact_tool_title(tool_name);
    println!("输入 help 查看详细说明，back 返回上一级");

    loop {
        let input = utils::get_user_input("请输入命令: ")?;
        let trimmed_input = input.trim();

        match trimmed_input {
            "back" => {
                println!("返回主菜单");
                break;
            }
            "help" => {
                show_usage_fn();
            }
            "" => continue,
            _ => {
                // 检查是否为单个字母（可能是误输入）
                if trimmed_input.len() == 1 && trimmed_input.chars().all(|c| c.is_alphabetic()) {
                    utils::print_warning("检测到单字母输入，可能是误操作");
                    utils::print_info("请输入完整的命令参数，如 '-n \"*.txt\"'");
                    utils::print_info("输入 'help' 查看详细使用说明");
                    continue;
                }

                match execute_fn(trimmed_input) {
                    Ok(_) => {
                        utils::print_success("命令执行完成");
                    }
                    Err(e) => {
                        handle_error(&e, "命令执行失败");
                    }
                }
            }
        }
    }
    Ok(())
}

/// 通用的交互式运行函数（支持 anyhow::Result）
pub fn run_interactive<F, G>(tool_name: &str, execute_fn: F, show_usage_fn: G) -> anyhow::Result<()>
where
    F: Fn(&str) -> anyhow::Result<()>,
    G: Fn(),
{
    utils::print_compact_tool_title(tool_name);
    println!("输入 help 查看详细说明，back 返回上一级");

    loop {
        let input = utils::get_user_input("请输入命令: ")?;
        let trimmed_input = input.trim();

        match trimmed_input {
            "back" => {
                println!("返回主菜单");
                break;
            }
            "help" => {
                show_usage_fn();
            }
            "" => continue,
            _ => {
                // 检查是否为单个字母（可能是误输入）
                if trimmed_input.len() == 1 && trimmed_input.chars().all(|c| c.is_alphabetic()) {
                    utils::print_warning("检测到单字母输入，可能是误操作");
                    utils::print_info("请输入完整的命令参数，如 '-n \"*.txt\"'");
                    utils::print_info("输入 'help' 查看详细使用说明");
                    continue;
                }

                match execute_fn(trimmed_input) {
                    Ok(_) => {
                        utils::print_success("命令执行完成");
                    }
                    Err(e) => {
                        handle_error(e.as_ref(), "命令执行失败");
                    }
                }
            }
        }
    }
    Ok(())
}

/// 通用的命令执行函数
pub fn execute_common_command<F>(
    input: &str,
    command_prefix: &str,
    build_command: F,
    show_usage_fn: fn(),
) -> HekitResult<clap::ArgMatches>
where
    F: Fn() -> clap::Command,
{
    // 处理help命令
    if input.trim() == "help" {
        show_usage_fn();
        return Ok(clap::ArgMatches::default());
    }

    // 检查是否为单个字母（可能是误输入）
    let trimmed_input = input.trim();
    if trimmed_input.len() == 1 && trimmed_input.chars().all(|c| c.is_alphabetic()) {
        return Err(HekitError::UserInput(
            "单字母输入，显示使用说明".to_string(),
        ));
    }

    // 预处理Windows路径
    let preprocessed_input = preprocess_windows_paths(input);

    // 解析命令行参数
    let full_command = format!("{} {}", command_prefix, preprocessed_input);
    let args = match split(&full_command) {
        Some(args) => args,
        None => return Err(HekitError::ArgumentParse("命令行参数解析失败".to_string())),
    };

    // 执行命令并处理结果
    match build_command().try_get_matches_from(&args) {
        Ok(matches) => Ok(matches),
        Err(e) => match e.kind() {
            ErrorKind::DisplayHelp => {
                show_usage_fn();
                Ok(clap::ArgMatches::default())
            }
            ErrorKind::DisplayVersion => {
                let version = env!("CARGO_PKG_VERSION");
                utils::print_info(&format!("{} v{}", command_prefix, version));
                Ok(clap::ArgMatches::default())
            }
            _ => {
                // 使用统一的错误处理
                handle_error(&e, "参数解析失败");
                Err(HekitError::ArgumentParse(e.to_string()))
            }
        },
    }
}

// 预处理Windows路径，修复单反斜杠问题
fn preprocess_windows_paths(input: &str) -> String {
    // 处理单反斜杠路径格式 C:\
    // 将 C:\ 转换为 C:\\ 以避免被当作转义字符
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' && chars.peek() == Some(&'"') {
            // 反斜杠后面是引号，需要转义
            result.push_str("\\\\");
            result.push('"');
            chars.next(); // 跳过引号
        } else if c == '\\' && (chars.peek().is_none() || chars.peek() == Some(&' ')) {
            // 反斜杠在路径末尾或后面是空格，需要转义
            result.push_str("\\\\");
        } else {
            result.push(c);
        }
    }

    result
}
