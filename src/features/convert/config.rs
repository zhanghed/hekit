use anyhow::{anyhow, Result};
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;

/// 批量转换配置
#[derive(Debug)]
pub struct BatchConvertConfig {
    pub source_dir: PathBuf,
    pub file_pattern: String,
    pub source_format: String,
    pub target_format: String,
    pub output_dir: Option<PathBuf>,
    pub quality: Option<u8>,
    pub resize: Option<(u32, u32)>,
    pub preview: bool,
    pub overwrite: bool,
}

impl BatchConvertConfig {
    /// 构建CLAP命令
    pub fn build_clap_command() -> Command {
        Command::new("convert")
            .about("批量文件格式转换工具")
            .arg(
                Arg::new("path")
                    .short('d')
                    .long("path")
                    .value_name("目标文件夹")
                    .help("源文件所在文件夹（默认当前目录）")
                    .default_value("."),
            )
            .arg(
                Arg::new("pattern")
                    .short('m')
                    .long("pattern")
                    .value_name("文件模式")
                    .help("文件匹配模式（通配符 *）")
                    .required(true),
            )
            .arg(
                Arg::new("from")
                    .short('f')
                    .long("from")
                    .value_name("源格式")
                    .help("源文件格式（如：jpg, png, pdf）")
                    .required(true),
            )
            .arg(
                Arg::new("to")
                    .short('t')
                    .long("to")
                    .value_name("目标格式")
                    .help("目标文件格式（如：png, webp, txt）")
                    .required(true),
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("输出目录")
                    .help("输出目录（默认源目录）"),
            )
            .arg(
                Arg::new("quality")
                    .short('q')
                    .long("quality")
                    .value_name("质量")
                    .help("输出质量（1-100，仅图片格式有效）"),
            )
            .arg(
                Arg::new("resize")
                    .short('r')
                    .long("resize")
                    .value_name("尺寸")
                    .help("调整尺寸（格式：宽x高，如：800x600）"),
            )
            .arg(
                Arg::new("preview")
                    .short('v')
                    .long("preview")
                    .action(ArgAction::SetTrue)
                    .help("预览模式（不实际转换）"),
            )
            .arg(
                Arg::new("overwrite")
                    .short('w')
                    .long("overwrite")
                    .action(ArgAction::SetTrue)
                    .help("覆盖已存在文件"),
            )
    }

    /// 从CLAP匹配结果创建配置
    pub fn from_matches(matches: &ArgMatches) -> Result<Self> {
        let source_dir = PathBuf::from(matches.get_one::<String>("path").unwrap());
        let file_pattern = matches.get_one::<String>("pattern").unwrap().clone();
        let source_format = matches.get_one::<String>("from").unwrap().clone();
        let target_format = matches.get_one::<String>("to").unwrap().clone();

        let output_dir = matches
            .get_one::<String>("output")
            .map(|s| PathBuf::from(s));

        let quality = matches
            .get_one::<String>("quality")
            .and_then(|q| q.parse::<u8>().ok())
            .map(|q| q.clamp(1, 100));

        let resize = matches.get_one::<String>("resize").and_then(|r| {
            let parts: Vec<&str> = r.split('x').collect();
            if parts.len() == 2 {
                if let (Ok(width), Ok(height)) = (parts[0].parse(), parts[1].parse()) {
                    Some((width, height))
                } else {
                    None
                }
            } else {
                None
            }
        });

        let preview = matches.get_flag("preview");
        let overwrite = matches.get_flag("overwrite");

        // 验证格式支持
        let supported_formats = ["jpg", "jpeg", "png", "webp", "bmp", "gif", "pdf", "txt"];
        if !supported_formats.contains(&source_format.to_lowercase().as_str()) {
            return Err(anyhow!("不支持的源格式: {}", source_format));
        }
        if !supported_formats.contains(&target_format.to_lowercase().as_str()) {
            return Err(anyhow!("不支持的目标格式: {}", target_format));
        }

        let config = Self {
            source_dir,
            file_pattern,
            source_format,
            target_format,
            output_dir,
            quality,
            resize,
            preview,
            overwrite,
        };

        // 调用验证方法
        config.validate()?;
        Ok(config)
    }

    /// 验证配置参数
    pub fn validate(&self) -> Result<()> {
        if !self.source_dir.exists() {
            return Err(anyhow!("源目录不存在: {}", self.source_dir.display()));
        }

        if !self.source_dir.is_dir() {
            return Err(anyhow!("源路径不是目录: {}", self.source_dir.display()));
        }

        if self.file_pattern.trim().is_empty() {
            return Err(anyhow!("文件匹配模式不能为空"));
        }

        if self.source_format == self.target_format {
            return Err(anyhow!("源格式和目标格式不能相同"));
        }

        // 验证输出目录
        if let Some(output_dir) = &self.output_dir {
            if output_dir.exists() && !output_dir.is_dir() {
                return Err(anyhow!("输出路径不是目录: {}", output_dir.display()));
            }
        }

        // 验证质量参数
        if let Some(quality) = self.quality {
            if quality < 1 || quality > 100 {
                return Err(anyhow!("质量参数必须在1-100之间"));
            }
        }

        // 验证尺寸参数
        if let Some((width, height)) = self.resize {
            if width == 0 || height == 0 {
                return Err(anyhow!("尺寸参数不能为0"));
            }
        }

        Ok(())
    }
}
