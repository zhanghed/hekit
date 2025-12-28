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

/// 打印标题
pub fn print_title(msg: &str) {
    println!("{}", msg);
}

/// 打印菜单项
pub fn print_menu_item(number: &str, description: &str) {
    println!("  {:>2}  {}", number, description);
}

/// 打印分隔线
pub fn print_separator() {
    println!("{}", "-".repeat(40));
}

/// 打印章节标题
pub fn print_chapter_title(msg: &str) {
    println!("{}", msg);
}

/// 打印简洁分隔线
pub fn print_simple_separator() {
    println!("{}", "-".repeat(30));
}

/// 打印提示信息
pub fn print_prompt(msg: &str) {
    println!("{}", msg);
}

/// 打印进度信息
pub fn print_progress(msg: &str) {
    println!("{}", msg);
}

/// 打印强调信息
pub fn print_emphasis(msg: &str) {
    println!("{}", msg);
}

/// 格式化打印成功信息
pub fn print_success_format(template: &str, args: &[(&str, &dyn std::fmt::Display)]) {
    let mut message = template.to_string();
    for (key, value) in args {
        message = message.replace(&format!("{{{}}}", key), &value.to_string());
    }
    println!("{}", message);
}

/// 打印醒目标题
pub fn print_banner_title(title: &str) {
    let separator = "=".repeat(title.len() + 4);
    println!("{}", separator);
    println!("  {}  ", title);
    println!("{}", separator);
    println!();
}

/// 打印极简程序标题（无边框装饰）
pub fn print_simple_program_title(name: &str, version: &str, description: &str) {
    println!();
    println!("{} v{}", name, version);
    if !description.is_empty() {
        println!("{}", description);
    }
    println!();
}

/// 打印精美的程序标题（带边框和装饰）
pub fn print_fancy_program_title(name: &str, version: &str, description: &str) {
    let title = format!("{} v{}", name, version);
    let max_width = title.len().max(description.len()) + 10;
    let top_border = format!("╔{}╗", "═".repeat(max_width - 2));
    let bottom_border = format!("╚{}╝", "═".repeat(max_width - 2));
    let empty_line = format!("║{:width$}║", "", width = max_width - 2);

    println!();
    println!("{}", top_border);
    println!("{}", empty_line);
    println!("║{:^width$}║", title, width = max_width - 2);
    println!("{}", empty_line);
    if !description.is_empty() {
        println!("║{:^width$}║", description, width = max_width - 2);
        println!("{}", empty_line);
    }
    println!("{}", bottom_border);
    println!();
}

/// 打印现代简约程序标题（无符号装饰，不显示版本号）
pub fn print_modern_minimal_program_title(name: &str, description: &str) {
    println!();
    println!("{}", name);
    println!("{}", "─".repeat(name.len()));

    if !description.is_empty() {
        println!("{}", description);
    }

    println!();
}

/// 打印极简大字号程序标题（无分割线，不显示版本号）
pub fn print_large_program_title(name: &str, description: &str) {
    println!();
    println!("{}", name.to_uppercase());

    if !description.is_empty() {
        println!("{}", description);
    }

    println!();
}

/// 打印简洁大号程序标题（只显示名称，无描述，使用ANSI转义码放大效果）
pub fn print_large_simple_title(name: &str, description: &str) {
    println!();
    println!("{}", name);
    if !description.is_empty() {
        println!("{}", description);
    }
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

/// 打印可点击的项目地址信息（简化版本）
pub fn print_project_info(label: &str, url: &str) {
    println!("{}:", label);
    print_clickable_link(url, url);
    println!(); // 添加空行分隔
}

/// 直接打印可点击的URL（简化版本）
pub fn print_clickable_url(url: &str) {
    print_clickable_link(url, url);
}

/// 检测当前终端类型并显示提示信息
pub fn print_terminal_info() {
    if supports_osc8() {
        println!("(当前终端支持可点击链接)");
    } else {
        println!("(当前终端不支持可点击链接，请复制链接到浏览器打开)");
    }
}

/// 打印增强版简洁程序标题（不显示版本号，明显醒目，兼容性好）
pub fn print_enhanced_program_title(name: &str, description: &str) {
    let title = format!("{}", name);
    let separator = "=".repeat(title.len() + 4);

    println!();
    println!("{}", separator);
    println!("  {}  ", title);
    println!("{}", separator);

    if !description.is_empty() {
        println!("  {}  ", description);
        println!("{}", "=".repeat(description.len() + 4));
    }

    println!();
}

/// 打印现代简约程序标题（无边框，简洁优雅）
pub fn print_clean_program_title(name: &str, description: &str) {
    println!();
    println!("{}", name);
    println!("{}", "─".repeat(name.len()));

    if !description.is_empty() {
        println!("{}", description);
    }

    println!();
}
