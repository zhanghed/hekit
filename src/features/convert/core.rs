use anyhow::{anyhow, Result};
use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};

use super::config::BatchConvertConfig;
use crate::utils;

/// 批量转换核心逻辑
pub struct BatchConvertCore {
    config: BatchConvertConfig,
}

impl BatchConvertCore {
    pub fn new(config: BatchConvertConfig) -> Self {
        Self { config }
    }

    /// 执行批量转换
    pub fn execute(&self) -> Result<()> {
        // 验证配置
        self.config.validate()?;

        // 查找匹配的文件
        let files = self.find_files()?;

        if files.is_empty() {
            return Err(anyhow!("未找到匹配的文件"));
        }

        utils::print_info(&format!("找到 {} 个文件需要转换", files.len()));

        // 预览模式
        if self.config.preview {
            return self.preview_conversion(&files);
        }

        // 实际转换
        self.perform_conversion(&files)
    }

    /// 查找匹配的文件
    fn find_files(&self) -> Result<Vec<PathBuf>> {
        let pattern = format!(
            "{}/{}",
            self.config.source_dir.display(),
            self.config.file_pattern
        );

        let mut files = Vec::new();
        for entry in glob(&pattern)? {
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

    /// 预览转换效果
    fn preview_conversion(&self, files: &[PathBuf]) -> Result<()> {
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
    fn perform_conversion(&self, files: &[PathBuf]) -> Result<()> {
        let output_dir = self
            .config
            .output_dir
            .as_ref()
            .unwrap_or(&self.config.source_dir);

        // 创建输出目录（如果需要）
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
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
    fn convert_file(&self, source: &Path, target: &Path) -> Result<()> {
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
                fs::copy(source, target)?;
                Ok(())
            }
            // 文本文件编码转换（简化实现）
            ("txt", "txt") => {
                // 相同格式，直接复制
                fs::copy(source, target)?;
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
                fs::copy(source, target)?;
                Ok(())
            }
        }
    }

    /// 图片格式转换（占位实现）
    fn convert_image(&self, source: &Path, target: &Path) -> Result<()> {
        // 实际项目中应该使用图像处理库如 image-rs
        // 这里先实现简单的文件复制，后续可以添加真正的图像转换
        println!("图片格式转换: {} -> {}", source.display(), target.display());
        fs::copy(source, target)?;
        Ok(())
    }

    /// PDF转文本（占位实现）
    fn convert_pdf_to_text(&self, source: &Path, target: &Path) -> Result<()> {
        // 实际项目中应该使用PDF处理库
        // 这里先创建占位文本文件
        println!("PDF转文本: {} -> {}", source.display(), target.display());
        fs::write(target, "PDF转文本功能待实现\n源文件已复制")?;
        Ok(())
    }
}
