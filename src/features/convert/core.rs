use crate::error::{HekitError, HekitResult};
use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};

use super::config::BatchConvertConfig;
use crate::utils;
use image::ImageFormat;

/// 批量转换核心逻辑
pub struct BatchConvertCore {
    pub config: BatchConvertConfig,
}

impl BatchConvertCore {
    pub fn new(config: BatchConvertConfig) -> Self {
        Self { config }
    }

    /// 查找匹配的文件
    pub fn find_files(&self) -> HekitResult<Vec<PathBuf>> {
        let pattern = format!(
            "{}/{}",
            self.config.source_dir.display(),
            self.config.file_pattern
        );

        let mut files = Vec::new();
        for entry in
            glob(&pattern).map_err(|e| HekitError::FileOperation(format!("文件匹配错误: {}", e)))?
        {
            match entry {
                Ok(path) => {
                    if path.is_file() {
                        files.push(path);
                    }
                }
                Err(e) => eprintln!("文件匹配错误: {}", e),
            }
        }

        Ok(files)
    }

    /// 执行批量转换
    pub fn execute(&self) -> HekitResult<()> {
        // 验证配置
        self.config.validate()?;

        // 查找匹配的文件
        let files = self.find_files()?;

        if files.is_empty() {
            return Err(HekitError::FileOperation("未找到匹配的文件".to_string()));
        }

        utils::print_info(&format!("找到 {} 个文件需要转换", files.len()));

        // 预览模式
        if self.config.preview {
            return self.preview_conversion(&files);
        }

        // 实际转换
        self.perform_conversion(&files)
    }

    /// 预览转换效果
    fn preview_conversion(&self, files: &[PathBuf]) -> HekitResult<()> {
        utils::print_info("转换预览");

        for (i, file) in files.iter().enumerate() {
            let target_path = self.generate_target_path(file);
            println!("{}. {} -> {}", i + 1, file.display(), target_path.display());
        }

        // 修复：改为与其他模块一致的提示信息，避免误导用户
        utils::print_info("预览完成，如需实际转换请使用完整命令：-d \"test/\" -m \"*.png\" -f png -t gif -q 90 -w");
        Ok(())
    }

    /// 执行实际转换
    fn perform_conversion(&self, files: &[PathBuf]) -> HekitResult<()> {
        let output_dir = self
            .config
            .output_dir
            .as_ref()
            .unwrap_or(&self.config.source_dir);

        // 创建输出目录（如果需要）
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)
                .map_err(|e| HekitError::FileOperation(format!("创建目录失败: {}", e)))?;
        }

        let mut success_count = 0;
        let total_count = files.len();

        for (i, file) in files.iter().enumerate() {
            let target_path = self.generate_target_path(file);

            // 检查文件是否已存在
            if target_path.exists() && !self.config.overwrite {
                println!("跳过已存在文件: {}", target_path.display());
                continue;
            }

            match self.convert_file(file, &target_path) {
                Ok(_) => {
                    println!("转换进度: {}/{} - {}", i + 1, total_count, file.display());
                    success_count += 1;
                }
                Err(e) => {
                    eprintln!("转换失败 {}: {}", file.display(), e);
                }
            }
        }

        utils::print_success(&format!(
            "转换完成: {}/{} 个文件成功",
            success_count, total_count
        ));
        Ok(())
    }

    /// 生成目标文件路径
    fn generate_target_path(&self, source_file: &Path) -> PathBuf {
        let output_dir = self
            .config
            .output_dir
            .as_ref()
            .unwrap_or(&self.config.source_dir);
        let stem = source_file.file_stem().unwrap_or_default();

        let mut target_path = output_dir.join(stem);
        target_path.set_extension(&self.config.target_format);
        target_path
    }

    /// 转换单个文件
    fn convert_file(&self, source: &Path, target: &Path) -> HekitResult<()> {
        if source == target {
            return Ok(()); // 相同文件，无需转换
        }

        // 根据文件格式选择转换方法
        match (
            self.config.source_format.to_lowercase().as_str(),
            self.config.target_format.to_lowercase().as_str(),
        ) {
            // 相同格式，直接复制
            (src, dst) if src == dst => {
                fs::copy(source, target)
                    .map_err(|e| HekitError::FileOperation(format!("文件复制失败: {}", e)))?;
                Ok(())
            }
            // 文本文件编码转换（简化实现）
            ("txt", "txt") => {
                // 相同格式，直接复制
                fs::copy(source, target)
                    .map_err(|e| HekitError::FileOperation(format!("文件复制失败: {}", e)))?;
                Ok(())
            }
            // 图片格式转换（占位实现）
            (
                "jpg" | "jpeg" | "png" | "webp" | "bmp" | "gif",
                "jpg" | "jpeg" | "png" | "webp" | "bmp" | "gif",
            ) => self.convert_image(source, target),
            // PDF转文本（占位实现）
            ("pdf", "txt") => self.convert_pdf_to_text(source, target),
            // 其他格式暂不支持实际转换，先复制文件
            _ => {
                // 这里可以添加更多格式转换逻辑
                fs::copy(source, target)
                    .map_err(|e| HekitError::FileOperation(format!("文件复制失败: {}", e)))?;
                Ok(())
            }
        }
    }

    /// 图片格式转换（占位实现）
    /// 图片格式转换（实际实现）
    fn convert_image(&self, source: &Path, target: &Path) -> HekitResult<()> {
        let _source_format = self.config.source_format.to_lowercase(); // 添加下划线前缀
        let target_format = self.config.target_format.to_lowercase();

        // 获取图像格式枚举
        let target_image_format = match target_format.as_str() {
            "jpg" | "jpeg" => ImageFormat::Jpeg,
            "png" => ImageFormat::Png,
            "webp" => ImageFormat::WebP,
            "bmp" => ImageFormat::Bmp,
            "gif" => ImageFormat::Gif,
            _ => {
                // 不支持的格式，使用默认复制
                fs::copy(source, target)
                    .map_err(|e| HekitError::FileOperation(format!("文件复制失败: {}", e)))?;
                return Ok(());
            }
        };

        // 打开并转换图像
        let img = image::open(source).map_err(|e| {
            HekitError::FileOperation(format!("无法打开图像文件 {}: {}", source.display(), e))
        })?;

        // 根据质量设置调整图像
        let mut output_img = img;
        if let Some(quality) = self.config.quality {
            // 简单的质量调整（实际项目中可以更复杂）
            if quality < 80 {
                output_img = output_img.resize(
                    output_img.width() * quality as u32 / 100,
                    output_img.height() * quality as u32 / 100,
                    image::imageops::FilterType::Lanczos3,
                );
            }
        }

        // 保存图像
        output_img
            .save_with_format(target, target_image_format)
            .map_err(|e| {
                HekitError::FileOperation(format!("无法保存图像文件 {}: {}", target.display(), e))
            })?;
        println!(
            "✓ 图片转换完成: {} -> {}",
            source.display(),
            target.display()
        );
        Ok(())
    }

    /// PDF转文本（改进实现）
    fn convert_pdf_to_text(&self, source: &Path, target: &Path) -> HekitResult<()> {
        // 检查文件是否为PDF
        if let Some(ext) = source.extension() {
            if ext.to_string_lossy().to_lowercase() != "pdf" {
                return Err(HekitError::FileOperation("源文件不是PDF格式".to_string()));
            }
        }

        // 创建包含基本信息的文本文件
        let metadata = fs::metadata(source)
            .map_err(|e| HekitError::FileOperation(format!("获取文件元数据失败: {}", e)))?;
        let file_size = metadata.len();
        let modified = metadata
            .modified()
            .map_err(|e| HekitError::FileOperation(format!("获取文件修改时间失败: {}", e)))?;

        let content = format!(
            "PDF文件: {}\n文件大小: {} 字节\n修改时间: {:?}\n\nPDF转文本功能需要额外的PDF处理库支持。\n建议使用专门的PDF工具进行转换。",
            source.display(),
            file_size,
            modified
        );

        fs::write(target, content)
            .map_err(|e| HekitError::FileOperation(format!("写入文件失败: {}", e)))?;
        println!(
            "✓ PDF信息提取完成: {} -> {}",
            source.display(),
            target.display()
        );
        Ok(())
    }
}
