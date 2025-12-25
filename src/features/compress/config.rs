use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;

/// 批量压缩配置
pub struct BatchCompressConfig {
    pub path: PathBuf,
    pub match_pattern: String,
    pub output_format: String,
    pub output_path: Option<PathBuf>,
    pub compression_level: u32,
    pub recursive: bool,
    pub preview: bool,
}

impl BatchCompressConfig {
    /// 创建CLAP命令
    pub fn build_clap_command() -> Command {
        Command::new("compress")
            .about("批量压缩工具")
            .arg(
                Arg::new("path")
                    .short('d')
                    .long("path")
                    .value_name("目标文件夹")
                    .help("目标文件夹（默认当前目录）")
                    .default_value("."),
            )
            .arg(
                Arg::new("match")
                    .short('m')
                    .long("match")
                    .value_name("文件模式")
                    .help("选文件（通配符 *）")
                    .required(true),
            )
            .arg(
                Arg::new("format")
                    .short('f')
                    .long("format")
                    .value_name("压缩格式")
                    .help("压缩格式（zip, tar.gz, tar.bz2）")
                    .default_value("zip"),
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("输出路径")
                    .help("输出文件路径（默认同目录）"),
            )
            .arg(
                Arg::new("level")
                    .short('l')
                    .long("level")
                    .value_name("压缩级别")
                    .help("压缩级别 1-9（默认6）")
                    .default_value("6"),
            )
            .arg(
                Arg::new("recursive")
                    .short('r')
                    .long("recursive")
                    .action(ArgAction::SetTrue)
                    .help("递归处理子目录"),
            )
            .arg(
                Arg::new("preview")
                    .short('p')
                    .long("preview")
                    .action(ArgAction::SetTrue)
                    .help("预览效果（不真压缩）"),
            )
    }

    /// 从CLAP匹配结果创建配置
    pub fn from_matches(matches: &clap::ArgMatches) -> Result<Self, anyhow::Error> {
        let path = matches
            .get_one::<String>("path")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        let match_pattern = matches
            .get_one::<String>("match")
            .ok_or_else(|| anyhow::anyhow!("必须指定文件匹配模式"))?
            .to_string();

        let output_format = matches
            .get_one::<String>("format")
            .map(|s| s.to_string())
            .unwrap_or_else(|| "zip".to_string());

        let output_path = matches
            .get_one::<String>("output")
            .map(|s| PathBuf::from(s));

        let compression_level = matches
            .get_one::<String>("level")
            .and_then(|s| s.parse().ok())
            .unwrap_or(6);

        let recursive = matches.get_flag("recursive");
        let preview = matches.get_flag("preview");

        Ok(Self {
            path,
            match_pattern,
            output_format,
            output_path,
            compression_level,
            recursive,
            preview,
        })
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if !self.path.exists() {
            return Err(anyhow::anyhow!("目标路径不存在: {}", self.path.display()));
        }

        if !self.path.is_dir() {
            return Err(anyhow::anyhow!(
                "目标路径必须是目录: {}",
                self.path.display()
            ));
        }

        if !["zip", "tar.gz", "tar.bz2"].contains(&self.output_format.as_str()) {
            return Err(anyhow::anyhow!("不支持的压缩格式: {}", self.output_format));
        }

        if self.compression_level < 1 || self.compression_level > 9 {
            return Err(anyhow::anyhow!("压缩级别必须在1-9之间"));
        }

        Ok(())
    }
}
