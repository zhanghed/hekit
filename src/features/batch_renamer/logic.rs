use anyhow::Result;
use std::fs;

use crate::core::scanner;
use crate::core::utils;

/// 批量重命名配置
pub struct BatchRenameConfig {
    pub dir_path: String,
    pub recursive: bool,
    pub extension: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub replace: Option<(String, String)>,
    pub serial_number: bool,
    pub start_number: u32,
    pub new_extension: Option<String>,
    pub dry_run: bool,
}

/// 运行批量重命名
pub fn run(config: &BatchRenameConfig) -> Result<()> {
    // 扫描文件
    let files = scanner::scan_files(
        &config.dir_path,
        config.recursive,
        config.extension.as_deref(),
    );

    if files.is_empty() {
        utils::print_error("没有找到符合条件的文件");
        return Ok(());
    }

    // 处理每个文件
    let mut count = 0;
    for (index, file_path) in files.iter().enumerate() {
        let (parent_dir, name_without_ext, ext) = utils::split_file_path(file_path);

        // 生成新文件名
        let mut new_name = name_without_ext.clone();

        // 应用前缀
        if let Some(prefix) = &config.prefix {
            new_name = format!("{}{}", prefix, new_name);
        }

        // 应用后缀
        if let Some(suffix) = &config.suffix {
            new_name = format!("{}{}", new_name, suffix);
        }

        // 应用替换
        if let Some((from, to)) = &config.replace {
            new_name = new_name.replace(from, to);
        }

        // 应用序号
        if config.serial_number {
            let serial = config.start_number + index as u32;
            new_name = format!("{}_{}", new_name, serial);
        }

        // 应用新扩展名
        let final_ext = config.new_extension.as_ref().or(ext.as_ref());

        // 构建完整的新文件名
        let full_new_name = if let Some(ext) = final_ext {
            format!("{}.{}", new_name, ext)
        } else {
            new_name
        };

        // 构建新路径
        let new_path = parent_dir.join(full_new_name);

        // 输出预览信息
        utils::print_preview(file_path.to_str().unwrap(), new_path.to_str().unwrap());

        // 执行重命名（如果不是干运行）
        if !config.dry_run {
            fs::rename(file_path, &new_path)?;
            count += 1;
        }
    }

    // 输出结果
    if config.dry_run {
        utils::print_success(&format!("干运行完成，共预览 {} 个文件", files.len()));
    } else {
        utils::print_success(&format!("重命名完成，共处理 {} 个文件", count));
    }

    Ok(())
}
