# hekit

src/
├── main.rs          # 程序入口
├── lib.rs           # 库定义
├── app.rs           # 应用程序主逻辑（包含UI工具函数）
└── features/
    ├── mod.rs       # 功能模块定义
    └── batch_rename/
        ├── mod.rs   # 批量重命名模块
        ├── config.rs # 配置管理
        ├── executor.rs # 执行器
        ├── generator.rs # 文件名生成器
        ├── scanner.rs  # 文件扫描器
        └── utils.rs    # 工具函数

