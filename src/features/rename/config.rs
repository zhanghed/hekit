use anyhow::{anyhow, Result};
use clap::{Arg, Command};
use std::path::PathBuf;

/// 批量重命名配置结构体
#[derive(Debug, Clone)]
pub struct BatchRenameConfig {
    pub path: PathBuf,
    pub match_pattern: String,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub replace_pattern: Option<String>,
    pub number_start: Option<usize>,
    pub extension: Option<String>,
    pub preview: bool,
    pub backup: bool,
    pub case_insensitive: bool,
}

impl BatchRenameConfig {
    /// 构建CLAP命令
    pub fn build_clap_command() -> Command {
        Command::new("rename")
            .about("批量重命名工具")
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
                Arg::new("prefix")
                    .short('p')
                    .long("prefix")
                    .value_name("前缀")
                    .help("加前缀"),
            )
            .arg(
                Arg::new("suffix")
                    .short('s')
                    .long("suffix")
                    .value_name("后缀")
                    .help("加后缀（扩展名前）"),
            )
            .arg(
                Arg::new("replace")
                    .short('r')
                    .long("replace")
                    .value_name("替换规则")
                    .help("替换文字（支持简单替换和正则替换）"),
            )
            .arg(
                Arg::new("number")
                    .short('n')
                    .long("number")
                    .value_name("起始序号")
                    .help("加序号（3位补零）"),
            )
            .arg(
                Arg::new("ext")
                    .short('e')
                    .long("ext")
                    .value_name("扩展名")
                    .help("改扩展名（空值删除）"),
            )
            .arg(
                Arg::new("preview")
                    .short('v')
                    .long("preview")
                    .help("预览效果（不真改名）")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("backup")
                    .short('b')
                    .long("backup")
                    .help("备份原文件（加.bak）")
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

    /// 从命令行参数解析配置
    pub fn from_matches(matches: &clap::ArgMatches) -> Result<Self> {
        let path = matches
            .get_one::<String>("path")
            .map(|s| PathBuf::from(s))
            .unwrap_or_else(|| PathBuf::from("."));

        let match_pattern = matches
            .get_one::<String>("match")
            .ok_or_else(|| anyhow!("缺少必要的 match 参数"))?
            .to_string();

        let prefix = matches.get_one::<String>("prefix").cloned();
        let suffix = matches.get_one::<String>("suffix").cloned();
        let replace_pattern = matches.get_one::<String>("replace").cloned();
        let extension = matches.get_one::<String>("ext").cloned();

        // 修复：序号参数解析逻辑
        let number_start = if matches.contains_id("number") {
            match matches.get_one::<String>("number") {
                Some(num_str) if !num_str.is_empty() => num_str.parse::<usize>().ok(),
                _ => Some(1), // 当用户只输入 -n 没有值时，默认使用1
            }
        } else {
            None
        };

        let preview = matches.get_flag("preview");
        let backup = matches.get_flag("backup");
        let case_insensitive = matches.get_flag("case");

        let config = Self {
            path,
            match_pattern,
            prefix,
            suffix,
            replace_pattern,
            number_start,
            extension,
            preview,
            backup,
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

        if self.match_pattern.trim().is_empty() {
            return Err(anyhow!("文件匹配模式不能为空"));
        }

        if let Some(prefix) = &self.prefix {
            if prefix.contains(std::path::MAIN_SEPARATOR) {
                return Err(anyhow!("前缀不能包含路径分隔符: {}", prefix));
            }
        }

        if let Some(suffix) = &self.suffix {
            if suffix.contains(std::path::MAIN_SEPARATOR) {
                return Err(anyhow!("后缀不能包含路径分隔符: {}", suffix));
            }
        }

        if let Some(ext) = &self.extension {
            if !ext.is_empty() && ext.contains('.') {
                return Err(anyhow!(
                    "扩展名不能包含点号，请直接输入扩展名（如 'txt' 而不是 '.txt'）"
                ));
            }
        }

        if let Some(start) = self.number_start {
            if start == 0 {
                return Err(anyhow!("序号起始值不能为0"));
            }
        }

        let has_rename_method = self.prefix.is_some()
            || self.suffix.is_some()
            || self.replace_pattern.is_some()
            || self.number_start.is_some()
            || self.extension.is_some();

        if !has_rename_method {
            return Err(anyhow!(
                "请至少指定一种重命名方式（--prefix, --suffix, --replace, --number 或 --ext）"
            ));
        }

        Ok(())
    }
}
