pub mod clean;
pub mod common;
pub mod compress;
pub mod convert;
pub mod rename;
pub mod search;
pub mod sysinfo;

pub use clean::interface::run_interactive as run_clean;
pub use common::*; // 这行确保common模块中的所有公共项都能被其他模块访问
pub use compress::interface::run_interactive as run_compress;
pub use convert::interface::run_interactive as run_convert;
pub use rename::interface::run_interactive as run_rename;
pub use search::interface::run_interactive as run_search;
pub use sysinfo::interface::run_interactive as run_sysinfo;
