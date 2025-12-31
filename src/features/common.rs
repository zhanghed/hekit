use crate::error::{handle_error, HekitError, HekitResult};
use crate::progress::ProgressManager;
use crate::utils;
use clap::{error::ErrorKind, Command};
use std::env;

/// 工具接口特征
pub trait ToolInterface {
    /// 工具名称
    fn tool_name() -> &'static str;

    /// 显示使用说明
    fn show_usage();

    /// 执行命令
    fn execute_command(input: &str) -> HekitResult<()>;
}

/// 运行交互式界面
pub fn run_interactive<F>(
    tool_name: &str,
    execute_fn: F,
    show_usage_fn: impl Fn(),
) -> HekitResult<()>
where
    F: Fn(&str) -> HekitResult<()>,
{
    // 优化工具界面提示，只保留有用的命令
    utils::print_info(&format!("进入 {} 工具", tool_name));
    utils::print_compact_command_hint();

    loop {
        utils::print_prompt(&format!("{} > ", tool_name));
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(|e| HekitError::UserInput(format!("读取输入失败: {}", e)))?;

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // 处理help命令
        if input.eq_ignore_ascii_case("help") {
            show_usage_fn();
            continue;
        }

        // 处理back命令 - 返回主菜单
        if input.eq_ignore_ascii_case("back") {
            utils::print_info("返回主菜单");
            return Err(HekitError::BackToMainMenu("返回主菜单".to_string()));
        }

        // 执行命令
        match execute_fn(input) {
            Ok(_) => {}
            Err(e) => {
                // 如果是返回主菜单的错误，直接向上传递
                if let HekitError::BackToMainMenu(_) = e {
                    return Err(e);
                }
                utils::print_error(&format!("执行失败: {}", e));
            }
        }
    }

    // 删除这行不可达的代码
}

/// 通用的命令执行函数
pub fn execute_common_command<F, G>(
    input: &str,
    command_prefix: &str,
    build_command: F,
    show_usage_fn: G,
) -> HekitResult<clap::ArgMatches>
where
    F: Fn() -> Command,
    G: Fn(),
{
    // 输入验证
    if input.trim().is_empty() {
        return Err(HekitError::UserInput("输入不能为空".to_string()));
    }

    // 检查输入长度
    if input.len() > 1000 {
        return Err(HekitError::UserInput("命令过长，请简化输入".to_string()));
    }

    // 危险字符检查
    let dangerous_patterns = [
        "..\\", "../", "|", "&", ";", "`", "$", ">", "<", "(", ")", "{", "}", "[", "]", "~",
    ];
    for pattern in dangerous_patterns {
        if input.contains(pattern) {
            return Err(HekitError::UserInput(format!(
                "检测到潜在危险字符: {}",
                pattern
            )));
        }
    }

    // 检查路径遍历攻击
    if input.contains("..") && (input.contains("\\") || input.contains("/")) {
        return Err(HekitError::UserInput("检测到路径遍历攻击尝试".to_string()));
    }

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
        Some(args) => {
            // 验证参数数量
            if args.len() > 20 {
                return Err(HekitError::UserInput("参数过多，请简化命令".to_string()));
            }
            args
        }
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

/// 命令行参数分割函数
fn split(input: &str) -> Option<Vec<String>> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut escape_next = false;

    for c in input.chars() {
        if escape_next {
            current.push(c);
            escape_next = false;
            continue;
        }

        match c {
            '\\' => {
                escape_next = true;
            }
            '"' => {
                in_quotes = !in_quotes;
            }
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                // 处理所有其他字符 - 修复：使用current而不是result
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    Some(args)
}

// 预处理Windows路径，修复单反斜杠问题
// 改进的Windows路径预处理函数
fn preprocess_windows_paths(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    let mut in_quotes = false;
    let mut in_path = false;

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                result.push(c);
            }
            '\\' => {
                // 检查是否需要转义反斜杠
                match chars.peek() {
                    // 在引号内或路径末尾需要转义
                    Some(next_char)
                        if next_char == &'"' || next_char == &' ' || next_char == &'\t' =>
                    {
                        result.push_str("\\\\");
                    }
                    // 驱动器路径格式 C:\
                    Some(next_char) if next_char == &':' => {
                        result.push('\\');
                        in_path = true;
                    }
                    // UNC路径格式 \\server\share
                    Some(next_char) if next_char == &'\\' && !in_path => {
                        result.push_str("\\\\");
                        chars.next(); // 跳过第二个反斜杠
                        in_path = true;
                    }
                    // 普通反斜杠
                    _ => {
                        result.push('\\');
                    }
                }
            }
            ' ' | '\t' => {
                if !in_quotes {
                    in_path = false; // 空格结束路径
                }
                result.push(c);
                // 检测路径开始
                if c.is_alphabetic()
                    && chars.peek() == Some(&':')
                    && chars.clone().nth(1) == Some('\\')
                {
                    in_path = true;
                }
            }
            _ => {
                // 处理所有其他字符（包括控制字符、'!'等）
                result.push(c);
            }
        }
    }

    // 额外的验证：确保路径格式正确
    if result.contains(":\\") && !result.contains("\\\\") {
        // 自动修复单反斜杠的驱动器路径
        result = result.replace(":\\", ":\\\\");
    }

    // 处理长路径前缀
    if result.contains("\\\\?\\") {
        // 确保长路径格式正确
        result = result.replace("\\\\?\\\\", "\\\\?\\");
    }

    result
}

/// 通用的批量处理函数（带进度显示）
pub fn execute_batch_operation_with_progress<F, T>(
    items: Vec<T>,
    operation_name: &str,
    operation_fn: F,
) -> HekitResult<()>
where
    F: Fn(&T) -> HekitResult<()>,
{
    if items.is_empty() {
        utils::print_warning("没有找到需要处理的文件");
        return Ok(());
    }

    let total = items.len() as u64;
    let progress_manager = ProgressManager::new(total, operation_name);

    for (index, item) in items.iter().enumerate() {
        // 更新进度消息
        progress_manager.set_message(&format!("处理第 {} 个文件", index + 1));

        // 执行操作
        if let Err(e) = operation_fn(item) {
            progress_manager.finish_with_message("处理失败");
            return Err(e);
        }

        // 更新进度
        progress_manager.inc(1);
    }

    progress_manager.finish_with_message(&format!("{} 完成", operation_name));
    Ok(())
}

/// 通用的长时间运行操作（带不确定进度显示）
pub fn execute_long_running_operation<F>(operation_name: &str, operation_fn: F) -> HekitResult<()>
where
    F: Fn() -> HekitResult<()>,
{
    let progress_manager = ProgressManager::create_indeterminate(operation_name);

    match operation_fn() {
        Ok(_) => {
            progress_manager.finish_with_message(&format!("{} 完成", operation_name));
            Ok(())
        }
        Err(e) => {
            progress_manager.finish_with_message("操作失败");
            Err(e)
        }
    }
}
