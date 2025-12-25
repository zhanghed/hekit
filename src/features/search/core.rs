use anyhow::{anyhow, Result};
use glob::Pattern;
use std::fs;
use std::path::{Path, PathBuf};

/// 批量搜索核心逻辑
pub struct BatchSearchCore;

impl BatchSearchCore {
    /// 执行文件搜索
    pub fn search_files(
        config: &crate::features::search::config::BatchSearchConfig,
    ) -> Result<Vec<PathBuf>> {
        let mut results = Vec::new();

        // 构建文件名匹配模式
        let name_pattern = if config.case_insensitive {
            Pattern::new(&config.name_pattern.to_lowercase())?
        } else {
            Pattern::new(&config.name_pattern)?
        };

        Self::search_directory(
            &config.path,
            &name_pattern,
            &config.file_type,
            config.min_size,
            config.max_size,
            config.recursive,
            config.case_insensitive,
            &mut results,
        )?;

        Ok(results)
    }

    fn search_directory(
        dir: &Path,
        name_pattern: &Pattern,
        file_type: &Option<String>,
        min_size: Option<u64>,
        max_size: Option<u64>,
        recursive: bool,
        case_insensitive: bool,
        results: &mut Vec<PathBuf>,
    ) -> Result<()> {
        let entries =
            fs::read_dir(dir).map_err(|e| anyhow!("无法读取目录 {}: {}", dir.display(), e))?;

        for entry in entries {
            let entry = entry.map_err(|e| anyhow!("无法读取目录项: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                if recursive {
                    Self::search_directory(
                        &path,
                        name_pattern,
                        file_type,
                        min_size,
                        max_size,
                        recursive,
                        case_insensitive,
                        results,
                    )?;
                }
            } else if Self::matches_criteria(
                &path,
                name_pattern,
                file_type,
                min_size,
                max_size,
                case_insensitive,
            )? {
                results.push(path);
            }
        }

        Ok(())
    }

    fn matches_criteria(
        path: &Path,
        name_pattern: &Pattern,
        file_type: &Option<String>,
        min_size: Option<u64>,
        max_size: Option<u64>,
        case_insensitive: bool,
    ) -> Result<bool> {
        // 检查文件名匹配
        let file_name = if case_insensitive {
            path.file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_lowercase())
                .unwrap_or_default()
        } else {
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string()
        };

        if !name_pattern.matches(&file_name) {
            return Ok(false);
        }

        // 检查文件类型
        if let Some(expected_type) = file_type {
            if let Some(actual_ext) = path.extension().and_then(|e| e.to_str()) {
                let expected = if case_insensitive {
                    expected_type.to_lowercase()
                } else {
                    expected_type.clone()
                };
                let actual = if case_insensitive {
                    actual_ext.to_lowercase()
                } else {
                    actual_ext.to_string()
                };

                if actual != expected {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        // 检查文件大小
        if min_size.is_some() || max_size.is_some() {
            let metadata = fs::metadata(path)
                .map_err(|e| anyhow!("无法获取文件元数据 {}: {}", path.display(), e))?;
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
}
