pub mod config;
pub mod core;
pub mod interface;

pub use config::BatchCompressConfig;
pub use core::BatchCompressCore;
pub use interface::{run_interactive, CompressTool};
