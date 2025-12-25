use anyhow::Result;
use std::io::{self, Write};

/// 获取用户输入的工具函数
pub fn get_user_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

/// 打印错误信息
pub fn print_error(msg: &str) {
    eprintln!("错误: {}", msg);
}

/// 打印成功信息
pub fn print_success(msg: &str) {
    println!("成功: {}", msg);
}

/// 打印信息
pub fn print_info(msg: &str) {
    println!("信息: {}", msg);
}

/// 格式化打印成功信息（使用命名参数）
pub fn print_success_format(template: &str, args: &[(&str, &dyn std::fmt::Display)]) {
    let mut message = template.to_string();
    for (key, value) in args {
        message = message.replace(&format!("{{{}}}", key), &value.to_string());
    }
    println!("成功: {}", message);
}

/// 格式化打印错误信息（使用命名参数）
pub fn print_error_format(template: &str, args: &[(&str, &dyn std::fmt::Display)]) {
    let mut message = template.to_string();
    for (key, value) in args {
        message = message.replace(&format!("{{{}}}", key), &value.to_string());
    }
    eprintln!("错误: {}", message);
}
