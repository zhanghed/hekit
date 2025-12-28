use clap::{Arg, ArgMatches, Command};
use std::path::PathBuf;

/// 批量清理配置
#[derive(Debug, Clone)]
pub struct BatchCleanConfig {
    /// 目标目录
    pub target_dir: PathBuf,
    /// 清理模式
    pub clean_mode: CleanMode,
    /// 是否启用预览模式
    pub preview_mode: bool,
    /// 是否启用备份功能
    pub backup_enabled: bool,
    /// 备份目录
    pub backup_dir: Option<PathBuf>,
}

/// 清理模式
#[derive(Debug, Clone)]
pub enum CleanMode {
    /// 清理空文件夹
    EmptyFolders,
    /// 清理临时文件 (*.tmp, *.bak等)
    TempFiles,
    /// 清理日志文件（按时间）
    LogFiles { days_old: u32 },
    /// 安全删除（文件粉碎）
    SecureDelete,
    /// 自定义规则清理
    Custom { patterns: Vec<String> },
}

impl BatchCleanConfig {
    /// 创建新的清理配置
    pub fn new(target_dir: PathBuf, clean_mode: CleanMode) -> Self {
        Self {
            target_dir,
            clean_mode,
            preview_mode: true,
            backup_enabled: true,
            backup_dir: None,
        }
    }

    /// 构建CLAP命令
    pub fn build_clap_command() -> Command {
        Command::new("clean")
            .about("批量清理工具")
            .arg(
                Arg::new("path")
                    .short('d')
                    .long("path")
                    .value_name("目标文件夹")
                    .help("目标文件夹（默认当前目录）"),
            )
            .arg(
                Arg::new("mode")
                    .short('m')
                    .long("mode")
                    .value_name("清理模式")
                    .help("清理模式: empty(空文件夹), temp(临时文件), log(日志文件), secure(安全删除), custom(自定义)"),
            )
            .arg(
                Arg::new("days")
                    .long("days")
                    .value_name("天数")
                    .help("清理多少天前的日志文件"),
            )
            .arg(
                Arg::new("patterns")
                    .long("patterns")
                    .value_name("模式")
                    .help("自定义文件模式（用逗号分隔）"),
            )
            .arg(
                Arg::new("preview")
                    .short('v')
                    .long("preview")
                    .action(clap::ArgAction::SetTrue)
                    .help("预览模式（不实际删除）"),
            )
            .arg(
                Arg::new("backup")
                    .short('b')
                    .long("backup")
                    .action(clap::ArgAction::SetTrue)
                    .help("启用备份功能"),
            )
            .arg(
                Arg::new("backup-dir")
                    .long("backup-dir")
                    .value_name("备份目录")
                    .help("备份目录路径"),
            )
    }

    /// 从CLAP匹配结果创建配置
    pub fn from_matches(matches: &ArgMatches) -> Result<Self, crate::error::HekitError> {
        let target_dir = match matches.get_one::<String>("path") {
            Some(path) => PathBuf::from(path),
            None => std::env::current_dir().map_err(|e| {
                crate::error::HekitError::FileOperation(format!("获取当前目录失败: {}", e))
            })?,
        };

        let clean_mode = match matches.get_one::<String>("mode") {
            Some(mode) => match mode.as_str() {
                "empty" => CleanMode::EmptyFolders,
                "temp" => CleanMode::TempFiles,
                "log" => {
                    let days_old = matches
                        .get_one::<String>("days")
                        .and_then(|d| d.parse().ok())
                        .unwrap_or(7);
                    CleanMode::LogFiles { days_old }
                }
                "secure" => CleanMode::SecureDelete,
                "custom" => {
                    let patterns = matches
                        .get_one::<String>("patterns")
                        .map(|p| p.split(',').map(|s| s.trim().to_string()).collect())
                        .unwrap_or_default();
                    CleanMode::Custom { patterns }
                }
                _ => {
                    return Err(crate::error::HekitError::UserInput(
                        "无效的清理模式".to_string(),
                    ))
                }
            },
            None => {
                return Err(crate::error::HekitError::UserInput(
                    "必须指定清理模式".to_string(),
                ))
            }
        };

        let preview_mode = matches.get_flag("preview");
        let backup_enabled = matches.get_flag("backup");
        let backup_dir = matches.get_one::<String>("backup-dir").map(PathBuf::from);

        Ok(Self {
            target_dir,
            clean_mode,
            preview_mode,
            backup_enabled,
            backup_dir,
        })
    }
}
