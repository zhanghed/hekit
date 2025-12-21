use std::fs;
use std::path::PathBuf;

/// 扫描指定目录，返回符合条件的文件路径列表
///
/// # 参数
/// - `dir_path`: 要扫描的目录路径
/// - `recursive`: 是否递归扫描子目录
/// - `extension`: 可选的文件扩展名筛选（例如 "txt"）
///
/// # 返回值
/// 符合条件的文件路径列表
pub fn scan_files(dir_path: &str, recursive: bool, extension: Option<&str>) -> Vec<PathBuf> {
    let mut files = Vec::new();

    // 检查目录是否存在
    if !PathBuf::from(dir_path).is_dir() {
        return files;
    }

    if recursive {
        // 递归扫描
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    // 递归处理子目录
                    let sub_files = scan_files(path.to_str().unwrap(), true, extension);
                    files.extend(sub_files);
                } else if path.is_file() {
                    // 检查扩展名
                    if let Some(ext) = extension {
                        if let Some(file_ext) = path.extension() {
                            if file_ext == ext {
                                files.push(path);
                            }
                        }
                    } else {
                        files.push(path);
                    }
                }
            }
        }
    } else {
        // 非递归扫描
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() {
                    // 检查扩展名
                    if let Some(ext) = extension {
                        if let Some(file_ext) = path.extension() {
                            if file_ext == ext {
                                files.push(path);
                            }
                        }
                    } else {
                        files.push(path);
                    }
                }
            }
        }
    }

    files
}
