use anyhow::{anyhow, Result};
use clap::{Arg, Command};
use std::path::PathBuf;

/// 批量搜索配置结构体
#[derive(Debug, Clone)]
pub struct BatchSearchConfig {
    pub path: PathBuf,
    pub name_pattern: String,
    pub file_type: Option<String>,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub recursive: bool,
    pub case_insensitive: bool,
}

impl BatchSearchConfig {
    /// 构建CLAP命令
    pub fn build_clap_command() -> Command {
        Command::new("search")
            .about("批量搜索工具")
            .arg(
                Arg::new("path")
                    .short('d')
                    .long("path")
                    .value_name("搜索路径")
                    .help("搜索路径（默认当前目录）")
                    .default_value("."),
            )
            .arg(
                Arg::new("name")
                    .short('n')
                    .long("name")
                    .value_name("文件名模式")
                    .help("文件名匹配模式（支持通配符 *）")
                    .default_value("*"),
            )
            .arg(
                Arg::new("type")
                    .short('t')
                    .long("type")
                    .value_name("文件类型")
                    .help("按文件类型筛选（如 txt, jpg, pdf）"),
            )
            .arg(
                Arg::new("min-size")
                    .long("min-size")
                    .value_name("最小大小")
                    .help("最小文件大小（字节）"),
            )
            .arg(
                Arg::new("max-size")
                    .long("max-size")
                    .value_name("最大大小")
                    .help("最大文件大小（字节）"),
            )
            .arg(
                Arg::new("recursive")
                    .short('r')
                    .long("recursive")
                    .help("递归搜索子目录")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("case")
                    .short('c')
                    .long("case")
                    .help("不区分大小写匹配")
                    .action(clap::ArgAction::SetTrue),
            )
    }

    /// 修复Windows路径中的反斜杠问题
    fn fix_windows_path(path: &str) -> String {
        // 处理单反斜杠路径格式 C:\
        if path.ends_with('\\') && !path.ends_with("\\\\") {
            // 对于以单反斜杠结尾的路径，直接返回原路径
            // Windows路径系统会自动处理单反斜杠
            path.to_string()
        } else {
            path.to_string()
        }
    }

    /// 从命令行参数解析配置
    pub fn from_matches(matches: &clap::ArgMatches) -> Result<Self> {
        let raw_path = matches
            .get_one::<String>("path")
            .map(|s| s.as_str())
            .unwrap_or(".");

        // 修复Windows路径问题
        let fixed_path = Self::fix_windows_path(raw_path);
        let path = PathBuf::from(&fixed_path);

        let name_pattern = matches
            .get_one::<String>("name")
            .map(|s| s.to_string())
            .unwrap_or_else(|| "*".to_string());

        let file_type = matches.get_one::<String>("type").cloned();
        let recursive = matches.get_flag("recursive");
        let case_insensitive = matches.get_flag("case");

        // 解析文件大小参数
        let min_size = matches
            .get_one::<String>("min-size")
            .and_then(|s| s.parse::<u64>().ok());
        let max_size = matches
            .get_one::<String>("max-size")
            .and_then(|s| s.parse::<u64>().ok());

        let config = Self {
            path,
            name_pattern,
            file_type,
            min_size,
            max_size,
            recursive,
            case_insensitive,
        };

        config.validate()?;
        Ok(config)
    }

    /// 验证配置参数
    pub fn validate(&self) -> Result<()> {
        if !self.path.exists() {
            return Err(anyhow!("目录不存在: {}", self.path.display()));
        }

        if !self.path.is_dir() {
            return Err(anyhow!("路径不是目录: {}", self.path.display()));
        }

        if self.name_pattern.trim().is_empty() {
            return Err(anyhow!("文件名匹配模式不能为空"));
        }

        if let (Some(min), Some(max)) = (self.min_size, self.max_size) {
            if min > max {
                return Err(anyhow!("最小文件大小不能大于最大文件大小"));
            }
        }

        Ok(())
    }
}
