use std::fs;
use std::path::PathBuf;

/// 扫描文件
pub fn scan_files(dir_path: &str, recursive: bool, _extension: Option<&String>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let path = PathBuf::from(dir_path);

    if recursive {
        scan_recursive(&path, &mut files);
    } else {
        scan_non_recursive(&path, &mut files);
    }

    files
}

/// 递归扫描
fn scan_recursive(dir_path: &PathBuf, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_dir() {
                scan_recursive(&file_path, files);
            } else if file_path.is_file() {
                files.push(file_path);
            }
        }
    }
}

/// 非递归扫描
fn scan_non_recursive(dir_path: &PathBuf, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() {
                files.push(file_path);
            }
        }
    }
}
