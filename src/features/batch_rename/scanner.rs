use std::fs;
use std::path::PathBuf;

/// 扫描指定目录下的文件
///
/// # 参数
/// - dir_path: 目录路径
/// - recursive: 是否递归扫描子目录
/// - extension: 文件扩展名过滤（可选）
///
/// # 返回值
/// 匹配的文件路径列表
pub fn scan_files(dir_path: &str, recursive: bool, extension: Option<&String>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let path = PathBuf::from(dir_path);

    // 根据递归标志选择扫描方式
    if recursive {
        scan_directory_recursive(&path, extension, &mut files);
    } else {
        scan_directory_non_recursive(&path, extension, &mut files);
    }

    files
}

/// 递归扫描目录
///
/// # 参数
/// - dir_path: 目录路径
/// - extension: 文件扩展名过滤
/// - files: 存储文件路径的向量
fn scan_directory_recursive(
    dir_path: &PathBuf,
    extension: Option<&String>,
    files: &mut Vec<PathBuf>,
) {
    // 读取目录内容
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let file_path = entry.path();

            if file_path.is_dir() {
                // 递归扫描子目录
                scan_directory_recursive(&file_path, extension, files);
            } else if file_path.is_file() {
                // 检查文件扩展名是否匹配
                if matches_extension(&file_path, extension) {
                    files.push(file_path);
                }
            }
        }
    }
}

/// 非递归扫描目录（只扫描当前目录）
///
/// # 参数
/// - dir_path: 目录路径
/// - extension: 文件扩展名过滤
/// - files: 存储文件路径的向量
fn scan_directory_non_recursive(
    dir_path: &PathBuf,
    extension: Option<&String>,
    files: &mut Vec<PathBuf>,
) {
    // 读取目录内容
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let file_path = entry.path();

            // 只处理文件，不处理目录
            if file_path.is_file() {
                // 检查文件扩展名是否匹配
                if matches_extension(&file_path, extension) {
                    files.push(file_path);
                }
            }
        }
    }
}

/// 检查文件扩展名是否匹配
///
/// # 参数
/// - file_path: 文件路径
/// - extension: 要匹配的扩展名
///
/// # 返回值
/// 如果匹配返回 true，否则返回 false
fn matches_extension(file_path: &PathBuf, extension: Option<&String>) -> bool {
    if let Some(ext) = extension {
        // 检查文件是否有扩展名
        if let Some(file_ext) = file_path.extension() {
            // 比较扩展名
            file_ext.to_string_lossy() == ext.as_str()
        } else {
            false // 文件没有扩展名
        }
    } else {
        true // 没有指定扩展名，匹配所有文件
    }
}
