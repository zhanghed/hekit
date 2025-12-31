use crate::error::{HekitError, HekitResult};
use crate::features::compress::config::BatchCompressConfig;
use crate::hekit_error; // 添加宏导入
use crate::progress::ProgressManager;
use flate2::write::GzEncoder;
use glob::glob;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tar::{Builder, Header};

/// 批量压缩核心逻辑
pub struct BatchCompressCore {
    pub config: BatchCompressConfig,
}

impl BatchCompressCore {
    /// 创建新的批量压缩实例
    pub fn new(config: BatchCompressConfig) -> Self {
        Self { config }
    }

    /// 执行批量压缩
    pub fn execute(&self) -> HekitResult<()> {
        let files = self.scan_files()?;
        if files.is_empty() {
            return hekit_error!(Compression, "没有找到匹配的文件");
        }

        if self.config.preview {
            self.execute_preview(&files)
        } else {
            self.execute_compression(&files)
        }
    }

    /// 扫描匹配的文件
    pub fn scan_files(&self) -> HekitResult<Vec<PathBuf>> {
        let pattern = self.config.path.join(&self.config.match_pattern);
        let pattern = pattern.to_string_lossy().to_string();

        let mut files = Vec::new();

        match glob(&pattern) {
            Ok(paths) => {
                for entry in paths.flatten() {
                    if entry.is_file() {
                        files.push(entry);
                    }
                }
                files.sort();
                Ok(files)
            }
            Err(e) => hekit_error!(FileOperation, &format!("文件扫描失败: {}", e)),
        }
    }

    /// 执行预览模式
    fn execute_preview(&self, files: &[PathBuf]) -> HekitResult<()> {
        println!("预览压缩结果:");

        for (i, file_path) in files.iter().enumerate() {
            let output_path = self.generate_output_path(file_path, i + 1, files.len())?;
            println!("  {} → {}", file_path.display(), output_path.display());
        }

        println!("总计: {} 个文件", files.len());
        Ok(())
    }

    /// 执行实际压缩
    fn execute_compression(&self, files: &[PathBuf]) -> HekitResult<()> {
        println!("开始批量压缩...");
        let progress = ProgressManager::new(files.len() as u64, "批量压缩中...");
        let mut success_count = 0;
        let mut error_count = 0;

        for (i, file_path) in files.iter().enumerate() {
            progress.set_message(&format!("压缩: {}", file_path.display()));

            let output_path = self.generate_output_path(file_path, i + 1, files.len())?;

            match self.compress_file(file_path, &output_path) {
                Ok(_) => {
                    println!("✓ {} → {}", file_path.display(), output_path.display());
                    success_count += 1;
                }
                Err(e) => {
                    eprintln!("✗ {} 压缩失败: {}", file_path.display(), e);
                    error_count += 1;
                }
            }

            progress.inc(1);
        }

        progress.finish_with_message(&format!(
            "完成: 成功 {} 个, 失败 {} 个",
            success_count, error_count
        ));

        if error_count > 0 {
            hekit_error!(Compression, "部分文件压缩失败")
        } else {
            Ok(())
        }
    }

    /// 生成输出文件路径
    fn generate_output_path(
        &self,
        file_path: &Path,
        index: usize,
        total_files: usize,
    ) -> HekitResult<PathBuf> {
        let file_stem = file_path
            .file_stem()
            .ok_or_else(|| HekitError::Compression("无法获取文件名".to_string()))?
            .to_string_lossy();
        let extension = match self.config.output_format.as_str() {
            "zip" => "zip",
            "tar.gz" => "tar.gz",
            "tar.bz2" => "tar.bz2",
            _ => "zip",
        };

        let output_filename = if total_files > 1 {
            format!("{}_{}.{}", file_stem, index, extension)
        } else {
            format!("{}.{}", file_stem, extension)
        };

        if let Some(output_dir) = &self.config.output_path {
            Ok(output_dir.join(output_filename))
        } else {
            Ok(file_path
                .parent()
                .unwrap_or(Path::new("."))
                .join(output_filename))
        }
    }

    /// 压缩单个文件
    fn compress_file(&self, input_path: &Path, output_path: &Path) -> HekitResult<()> {
        match self.config.output_format.as_str() {
            "zip" => self.compress_zip(input_path, output_path),
            "tar.gz" => self.compress_tar_gz(input_path, output_path),
            "tar.bz2" => self.compress_tar_bz2(input_path, output_path),
            _ => self.compress_zip(input_path, output_path),
        }
    }

    /// 压缩为ZIP格式
    fn compress_zip(&self, input_path: &Path, output_path: &Path) -> HekitResult<()> {
        let file = File::create(output_path)
            .map_err(|e| HekitError::Compression(format!("创建ZIP文件失败: {}", e)))?;
        let mut zip = zip::ZipWriter::new(file);

        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .compression_level(Some(self.config.compression_level as i32));

        let file_name = input_path.file_name().unwrap_or_default().to_string_lossy();

        zip.start_file(file_name.as_ref(), options)
            .map_err(|e| HekitError::Compression(format!("ZIP文件写入失败: {}", e)))?;

        let mut input_file = File::open(input_path)
            .map_err(|e| HekitError::Compression(format!("打开输入文件失败: {}", e)))?;
        io::copy(&mut input_file, &mut zip)
            .map_err(|e| HekitError::Compression(format!("文件复制失败: {}", e)))?;

        zip.finish()
            .map_err(|e| HekitError::Compression(format!("完成ZIP文件失败: {}", e)))?;
        Ok(())
    }

    /// 压缩为tar.gz格式
    fn compress_tar_gz(&self, input_path: &Path, output_path: &Path) -> HekitResult<()> {
        let tar_gz_file = File::create(output_path)
            .map_err(|e| HekitError::Compression(format!("创建tar.gz文件失败: {}", e)))?;
        let encoder = GzEncoder::new(
            tar_gz_file,
            flate2::Compression::new(self.config.compression_level),
        );
        let mut tar = Builder::new(encoder);

        self.add_file_to_tar(input_path, &mut tar)?;
        tar.finish()
            .map_err(|e| HekitError::Compression(format!("完成tar.gz文件失败: {}", e)))?;

        Ok(())
    }

    /// 压缩为tar.bz2格式
    fn compress_tar_bz2(&self, input_path: &Path, output_path: &Path) -> HekitResult<()> {
        let tar_bz2_file = File::create(output_path)
            .map_err(|e| HekitError::Compression(format!("创建tar.bz2文件失败: {}", e)))?;
        let encoder = bzip2::write::BzEncoder::new(
            tar_bz2_file,
            bzip2::Compression::new(self.config.compression_level as u32),
        );
        let mut tar = Builder::new(encoder);

        self.add_file_to_tar(input_path, &mut tar)?;
        tar.finish()
            .map_err(|e| HekitError::Compression(format!("完成tar.bz2文件失败: {}", e)))?;

        Ok(())
    }

    /// 添加文件到tar包
    fn add_file_to_tar<T: Write>(
        &self,
        input_path: &Path,
        tar: &mut Builder<T>,
    ) -> HekitResult<()> {
        let mut file = File::open(input_path)
            .map_err(|e| HekitError::Compression(format!("打开输入文件失败: {}", e)))?;
        let metadata = file
            .metadata()
            .map_err(|e| HekitError::Compression(format!("获取文件元数据失败: {}", e)))?;

        let mut header = Header::new_gnu();
        header
            .set_path(input_path.file_name().unwrap_or_default())
            .map_err(|e| HekitError::Compression(format!("设置tar头路径失败: {}", e)))?;
        header.set_size(metadata.len());
        header.set_mode(0o644);
        header.set_cksum();

        tar.append(&header, &mut file)
            .map_err(|e| HekitError::Compression(format!("添加文件到tar包失败: {}", e)))?;

        Ok(())
    }
}
