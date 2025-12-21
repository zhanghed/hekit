use crate::app::App;

/// 文件名生成器 - 负责根据模板生成新的文件名
pub struct FilenameGenerator;

impl FilenameGenerator {
    /// 根据模板生成新的文件名
    ///
    /// # 参数
    /// - file: 原始文件名
    /// - target_pattern: 目标文件名模式
    /// - index: 文件序号（从1开始）
    ///
    /// # 返回值
    /// 生成的新文件名
    pub fn generate_new_filename(file: &str, target_pattern: &str, index: usize) -> String {
        // 分割文件名和扩展名
        let (_name, ext) = App::split_file_path(file);

        // 替换序号占位符 {n}
        let mut result = target_pattern.replace("{n}", &index.to_string());

        // 替换扩展名占位符 {ext}
        result = result.replace("{ext}", &ext);

        // 如果目标模式中没有扩展名占位符，自动添加原扩展名
        if !result.contains('.') && !ext.is_empty() {
            result = format!("{}.{}", result, ext);
        }

        result
    }

    /// 验证文件名模式的合法性
    ///
    /// # 参数
    /// - pattern: 要验证的模式字符串
    ///
    /// # 返回值
    /// 如果模式合法返回 true，否则返回 false
    pub fn validate_pattern(pattern: &str) -> bool {
        // 检查是否包含必要的序号占位符
        pattern.contains("{n}")
    }
}
