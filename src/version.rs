use anyhow::Result;

const GITEE_API_URL: &str = "https://gitee.com/api/v5/repos";
const REPO_OWNER: &str = "zhanghed";
const REPO_NAME: &str = "hekit";

pub struct VersionChecker;

impl VersionChecker {
    pub fn check_update_sync() -> Result<(bool, String)> {
        let current_version = env!("CARGO_PKG_VERSION");
        match Self::get_latest_version_sync() {
            Ok(latest_version) => {
                if latest_version.trim().is_empty()
                    || !latest_version.chars().any(|c| c.is_ascii_digit())
                {
                    return Ok((false, latest_version));
                }
                if Self::is_newer_version(current_version, &latest_version) {
                    return Ok((true, latest_version));
                } else {
                    return Ok((false, latest_version));
                }
            }
            Err(_) => Ok((false, "".to_string())),
        }
    }

    fn get_latest_version_sync() -> Result<String> {
        let url = format!(
            "{}/{}/{}/releases/latest",
            GITEE_API_URL, REPO_OWNER, REPO_NAME
        );
        let response = reqwest::blocking::get(&url)?;
        if response.status().is_success() {
            let text = response.text()?;
            let version = Self::extract_version_from_json(&text);
            if !version.is_empty() {
                return Ok(version);
            }
        }
        Ok("".to_string())
    }

    fn extract_version_from_json(text: &str) -> String {
        if let Some(tag_start) = text.find("\"tag_name\":\"") {
            let tag_start = tag_start + 12;
            if let Some(tag_end) = text[tag_start..].find('\"') {
                let tag_value = text[tag_start..tag_start + tag_end].to_string();
                if tag_value.chars().any(|c| c.is_ascii_digit()) {
                    return tag_value;
                }
            }
        }
        if let Some(name_start) = text.find("\"name\":\"") {
            let name_start = name_start + 8;
            if let Some(name_end) = text[name_start..].find('\"') {
                let name_value = text[name_start..name_start + name_end].to_string();
                if name_value.chars().any(|c| c.is_ascii_digit()) {
                    return name_value;
                }
            }
        }
        "".to_string()
    }

    fn is_newer_version(current: &str, latest: &str) -> bool {
        let current = current.trim_start_matches('v');
        let latest = latest.trim_start_matches('v');
        if latest.trim().is_empty() || current == latest {
            return false;
        }
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
}
