/// 文件名生成器
pub struct FilenameGenerator;

impl FilenameGenerator {
    /// 生成新的文件名
    pub fn generate_new_filename(file: &str, target_pattern: &str, index: usize) -> String {
        // 直接实现文件路径分割，避免依赖App模块
        let (_name, ext) = Self::split_file_path(file); // 添加下划线表示未使用

        let mut result = target_pattern.replace("{n}", &index.to_string());
        result = result.replace("{ext}", &ext);

        if !result.contains('.') && !ext.is_empty() {
            result = format!("{}.{}", result, ext);
        }

        result
    }

    /// 分割文件路径为文件名和扩展名
    fn split_file_path(path: &str) -> (String, String) {
        if let Some(pos) = path.rfind('.') {
            let name = path[..pos].to_string();
            let ext = path[pos + 1..].to_string();
            (name, ext)
        } else {
            (path.to_string(), String::new())
        }
    }
}
