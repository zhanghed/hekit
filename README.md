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

### 直接下载二进制文件（推荐）

1. 访问 [发布页面](https://gitee.com/zhanghed/hekit/releases)
2. 下载对应平台的二进制文件：
   - **Windows**: `hekit-windows-x86_64.exe`
   - **Linux**: `hekit-linux-x86_64`
   - **macOS**: `hekit-macos-x86_64`

3. 使用方法：
   - Windows: 双击运行或命令行执行
   - Linux/macOS: 添加执行权限后运行
   ```bash
   chmod +x hekit-linux-x86_64
   ./hekit-linux-x86_64
   ```

## 🎯 使用方法

### 启动程序

```bash
cargo run
```

程序启动后显示菜单界面：

```
=== HEKIT - 一个简单实用的命令行工具集合 ===
1. 批量重命名工具
2. 批量搜索工具
3. 批量压缩工具
0. 关于HEKIT
======================
请选择 (输入数字): 
```

## 📁 项目结构

```
src/
├── app.rs          # 主应用程序
├── lib.rs          # 库入口
├── main.rs         # 程序入口
├── utils.rs        # 工具函数
├── version.rs      # 版本检查
└── features/       # 功能模块
    ├── common.rs   # 通用接口
    ├── rename/     # 重命名功能
    ├── search/     # 搜索功能
    └── compress/   # 压缩功能
```

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

## 🤝 贡献指南

欢迎提交Issue和Pull Request来改进HEKIT！

1. Fork本项目
2. 创建功能分支：`git checkout -b feature/新功能`
3. 提交更改：`git commit -am '添加新功能'`
4. 推送分支：`git push origin feature/新功能`
5. 提交Pull Request

## 📄 许可证

本项目采用MIT许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。


## ❓ 常见问题

**Q: 程序无法运行怎么办？**
A: 请确保系统满足要求，或尝试重新下载二进制文件。

**Q: 如何报告问题？**
A: 请在Gitee提交Issue，附上详细的问题描述。

**Q: 可以自定义功能吗？**
A: 可以，参考开发指南部分添加新功能。