use anyhow::Result;

const GITEE_API_URL: &str = "https://gitee.com/api/v5/repos";
const REPO_OWNER: &str = "zhanghed"; // 替换为你的Gitee用户名
const REPO_NAME: &str = "hekit"; // 替换为你的仓库名

/// 简化版版本检查器
pub struct VersionChecker;

impl VersionChecker {
    /// 检查是否有新版本
    pub async fn check_update() -> Result<()> {
        let current_version = env!("CARGO_PKG_VERSION");
        let latest_version = Self::get_latest_version().await?;

        if Self::is_newer_version(current_version, &latest_version) {
            Self::show_update_prompt(current_version, &latest_version);
        }

        Ok(())
    }

    /// 获取最新版本号（简化版，直接解析JSON字符串）
    async fn get_latest_version() -> Result<String> {
        let url = format!(
            "{}/{}/{}/releases/latest",
            GITEE_API_URL, REPO_OWNER, REPO_NAME
        );

        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let text = response.text().await?;
            // 简单解析tag_name字段
            if let Some(start) = text.find("\"tag_name\":\"") {
                let start = start + 11; // "\"tag_name\":\""的长度
                if let Some(end) = text[start..].find('\"') {
                    return Ok(text[start..start + end].to_string());
                }
            }
        }

        // 网络错误或解析失败时返回当前版本
        Ok(env!("CARGO_PKG_VERSION").to_string())
    }

    /// 比较版本号
    fn is_newer_version(current: &str, latest: &str) -> bool {
        let current = current.trim_start_matches('v');
        let latest = latest.trim_start_matches('v');

        let current_parts: Vec<u32> = current.split('.').map(|s| s.parse().unwrap_or(0)).collect();
        let latest_parts: Vec<u32> = latest.split('.').map(|s| s.parse().unwrap_or(0)).collect();

        for i in 0..current_parts.len().max(latest_parts.len()) {
            let current_num = current_parts.get(i).unwrap_or(&0);
            let latest_num = latest_parts.get(i).unwrap_or(&0);

            if latest_num > current_num {
                return true;
            }
            if latest_num < current_num {
                return false;
            }
        }
        false
    }

    /// 显示更新提示
    fn show_update_prompt(current: &str, latest: &str) {
        println!();
        println!("🎉 发现新版本: {} → {}", current, latest);
        println!(
            "下载地址: https://gitee.com/{}/{}/releases",
            REPO_OWNER, REPO_NAME
        );
        println!();
    }
}
