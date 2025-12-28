use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// 进度显示管理器
pub struct ProgressManager {
    progress_bar: ProgressBar,
}

impl ProgressManager {
    /// 创建新的进度管理器
    pub fn new(total: u64, message: &str) -> Self {
        let progress_bar = ProgressBar::new(total);

        // 设置进度条样式
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );

        progress_bar.set_message(message.to_string());

        Self { progress_bar }
    }

    /// 更新进度
    pub fn inc(&self, delta: u64) {
        self.progress_bar.inc(delta);
    }

    /// 设置当前消息
    pub fn set_message(&self, message: &str) {
        self.progress_bar.set_message(message.to_string());
    }

    /// 完成进度条
    pub fn finish(&self) {
        self.progress_bar.finish_with_message("完成");
    }

    /// 完成并显示自定义消息
    pub fn finish_with_message(&self, message: &str) {
        self.progress_bar.finish_with_message(message.to_string());
    }

    /// 创建不确定的进度条（用于长时间运行但不知道总进度的任务）
    pub fn create_indeterminate(message: &str) -> Self {
        let progress_bar = ProgressBar::new_spinner();
        progress_bar.enable_steady_tick(Duration::from_millis(100));

        progress_bar.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );

        progress_bar.set_message(message.to_string());

        Self { progress_bar }
    }
}

impl Drop for ProgressManager {
    fn drop(&mut self) {
        self.progress_bar.finish_and_clear();
    }
}
