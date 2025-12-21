use clap::{Arg, ArgMatches, Command};
use std::path::PathBuf;

/// 批量重命名配置结构体
pub struct BatchRenameConfig {
    pub pattern: String,    // 文件匹配模式
    pub target: String,     // 目标文件名模式
    pub directory: PathBuf, // 扫描目录
    pub recursive: bool,    // 是否递归扫描
    pub dry_run: bool,      // 预览模式
    pub interactive: bool,  // 交互模式
}

impl BatchRenameConfig {
    /// 从命令行参数解析配置
    ///
    /// # 参数
    /// - matches: 命令行参数匹配结果
    ///
    /// # 返回值
    /// 解析后的配置或错误
    pub fn from_matches(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        // 获取文件匹配模式
        let pattern = matches
            .get_one::<String>("pattern")
            .ok_or_else(|| anyhow::anyhow!("缺少模式参数"))?
            .to_string();

        // 获取目标文件名模式
        let target = matches
            .get_one::<String>("target")
            .ok_or_else(|| anyhow::anyhow!("缺少目标参数"))?
            .to_string();

        // 获取扫描目录，默认为当前目录
        let directory = matches
            .get_one::<String>("directory")
            .map(|s| PathBuf::from(s))
            .unwrap_or_else(|| PathBuf::from("."));

        // 获取各种选项标志
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

    /// 构建 clap 命令行参数定义
    ///
    /// # 返回值
    /// 配置好的命令行命令
    pub fn build_clap_command() -> Command {
        Command::new("batch-rename")
            .about("批量重命名文件")
            .arg(
                Arg::new("pattern")
                    .short('p')
                    .long("pattern")
                    .value_name("PATTERN")
                    .help("要匹配的文件模式（支持通配符）")
                    .required(true),
            )
            .arg(
                Arg::new("target")
                    .short('t')
                    .long("target")
                    .value_name("TARGET")
                    .help("目标文件名模式（支持 {n} 序号和 {ext} 扩展名）")
                    .required(true),
            )
            .arg(
                Arg::new("directory")
                    .short('d')
                    .long("directory")
                    .value_name("DIRECTORY")
                    .help("要扫描的目录（默认为当前目录）")
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
                    .help("预览模式，不实际执行重命名")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("interactive")
                    .short('i')
                    .long("interactive")
                    .help("交互模式，逐个确认重命名")
                    .action(clap::ArgAction::SetTrue),
            )
    }
}
