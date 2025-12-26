use anyhow::Result;
use glob::Pattern;
use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[cfg(windows)]
use winapi::um::winuser::GetAsyncKeyState;

/// 批量搜索核心逻辑
pub struct BatchSearchCore;

impl BatchSearchCore {
    /// 执行文件搜索 - 简化显示版本
    pub fn search_files(
        config: &crate::features::search::config::BatchSearchConfig,
    ) -> Result<(Vec<PathBuf>, usize)> {
        // 预编译文件名匹配模式
        let name_pattern = if config.case_insensitive {
            Pattern::new(&config.name_pattern.to_lowercase())?
        } else {
            Pattern::new(&config.name_pattern)?
        };

        // 预编译文件扩展名（如果指定）- 修复：统一处理大小写
        let file_type_lower = config.file_type.as_ref().map(|s| {
            if config.case_insensitive {
                s.to_lowercase()
            } else {
                s.clone()
            }
        });

        // 使用线程安全的共享数据结构
        let results = Arc::new(Mutex::new(Vec::new()));
        let skipped_dirs = Arc::new(Mutex::new(0usize));
        let processed_dirs = Arc::new(Mutex::new(0usize));
        let found_files = Arc::new(Mutex::new(0usize));

        // 显示搜索开始信息
        println!(
            "搜索: {} (模式: {})",
            config.path.display(),
            config.name_pattern
        );

        let start_time = Instant::now();

        // 使用广度优先搜索（BFS）
        let mut queue = VecDeque::new();
        queue.push_back(config.path.clone());

        // 中断标志
        let mut interrupted = false;
        let mut last_check_time = Instant::now();

        while let Some(current_dir) = queue.pop_front() {
            // 检查是否被中断
            if interrupted {
                println!("搜索被中断");
                break;
            }

            // 每处理10个目录检查一次键盘输入（避免频繁检查影响性能）
            let processed = *processed_dirs.lock().unwrap();
            if processed % 10 == 0 && last_check_time.elapsed() > Duration::from_millis(100) {
                if Self::check_keyboard_input() {
                    interrupted = true;
                    println!("搜索被中断");
                    break;
                }
                last_check_time = Instant::now();
            }

            // 尝试读取目录
            let entries = match fs::read_dir(&current_dir) {
                Ok(entries) => entries,
                Err(_) => {
                    *skipped_dirs.lock().unwrap() += 1;
                    continue;
                }
            };

            *processed_dirs.lock().unwrap() += 1;

            // 批量处理目录项
            let mut subdirs = Vec::new();
            let mut files = Vec::new();

            for entry in entries {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(_) => continue,
                };

                let path = entry.path();

                if path.is_dir() {
                    if config.recursive {
                        subdirs.push(path);
                    }
                } else {
                    files.push(path);
                }
            }

            // 批量处理文件匹配
            let current_found = Self::process_files_sequential_simple(
                &files,
                &name_pattern,
                &file_type_lower,
                config.min_size,
                config.max_size,
                config.case_insensitive,
                results.clone(),
            );

            *found_files.lock().unwrap() += current_found;

            // 添加子目录到队列
            for subdir in subdirs {
                queue.push_back(subdir);
            }
        }

        let final_results = results.lock().unwrap().clone();
        let final_skipped = *skipped_dirs.lock().unwrap();
        let final_found = *found_files.lock().unwrap();
        let elapsed_time = start_time.elapsed();

        // 简化最终统计信息显示 - 只在搜索结束时显示一次
        if !interrupted {
            println!(
                "\n搜索完成! 找到 {} 个文件 (耗时: {:.2}秒)",
                final_found,
                elapsed_time.as_secs_f64()
            );
        } else {
            println!(
                "\n搜索被中断! 找到 {} 个文件 (耗时: {:.2}秒)",
                final_found,
                elapsed_time.as_secs_f64()
            );
        }

        // 显示跳过的目录信息（如果有）- 只在搜索完成后显示
        if final_skipped > 0 {
            println!("因权限问题跳过 {} 个目录", final_skipped);
        }

        Ok((final_results, final_skipped))
    }

    /// 检查键盘输入（Windows系统专用，非阻塞）
    #[cfg(windows)]
    fn check_keyboard_input() -> bool {
        unsafe {
            // 检查Q键是否被按下（VK_Q的虚拟键码是0x51）
            (GetAsyncKeyState(0x51) as u16) & 0x8000 != 0
        }
    }

    /// 检查键盘输入（非Windows系统备用方案）
    #[cfg(not(windows))]
    fn check_keyboard_input() -> bool {
        // 对于非Windows系统，使用简单的标准输入检查
        use std::io::{self, Read};

        let mut buffer = [0u8; 1];
        if io::stdin().read(&mut buffer).is_ok() && buffer[0] != 0 {
            let input = buffer[0] as char;
            input == 'q' || input == 'Q'
        } else {
            false
        }
    }

    /// 快速文件名检查 - 避免不必要的系统调用
    fn quick_filename_check(path: &Path, name_pattern: &Pattern, case_insensitive: bool) -> bool {
        let file_name = if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if case_insensitive {
                name.to_lowercase()
            } else {
                name.to_string()
            }
        } else {
            return false;
        };

        name_pattern.matches(&file_name)
    }

    /// 优化后的匹配条件检查 - 修复文件类型比较逻辑
    fn matches_criteria_optimized(
        path: &Path,
        _name_pattern: &Pattern,
        file_type: &Option<String>,
        min_size: Option<u64>,
        max_size: Option<u64>,
        case_insensitive: bool,
    ) -> Result<bool> {
        // 文件名已经在quick_filename_check中检查过，这里跳过

        // 检查文件类型（如果指定）- 修复：正确处理大小写
        if let Some(expected_type) = file_type {
            if let Some(actual_ext) = path.extension().and_then(|e| e.to_str()) {
                let actual = if case_insensitive {
                    actual_ext.to_lowercase()
                } else {
                    actual_ext.to_string()
                };

                // 修复：比较时考虑大小写设置
                let expected = if case_insensitive {
                    expected_type.to_lowercase()
                } else {
                    expected_type.clone()
                };

                if actual != expected {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        // 检查文件大小（只有在需要时才获取元数据）
        if min_size.is_some() || max_size.is_some() {
            let metadata = match fs::metadata(path) {
                Ok(metadata) => metadata,
                Err(_) => return Ok(false),
            };

            let file_size = metadata.len();

            if let Some(min) = min_size {
                if file_size < min {
                    return Ok(false);
                }
            }

            if let Some(max) = max_size {
                if file_size > max {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// 批量处理文件匹配（简化显示版本）- 只显示找到的文件
    fn process_files_sequential_simple(
        files: &[PathBuf],
        name_pattern: &Pattern,
        file_type: &Option<String>,
        min_size: Option<u64>,
        max_size: Option<u64>,
        case_insensitive: bool,
        results: Arc<Mutex<Vec<PathBuf>>>,
    ) -> usize {
        // 批量处理文件，避免频繁的锁操作
        let mut matched_files = Vec::new();
        let mut found_count = 0;

        for file in files {
            if Self::quick_filename_check(file, name_pattern, case_insensitive) {
                if Self::matches_criteria_optimized(
                    file,
                    name_pattern,
                    file_type,
                    min_size,
                    max_size,
                    case_insensitive,
                )
                .unwrap_or(false)
                {
                    matched_files.push(file.clone());
                    found_count += 1;
                    // 只显示找到的文件
                    println!("{}", file.display());
                }
            }
        }

        // 一次性添加所有匹配的文件到结果中
        if !matched_files.is_empty() {
            results.lock().unwrap().extend(matched_files);
        }

        found_count
    }
}
