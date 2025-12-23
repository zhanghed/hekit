pub mod config;
pub mod executor;
pub mod generator;
pub mod scanner;

/// 显示批量重命名使用说明
pub fn show_usage() {
    println!("批量重命名使用说明:");
    println!("用法: batch-rename -p <模式> -t <目标> [选项]");
    println!("");
    println!("参数:");
    println!("  -p, --pattern <PATTERN>    要匹配的文件模式");
    println!("  -t, --target <TARGET>      目标文件名模式");
    println!("  -d, --directory <DIRECTORY> 要扫描的目录");
    println!("  -r, --recursive             递归扫描子目录");
    println!("      --dry-run               预览模式");
    println!("  -i, --interactive          交互模式");
    println!("");
    println!("目标模式支持占位符:");
    println!("  {{n}}   文件序号");
    println!("  {{ext}} 文件扩展名");
    println!("");
}

/// 执行批量重命名命令
pub fn execute_command(input: &str) -> Result<(), anyhow::Error> {
    // 将输入转换为命令行参数
    let mut args = vec!["hekit".to_string()];
    if let Some(split_args) = shlex::split(input) {
        args.extend(split_args);
    } else {
        anyhow::bail!("命令解析失败");
    }

    // 处理批量重命名命令
    if args.len() >= 2 && args[1] == "batch-rename" {
        let command = config::BatchRenameConfig::build_clap_command();
        let matches = command.get_matches_from(&args[2..]);

        let config = config::BatchRenameConfig::from_matches(&matches)?;
        let executor = executor::BatchRenameExecutor;

        executor.execute(&config)?;
        return Ok(());
    }

    println!("未知命令: {}", input);
    Ok(())
}
