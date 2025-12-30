use anyhow::Result;
use clap::{Arg, Command};

/// 系统信息工具配置结构体
#[derive(Debug, Clone)]
pub struct SysInfoConfig {
    pub show_basic: bool,
    pub show_cpu: bool,
    pub show_memory: bool,
    pub show_disk: bool,
    pub show_network: bool,
    pub show_processes: bool,
    pub refresh: bool,
}

impl SysInfoConfig {
    /// 构建CLAP命令
    pub fn build_clap_command() -> Command {
        Command::new("sysinfo")
            .about("显示系统信息")
            .arg(
                Arg::new("basic")
                    .short('b')
                    .long("basic")
                    .help("显示基本系统信息")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("cpu")
                    .short('c')
                    .long("cpu")
                    .help("显示CPU信息")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("memory")
                    .short('m')
                    .long("memory")
                    .help("显示内存信息")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("disk")
                    .short('d')
                    .long("disk")
                    .help("显示磁盘信息")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("network")
                    .short('n')
                    .long("network")
                    .help("显示网络信息")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("processes")
                    .short('p')
                    .long("processes")
                    .help("显示进程信息")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("refresh")
                    .short('r')
                    .long("refresh")
                    .help("刷新系统信息")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("all")
                    .short('a')
                    .long("all")
                    .help("显示所有信息")
                    .action(clap::ArgAction::SetTrue),
            )
    }

    /// 从CLAP匹配结果创建配置
    pub fn from_matches(matches: &clap::ArgMatches) -> Result<Self> {
        let show_all = matches.get_flag("all");

        Ok(SysInfoConfig {
            show_basic: show_all || matches.get_flag("basic"),
            show_cpu: show_all || matches.get_flag("cpu"),
            show_memory: show_all || matches.get_flag("memory"),
            show_disk: show_all || matches.get_flag("disk"),
            show_network: show_all || matches.get_flag("network"),
            show_processes: show_all || matches.get_flag("processes"),
            refresh: matches.get_flag("refresh"),
        })
    }

    /// 验证配置
    pub fn validate(&self) -> Result<()> {
        // 如果没有选择任何显示选项，默认显示基本信息
        if !self.show_basic
            && !self.show_cpu
            && !self.show_memory
            && !self.show_disk
            && !self.show_network
            && !self.show_processes
        {
            // 默认显示基本信息
            Ok(())
        } else {
            Ok(())
        }
    }
}
