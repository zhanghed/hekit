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
    println!("{}", "-".repeat(30));
}

/// 打印章节标题
pub fn print_chapter_title(msg: &str) {
    println!("{}", msg);
    println!("{}", "-".repeat(30));
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

/// 打印现代化的程序标题（简洁风格）
pub fn print_modern_program_title(name: &str, version: &str, description: &str) {
    let title = format!("✨ {} v{} ✨", name, version);
    let max_width = title.len().max(description.len()) + 4;
    let separator = "─".repeat(max_width);

    println!();
    println!("┌{}┐", separator);
    println!("│{:^width$}│", title, width = max_width);
    if !description.is_empty() {
        println!("│{:^width$}│", "", width = max_width);
        println!("│{:^width$}│", description, width = max_width);
    }
    println!("└{}┘", separator);
    println!();
}
