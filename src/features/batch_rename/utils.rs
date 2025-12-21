/// 显示批量重命名使用说明
pub fn show_batch_rename_usage() {
    println!("批量重命名使用说明:");
    println!("用法: batch-rename -p <模式> -t <目标> [选项]");
    println!("");
    println!("参数:");
    println!("  -p, --pattern <PATTERN>    要匹配的文件模式（支持通配符）");
    println!("  -t, --target <TARGET>      目标文件名模式");
    println!("  -d, --directory <DIRECTORY> 要扫描的目录（默认为当前目录）");
    println!("  -r, --recursive             递归扫描子目录");
    println!("      --dry-run               预览模式，不实际执行重命名");
    println!("  -i, --interactive          交互模式，逐个确认重命名");
    println!("");
    println!("目标模式支持以下占位符:");
    println!("  {{n}}   文件序号（从1开始）");
    println!("  {{ext}} 文件扩展名");
    println!("");
    println!("示例:");
    println!("  batch-rename -p \"*.txt\" -t \"document_{{n}}\"");
    println!("  batch-rename -p \"image*\" -t \"photo_{{n}}.{{ext}}\" -r -i");
    println!("");
}

/// 显示交互菜单
pub fn show_interactive_menu() {
    println!("=== HEKIT 文件工具 ===");
    println!("1. 批量重命名文件");
    println!("2. 退出");
    println!("======================");
}

/// 验证文件名的合法性
///
/// # 参数
/// - filename: 要验证的文件名
///
/// # 返回值
/// 如果文件名合法返回 true，否则返回 false
pub fn validate_filename(filename: &str) -> bool {
    // 定义非法字符列表
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

    // 检查文件名是否包含非法字符
    !filename.chars().any(|c| invalid_chars.contains(&c))
}

/// 生成预览列表
///
/// # 参数
/// - files: 原始文件列表
/// - new_names: 新文件名列表
///
/// # 返回值
/// 格式化后的预览字符串列表
pub fn generate_preview_list(files: &[String], new_names: &[String]) -> Vec<String> {
    files
        .iter()
        .zip(new_names.iter())
        .enumerate()
        .map(|(i, (old, new))| format!("{}. {} -> {}", i + 1, old, new))
        .collect()
}
