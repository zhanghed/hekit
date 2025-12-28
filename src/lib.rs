pub mod app;
pub mod error;
pub mod features;
pub mod progress; // 添加进度模块
pub mod utils;
pub mod version;

// 重新导出错误类型，以便其他模块可以使用
pub use error::{HekitError, HekitResult};
// 宏会自动导出到 crate 根，不需要显式重新导出
