pub mod config;
pub mod core;
pub mod interface;

pub use config::SysInfoConfig;
pub use core::SysInfoCore;
pub use interface::{run_interactive, SysInfoTool};
