use crate::utils;
use std::fmt;

/// 统一的错误类型枚举
#[derive(Debug, Clone)]
pub enum HekitError {
    /// 参数解析错误
    ArgumentParse(String),
    /// 文件操作错误
    FileOperation(String),
    /// 网络错误
    Network(String),
    /// 配置错误
    Configuration(String),
    /// 用户输入错误
    UserInput(String),
    /// 系统错误
    System(String),
    /// 压缩错误
    Compression(String),
    /// 重命名错误
    Rename(String),
    /// 搜索错误
    Search(String),
    /// 转换错误
    Conversion(String),
    /// 返回主菜单
    BackToMainMenu(String),
    /// 未知错误
    Unknown(String),
}

impl fmt::Display for HekitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HekitError::ArgumentParse(msg) => write!(f, "参数解析错误: {}", msg),
            HekitError::FileOperation(msg) => write!(f, "文件操作错误: {}", msg),
            HekitError::Network(msg) => write!(f, "网络错误: {}", msg),
            HekitError::Configuration(msg) => write!(f, "配置错误: {}", msg),
            HekitError::UserInput(msg) => write!(f, "输入错误: {}", msg),
            HekitError::System(msg) => write!(f, "系统错误: {}", msg),
            HekitError::Compression(msg) => write!(f, "压缩错误: {}", msg),
            HekitError::Rename(msg) => write!(f, "重命名错误: {}", msg),
            HekitError::Search(msg) => write!(f, "搜索错误: {}", msg),
            HekitError::Conversion(msg) => write!(f, "转换错误: {}", msg),
            HekitError::BackToMainMenu(msg) => write!(f, "{}", msg), // 不显示错误信息
            HekitError::Unknown(msg) => write!(f, "未知错误: {}", msg),
        }
    }
}

impl std::error::Error for HekitError {}

// 为 anyhow::Error 提供转换
impl From<anyhow::Error> for HekitError {
    fn from(error: anyhow::Error) -> Self {
        HekitError::Unknown(error.to_string())
    }
}

// 为 std::io::Error 提供转换
impl From<std::io::Error> for HekitError {
    fn from(error: std::io::Error) -> Self {
        HekitError::FileOperation(error.to_string())
    }
}

/// 统一的错误处理函数
pub fn handle_error(error: &dyn std::error::Error, context: &str) {
    let error_type = classify_error(error);

    match error_type {
        HekitError::ArgumentParse(_) => {
            utils::print_error(&format!("{}: {}", context, error));
            utils::print_info("请输入 'help' 查看正确的命令格式");
        }
        HekitError::FileOperation(_) => {
            utils::print_error(&format!("{}: {}", context, error));
            utils::print_info("请检查文件路径和权限是否正确");
        }
        HekitError::UserInput(_) => {
            utils::print_warning(&format!("{}: {}", context, error));
            utils::print_info("请重新输入正确的参数");
        }
        HekitError::Compression(_) => {
            utils::print_error(&format!("{}: {}", context, error));
            utils::print_info("请检查文件格式和压缩设置");
        }
        HekitError::Rename(_) => {
            utils::print_error(&format!("{}: {}", context, error));
            utils::print_info("请检查文件名和重命名规则");
        }
        _ => {
            utils::print_error(&format!("{}: {}", context, error));
            utils::print_info("请检查系统环境或联系技术支持");
        }
    }
}

/// 错误分类函数
fn classify_error(error: &dyn std::error::Error) -> HekitError {
    let error_msg = error.to_string().to_lowercase();

    if error_msg.contains("参数") || error_msg.contains("argument") || error_msg.contains("parse")
    {
        HekitError::ArgumentParse(error.to_string())
    } else if error_msg.contains("文件") || error_msg.contains("file") || error_msg.contains("路径")
    {
        HekitError::FileOperation(error.to_string())
    } else if error_msg.contains("网络")
        || error_msg.contains("network")
        || error_msg.contains("http")
    {
        HekitError::Network(error.to_string())
    } else if error_msg.contains("输入")
        || error_msg.contains("input")
        || error_msg.contains("用户")
    {
        HekitError::UserInput(error.to_string())
    } else if error_msg.contains("压缩")
        || error_msg.contains("compress")
        || error_msg.contains("zip")
    {
        HekitError::Compression(error.to_string())
    } else if error_msg.contains("重命名") || error_msg.contains("rename") {
        HekitError::Rename(error.to_string())
    } else if error_msg.contains("搜索") || error_msg.contains("search") {
        HekitError::Search(error.to_string())
    } else if error_msg.contains("转换") || error_msg.contains("convert") {
        HekitError::Conversion(error.to_string())
    } else {
        HekitError::Unknown(error.to_string())
    }
}

/// 简化的错误处理宏
#[macro_export]
macro_rules! hekit_error {
    ($error_type:ident, $msg:expr) => {
        Err(crate::error::HekitError::$error_type($msg.to_string()).into())
    };
}

/// 结果类型别名，便于使用
pub type HekitResult<T> = Result<T, HekitError>;
