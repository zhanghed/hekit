use clap::{Arg, ArgMatches, Command};
use std::path::PathBuf;

/// 批量重命名配置
pub struct BatchRenameConfig {
    pub pattern: String,
    pub target: String,
    pub directory: PathBuf,
    pub recursive: bool,
    pub dry_run: bool,
    pub interactive: bool,
}

impl BatchRenameConfig {
    /// 从命令行参数解析配置
    pub fn from_matches(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        let pattern = matches
            .get_one::<String>("pattern")
            .ok_or_else(|| anyhow::anyhow!("缺少模式参数"))?
            .to_string();

        let target = matches
            .get_one::<String>("target")
            .ok_or_else(|| anyhow::anyhow!("缺少目标参数"))?
            .to_string();

        let directory = matches
            .get_one::<String>("directory")
            .map(|s| PathBuf::from(s))
            .unwrap_or_else(|| PathBuf::from("."));

        let recursive = matches.get_flag("recursive");
        let dry_run = matches.get_flag("dry-run");
        let interactive = matches.get_flag("interactive");

        Ok(Self {
            pattern,
            target,
            directory,
            recursive,
            dry_run,
            interactive,
        })
    }

    /// 构建命令行参数定义
    pub fn build_clap_command() -> Command {
        Command::new("batch-rename")
            .about("批量重命名文件")
            .arg(
                Arg::new("pattern")
                    .short('p')
                    .long("pattern")
                    .value_name("PATTERN")
                    .help("要匹配的文件模式")
                    .required(true),
            )
            .arg(
                Arg::new("target")
                    .short('t')
                    .long("target")
                    .value_name("TARGET")
                    .help("目标文件名模式（支持 {n} 和 {ext}）")
                    .required(true),
            )
            .arg(
                Arg::new("directory")
                    .short('d')
                    .long("directory")
                    .value_name("DIRECTORY")
                    .help("要扫描的目录")
                    .default_value("."),
            )
            .arg(
                Arg::new("recursive")
                    .short('r')
                    .long("recursive")
                    .help("递归扫描子目录")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("dry-run")
                    .long("dry-run")
                    .help("预览模式")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("interactive")
                    .short('i')
                    .long("interactive")
                    .help("交互模式")
                    .action(clap::ArgAction::SetTrue),
            )
    }
}
