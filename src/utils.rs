use anyhow::Result;
use std::io::{self, Write};

/// 获取用户输入的工具函数（极简格式）
pub fn get_user_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

/// 打印错误信息（极简格式，无前缀）
pub fn print_error(msg: &str) {
    println!("{}", msg);
}

/// 打印成功信息（极简格式，无前缀）
pub fn print_success(msg: &str) {
    println!("{}", msg);
}

/// 打印信息（极简格式，无前缀）
pub fn print_info(msg: &str) {
    println!("{}", msg);
}

/// 打印警告信息（极简格式，无前缀）
pub fn print_warning(msg: &str) {
    println!("{}", msg);
}

/// 打印标题（极简格式，无前缀）
pub fn print_title(msg: &str) {
    println!("{}", msg);
}

/// 打印菜单项（极简格式）
pub fn print_menu_item(number: &str, description: &str) {
    println!("  {:>2}  {}", number, description);
}

/// 打印分隔线（统一风格）
pub fn print_separator() {
    println!("{}", "-".repeat(30));
}

/// 打印章节标题（极简格式，无前缀）
pub fn print_chapter_title(msg: &str) {
    println!("{}", msg);
    println!("{}", "-".repeat(30));
}

/// 打印简洁分隔线（统一风格）
pub fn print_simple_separator() {
    println!("{}", "-".repeat(30));
}

/// 打印提示信息（极简格式，无前缀）
pub fn print_prompt(msg: &str) {
    println!("{}", msg);
}

/// 打印进度信息（极简格式，无前缀）
pub fn print_progress(msg: &str) {
    println!("{}", msg);
}

/// 打印强调信息（极简格式，无前缀）
pub fn print_emphasis(msg: &str) {
    println!("{}", msg);
}

/// 格式化打印成功信息（极简格式，无前缀）
pub fn print_success_format(template: &str, args: &[(&str, &dyn std::fmt::Display)]) {
    let mut message = template.to_string();
    for (key, value) in args {
        message = message.replace(&format!("{{{}}}", key), &value.to_string());
    }
    println!("{}", message);
}
