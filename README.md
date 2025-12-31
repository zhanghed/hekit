# HEKIT - 多功能工具集合
HEKIT是一个用Rust编写的多功能工具集合。

## 🚀 功能特性
- **批量重命名** - 多种重命名规则，预览模式
- **批量搜索** - 文件名模式搜索，文件类型过滤
- **批量压缩** - 支持ZIP、TAR.GZ、TAR.BZ2格式
- **批量转换** - 图片、文本、PDF格式转换
- **批量清理** - 清理空文件夹、临时文件
- **系统信息** - 系统监控
- **关于信息** - 查看程序信息和检查更新

## 🛠️ 安装使用
### 源码编译
```bash
git clone https://gitee.com/zhanghed/hekit.git
cd hekit
cargo build --release
```

### 直接下载
访问 [发布页面](https://gitee.com/zhanghed/hekit/releases) 下载二进制文件

## 📁 项目结构
```
hekit/
├── .cargo/config.toml                 # Cargo配置文件
├── .gitignore                         # Git忽略文件配置
├── Cargo.toml                         # 项目配置和依赖管理
├── Cargo.lock                         # 依赖版本锁定文件
├── LICENSE                            # 许可证文件
├── README.md                          # 项目说明文档
├── build.rs                           # 构建脚本
└── src/
    ├── main.rs                        # 程序入口点
    ├── app.rs                         # 主应用程序逻辑
    ├── lib.rs                         # 库文件
    ├── utils.rs                       # 工具函数
    ├── error.rs                       # 错误处理（HekitError和HekitResult定义）
    ├── version.rs                     # 版本检查功能
    ├── progress.rs                    # 进度条显示功能
    ├── assets/hekit.ico               # 应用程序图标
    └── features/
        ├── mod.rs                     # 功能模块导出
        ├── common.rs                  # 公共工具接口（ToolInterface trait定义）
        ├── clean/                     # 清理功能模块
        ├── compress/                  # 压缩功能模块
        ├── convert/                   # 转换功能模块
        ├── rename/                    # 重命名功能模块
        ├── search/                    # 搜索功能模块
        └── sysinfo/                   # 系统信息功能模块
```


## 📄 许可证
MIT许可证 - 详见 [LICENSE](LICENSE) 文件