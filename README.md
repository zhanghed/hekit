# HEKIT - 一个简单实用的命令行工具集合
HEKIT是一个用Rust编写的命令行工具集合，提供各种实用功能，专注于批量文件处理场景，跨平台兼容且性能优异。

## 🚀 功能特性

### 🔄 批量重命名工具
- 支持前缀、后缀添加
- 支持文本替换（简单替换和正则表达式）
- 支持序号生成（自动补零，可指定补零位数）
- 支持扩展名修改
- 预览模式（先查看修改效果，无实际变更）和备份功能（防止误操作）

### 🔍 批量搜索工具
- 按文件名模式搜索（支持通配符 *）
- 支持文件类型过滤
- 支持文件大小范围筛选（可指定B/KB/MB/GB单位）
- 递归搜索子目录
- 大小写敏感/不敏感选项
- 高性能BFS目录遍历（适合大目录/多层级目录）
- 支持中断搜索（按Q键停止，无冗余等待）

### 📦 批量压缩工具
- 支持多种压缩格式：ZIP、TAR.GZ、TAR.BZ2
- 可调节压缩级别（1-9级，1级最快/压缩率最低，9级最慢/压缩率最高，默认6级）
- 自定义输出目录（避免压缩文件与源文件混放）
- 预览模式

### 🔄 批量转换工具
- 支持主流文件格式转换（图片：JPG↔PNG↔WebP；文档：TXT↔MD等）
- 批量处理多个文件（自动过滤非支持格式）
- 自定义转换参数（如图片压缩质量、分辨率调整）

## 📋 系统要求
- **操作系统**: Windows, Linux, macOS
- **Rust版本**: 1.70.0 或更高版本
- **当前版本**: v1.1.0

## 🛠️ 安装方法

### 从源码编译安装
1.  克隆项目：
```bash
git clone https://gitee.com/zhanghed/hekit.git
cd hekit
```

2.  编译安装：
```bash
cargo build --release
```

3.  运行程序：
```bash
cargo run
```

4.  （可选）全局调用配置：
    编译完成后，二进制文件位于 `./target/release/` 目录下，可将其移动到系统环境变量目录：
    - Linux/macOS：`mv ./target/release/hekit /usr/local/bin`
    - Windows：将 `./target/release/hekit.exe` 移动到 `System32` 目录

### 直接下载二进制文件（推荐）
1.  访问 [发布页面](https://gitee.com/zhanghed/hekit/releases)
2.  下载对应平台的二进制文件：
    - **Windows**: `hekit-windows-x86_64.exe`
    - **Linux**: `hekit-linux-x86_64`
    - **macOS**: `hekit-macos-x86_64`

3.  使用与配置：
    - Windows: 双击运行；或重命名为`hekit.exe`，添加所在目录到系统`Path`，在CMD/PowerShell中直接输入`hekit`启动
    - Linux/macOS: 
      ```bash
      # 添加执行权限
      chmod +x hekit-linux-x86_64
      # （可选）重命名+全局调用
      mv hekit-linux-x86_64 hekit
      mv hekit /usr/local/bin
      # 运行
      ./hekit
      ```

### 编译失败解决方案
若源码编译失败，大概率是缺少系统依赖：
- Linux：安装`build-essential`（Debian/Ubuntu）或`gcc-c++`（CentOS/RHEL）
- macOS：安装Xcode Command Line Tools（执行`xcode-select --install`）
- Windows：安装Visual Studio Build Tools（勾选“C++ 构建工具”）

## 🎯 使用方法
### 启动程序
1.  本地运行（源码编译后/未配置全局）：
```bash
cargo run
```
2.  全局运行（已配置环境变量）：
```bash
hekit
```
程序启动后显示交互式菜单界面，用户可通过菜单选择相应工具进行操作。

## 📁 项目结构
```
├── Cargo.toml        # 项目配置和依赖管理
├── build.rs          # 构建脚本（Windows图标设置）
├── src/
│   ├── main.rs       # 程序入口点
│   ├── app.rs        # 主应用程序逻辑
│   ├── lib.rs        # 库文件
│   ├── utils.rs      # 工具函数
│   ├── version.rs    # 版本检查功能
│   ├── assets/       # 资源文件（图标等）
│   └── features/     # 功能模块
│       ├── compress/ # 批量压缩工具
│       ├── rename/   # 批量重命名工具
│       ├── search/   # 批量搜索工具
│       └── convert/  # 批量转换工具
```

## 🔧 开发指南
### 代码风格
- 使用Rust标准代码风格
- 所有公共API都有文档注释
- 错误处理使用anyhow库
- 模块化设计，职责分离清晰

### 添加新功能
1.  在`features`目录下创建新模块
2.  实现`ToolInterface`特征
3.  在`app.rs`中添加菜单选项
4.  更新模块导出

## 🤝 贡献指南
欢迎提交Issue和Pull Request来改进HEKIT！

1.  Fork本项目
2.  创建功能分支：`git checkout -b feature/新功能`
3.  提交更改：`git commit -am '添加新功能'`
4.  推送分支：`git push origin feature/新功能`
5.  提交Pull Request

## 📄 许可证
本项目采用MIT许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。