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
}

impl BatchRenameConfig {
    /// 构建CLI命令定义
    pub fn build_clap_command() -> Command {
        Command::new("rename")
            .about("文件重命名工具 - 简洁实用版")
            .arg(
                Arg::new("path")
                    .short('d') // 改为 -d (directory)
                    .long("path")
                    .value_name("目标文件夹")
                    .help("目标文件夹（默认当前目录）")
                    .default_value("."),
            )
            .arg(
                Arg::new("prefix")
                    .short('p') // 保持 -p (prefix)
                    .long("prefix")
                    .value_name("前缀")
                    .help("加前缀（如给文件统一加 \"2025_\")"),
            )
            .arg(
                Arg::new("match")
                    .short('m')
                    .long("match")
                    .value_name("文件模式")
                    .help("选文件（通配符 *，如 *.jpg 或 笔记*）")
                    .required(true),
            )
            .arg(
                Arg::new("suffix")
                    .short('s')
                    .long("suffix")
                    .value_name("后缀")
                    .help("加后缀（扩展名前，如加 \"_备份\")"),
            )
            .arg(
                Arg::new("replace")
                    .short('r')
                    .long("replace")
                    .value_name("替换规则")
                    .help("替换文字（支持简单替换/正则，正则加/包裹）"),
            )
            .arg(
                Arg::new("number")
                    .short('n')
                    .long("number")
                    .value_name("起始序号")
                    .help("加序号（默认1开始，3位补零）"),
            )
            .arg(
                Arg::new("ext")
                    .short('e')
                    .long("ext")
                    .value_name("扩展名")
                    .help("改扩展名（如txt转md，空值删扩展名）"),
            )
            .arg(
                Arg::new("preview")
                    .short('v')
                    .long("preview")
                    .help("预览效果（不真改名，避免出错）")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("backup")
                    .short('b')
                    .long("backup")
                    .help("备份原文件（自动加.bak后缀）")
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

        // 解析序号
        let number_start = match matches.get_one::<String>("number") {
            Some(num_str) => {
                if num_str.is_empty() {
                    Some(1)
                } else {
                    num_str.parse::<usize>().ok()
                }
            }
            None => None,
        };

        let preview = matches.get_flag("preview");
        let backup = matches.get_flag("backup");

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
        };

        config.validate()?;
        Ok(config)
    }

    /// 验证配置参数
    pub fn validate(&self) -> Result<()> {
        // 检查目录是否存在
        if !self.path.exists() {
            return Err(anyhow!("目录不存在: {}", self.path.display()));
        }

        if !self.path.is_dir() {
            return Err(anyhow!("路径不是目录: {}", self.path.display()));
        }

        // 验证至少指定了一种重命名方式
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
