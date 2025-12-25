# HEKIT - 一个简单实用的命令行工具集合

HEKIT是一个用Rust编写的命令行工具集合，提供批量重命名、批量搜索和批量压缩等实用功能。

## 🚀 功能特性

### 🔄 批量重命名工具
- 支持前缀、后缀添加
- 支持文本替换（简单替换和正则表达式）
- 支持序号生成（自动补零）
- 支持扩展名修改
- 预览模式和备份功能

### 🔍 批量搜索工具
- 按文件名模式搜索
- 支持文件类型过滤
- 支持文件大小范围筛选
- 递归搜索子目录
- 大小写敏感/不敏感选项

### 📦 批量压缩工具
- 支持多种压缩格式：ZIP、TAR.GZ、TAR.BZ2
- 可调节压缩级别
- 自定义输出目录
- 预览模式

## 📋 系统要求

- **操作系统**: Windows, Linux, macOS
- **Rust版本**: 1.70.0 或更高版本

## 🛠️ 安装方法

### 从源码编译安装

1. 克隆项目：
```bash
git clone https://gitee.com/zhanghed/hekit.git
cd hekit
```

2. 编译安装：
```bash
cargo build --release
```

3. 运行程序：
```bash
cargo run
```

### 直接下载二进制文件

从 [发布页面](https://gitee.com/zhanghed/hekit/releases) 下载对应平台的二进制文件。

## 🎯 使用方法

### 启动程序

```bash
cargo run
```

### 批量重命名工具使用示例

#### 基本用法
```bash
# 预览重命名效果
rename -m "*.txt" -p "2024_" -n -v

# 实际执行重命名
rename -m "*.jpg" -s "_backup" -b
```

#### 命令选项
| 选项 | 短选项 | 说明 | 示例 |
|------|--------|------|------|
| `--path` | `-d` | 目标文件夹 | `-d ./photos` |
| `--match` | `-m` | 文件匹配模式 | `-m "*.jpg"` |
| `--prefix` | `-p` | 添加前缀 | `-p "2024_"` |
| `--suffix` | `-s` | 添加后缀 | `-s "_backup"` |
| `--replace` | `-r` | 文本替换 | `-r "old=new"` |
| `--number` | `-n` | 添加序号 | `-n 5`（从5开始）|
| `--ext` | `-e` | 修改扩展名 | `-e "md"` |
| `--preview` | `-v` | 预览模式 | `-v` |
| `--backup` | `-b` | 备份原文件 | `-b` |

### 批量搜索工具使用示例

```bash
# 搜索所有txt文件
search -m "*.txt" -r

# 搜索大于1MB的图片文件
search -m "*.jpg" -t jpg --min-size 1048576
```

### 批量压缩工具使用示例

```bash
# 压缩为ZIP格式
compress -m "*.log" -f zip -l 6

# 压缩为TAR.GZ格式并预览
compress -m "*.txt" -f tar.gz -v
```

## 📁 项目结构
src/ ├── app.rs # 主应用程序 ├── lib.rs # 库入口 ├── main.rs # 程序入口 ├── utils.rs # 工具函数 ├── version.rs # 版本检查 └── features/ # 功能模块 ├── common.rs # 通用接口 ├── rename/ # 重命名功能 ├── search/ # 搜索功能 └── compress/ # 压缩功能

## 🔧 开发指南

### 代码风格
- 使用Rust标准代码风格
- 所有公共API都有文档注释
- 错误处理使用anyhow库
- 模块化设计，职责分离清晰

### 添加新功能
1. 在`features`目录下创建新模块
2. 实现`ToolInterface`特征
3. 在`app.rs`中添加菜单选项
4. 更新模块导出

### 测试
```bash
# 运行测试
cargo test

# 检查代码格式
cargo fmt

# 检查代码质量
cargo clippy
```

## 🤝 贡献指南

欢迎提交Issue和Pull Request来改进HEKIT！

1. Fork本项目
2. 创建功能分支：`git checkout -b feature/新功能`
3. 提交更改：`git commit -am '添加新功能'`
4. 推送分支：`git push origin feature/新功能`
5. 提交Pull Request

## 📄 许可证

本项目采用MIT许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

感谢以下开源项目的支持：
- [anyhow](https://github.com/dtolnay/anyhow) - 错误处理
- [clap](https://github.com/clap-rs/clap) - 命令行参数解析
- [glob](https://github.com/rust-lang/glob) - 文件匹配
- [regex](https://github.com/rust-lang/regex) - 正则表达式

---

**HEKIT** - 让文件处理变得更简单！ 🚀