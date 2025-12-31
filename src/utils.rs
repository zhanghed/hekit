use anyhow::Result;
use std::io::{self, Write};

/// 获取用户输入
pub fn get_user_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// 打印错误信息
pub fn print_error(msg: &str) {
    println!("{}", msg);
}

/// 打印成功信息
pub fn print_success(msg: &str) {
    println!("{}", msg);
}

/// 打印信息
pub fn print_info(msg: &str) {
    println!("{}", msg);
}

/// 打印警告信息
pub fn print_warning(msg: &str) {
    println!("{}", msg);
}

/// 打印提示信息
pub fn print_prompt(msg: &str) {
    println!("{}", msg);
}

/// 打印醒目标题
pub fn print_banner_title(title: &str) {
    let separator = "=".repeat(title.len() + 4);
    println!("{}", separator);
    println!("  {}  ", title);
    println!("{}", separator);
    println!();
}

/// 检测终端是否支持OSC 8协议（可点击链接）
pub fn supports_osc8() -> bool {
    // 更严格的检测逻辑，只在确认支持的终端中启用
    if let Ok(term) = std::env::var("TERM_PROGRAM") {
        if term.contains("WindowsTerminal") || term.contains("vscode") {
            return true;
        }
    }

    if let Ok(term) = std::env::var("WT_SESSION") {
        // Windows Terminal会话
        return !term.is_empty();
    }

    // 检查是否在传统CMD中
    if let Ok(comspec) = std::env::var("COMSPEC") {
        if comspec.to_lowercase().contains("cmd.exe") {
            return false;
        }
    }

    // 默认禁用，避免在不支持的终端中显示乱码
    false
}

/// 创建可点击的链接（支持Windows终端和现代终端）
pub fn print_clickable_link(label: &str, url: &str) {
    if supports_osc8() {
        // 使用OSC 8协议创建可点击链接
        println!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, label);
    } else {
        // 在传统终端中直接显示URL
        println!("{}", url);
    }
}

/// 检测终端是否支持Unicode字符（如表情符号）
pub fn supports_unicode() -> bool {
    // 首先检查是否在传统CMD中（最严格的检测）
    if let Ok(comspec) = std::env::var("COMSPEC") {
        if comspec.to_lowercase().contains("cmd.exe") {
            // 在传统CMD中，强制禁用Unicode
            return false;
        }
    }

    // 检查是否在Windows Terminal或现代终端中
    if let Ok(term) = std::env::var("TERM_PROGRAM") {
        if term.contains("WindowsTerminal") || term.contains("vscode") {
            return true;
        }
    }

    if let Ok(term) = std::env::var("WT_SESSION") {
        // Windows Terminal会话
        return !term.is_empty();
    }

    // 检查是否在PowerShell中（通常支持Unicode）
    if let Ok(psmodulepath) = std::env::var("PSModulePath") {
        if !psmodulepath.is_empty() {
            // 在PowerShell中，默认启用Unicode
            return true;
        }
    }

    // 默认保守策略：在不确定的情况下禁用Unicode，避免显示方框
    false
}

/// 获取兼容的图标字符
pub fn get_compatible_icon(icon_type: &str) -> &'static str {
    if supports_unicode() {
        match icon_type {
            "success" => "✅",
            "warning" => "⚠️",
            "error" => "❌",
            "info" => "ℹ️",
            "bullet" => "•",
            "arrow" => "➜",
            _ => "•",
        }
    } else {
        match icon_type {
            "success" => "[OK]",
            "warning" => "[!]",
            "error" => "[X]",
            "info" => "[i]",
            "bullet" => "*",
            "arrow" => ">",
            _ => "*",
        }
    }
}

/// 打印兼容的成功信息
pub fn print_compatible_success(msg: &str) {
    let icon = get_compatible_icon("success");
    println!("{} {}", icon, msg);
}

/// 打印兼容的警告信息
pub fn print_compatible_warning(msg: &str) {
    let icon = get_compatible_icon("warning");
    println!("{} {}", icon, msg);
}

/// 打印兼容的错误信息
pub fn print_compatible_error(msg: &str) {
    let icon = get_compatible_icon("error");
    println!("{} {}", icon, msg);
}

/// 打印兼容的信息
pub fn print_compatible_info(msg: &str) {
    let icon = get_compatible_icon("info");
    println!("{} {}", icon, msg);
}

/// 打印超紧凑程序标题（最简洁的显示）
pub fn print_super_compact_program_title(name: &str, description: &str) {
    println!();
    println!("{} {}", name, description);
    println!("{}", "─".repeat(name.len() + description.len() + 1));
}

/// 打印紧凑工具标题（单行显示）
pub fn print_compact_tool_title(title: &str) {
    println!("{}", title);
    println!("{}", "─".repeat(title.len()));
}

/// 打印紧凑菜单项（减少空行，更紧凑）
pub fn print_compact_menu_item(number: &str, name: &str, description: &str) {
    let bullet = get_compatible_icon("bullet");
    println!("{} {}. {:<10} - {}", bullet, number, name, description);
}

/// 打印简洁分隔线（更短的分隔线）
pub fn print_compact_separator() {
    println!("{}", "─".repeat(25));
}

/// 打印单行命令提示（help和back命令在同一行显示）
pub fn print_compact_command_hint() {
    println!("[help - 查看使用说明, back - 返回主菜单]");
}
