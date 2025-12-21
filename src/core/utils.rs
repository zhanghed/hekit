use std::path::Path;
use std::path::PathBuf;

/// 拆分文件路径为父目录、文件名（无后缀）和后缀
///
/// # 参数
/// - `file_path`: 完整的文件路径
///
/// # 返回值
/// (父目录路径, 文件名（无后缀）, 文件后缀)
pub fn split_file_path(file_path: &Path) -> (PathBuf, String, Option<String>) {
    // 获取父目录
    let parent_dir = file_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or(PathBuf::from("."));

    // 获取文件名
    let file_name = file_path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("")
        .to_string();

    // 拆分文件名和后缀
    let (name_without_ext, ext) = if let Some(dot_pos) = file_name.rfind('.') {
        let name = file_name[..dot_pos].to_string();
        let ext = file_name[dot_pos + 1..].to_string();
        (name, Some(ext))
    } else {
        (file_name, None)
    };

    (parent_dir, name_without_ext, ext)
}

/// 打印成功信息
pub fn print_success(message: &str) {
    println!("\x1b[32m✓ {}\x1b[0m", message);
}

/// 打印错误信息
pub fn print_error(message: &str) {
    println!("\x1b[31m✗ {}\x1b[0m", message);
}

/// 打印预览信息
pub fn print_preview(old_name: &str, new_name: &str) {
    println!(
        "\x1b[33m→ {}\x1b[0m → \x1b[36m{}\x1b[0m",
        old_name, new_name
    );
}
