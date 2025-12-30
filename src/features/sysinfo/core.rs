use crate::error::HekitResult;
use crate::features::sysinfo::config::SysInfoConfig;
use crate::utils;
use hostname;
use std::net::ToSocketAddrs;
use sysinfo::{CpuExt, DiskExt, NetworkExt, ProcessExt, System, SystemExt};

/// 系统信息核心逻辑
pub struct SysInfoCore {
    pub config: SysInfoConfig,
    system: System,
}

impl SysInfoCore {
    /// 创建新的系统信息实例
    pub fn new(config: SysInfoConfig) -> Self {
        let mut system = System::new_all();
        if config.refresh {
            system.refresh_all();
        }

        Self { config, system }
    }

    /// 执行系统信息显示
    pub fn execute(&self) -> HekitResult<()> {
        if self.config.show_basic {
            self.show_basic_info()?;
        }

        if self.config.show_cpu {
            self.show_cpu_info()?;
        }

        if self.config.show_memory {
            self.show_memory_info()?;
        }

        if self.config.show_disk {
            self.show_disk_info()?;
        }

        if self.config.show_network {
            self.show_network_info()?;
        }

        if self.config.show_processes {
            self.show_processes_info()?;
        }

        Ok(())
    }

    /// 显示基本信息
    fn show_basic_info(&self) -> HekitResult<()> {
        utils::print_banner_title("系统基本信息");

        println!(
            "系统名称: {}",
            self.system.name().unwrap_or("未知".to_string())
        );
        println!(
            "内核版本: {}",
            self.system.kernel_version().unwrap_or("未知".to_string())
        );
        println!(
            "操作系统版本: {}",
            self.system.os_version().unwrap_or("未知".to_string())
        );
        println!(
            "主机名: {}",
            self.system.host_name().unwrap_or("未知".to_string())
        );
        println!();

        Ok(())
    }

    /// 显示CPU信息
    fn show_cpu_info(&self) -> HekitResult<()> {
        utils::print_banner_title("CPU信息");

        let cpus = self.system.cpus();
        println!("CPU数量: {}", cpus.len());

        // 使用 global_cpu_info() 替代 global_cpu_usage()
        let global_cpu = self.system.global_cpu_info();
        println!("CPU使用率: {:.1}%", global_cpu.cpu_usage());

        for (i, cpu) in cpus.iter().enumerate() {
            println!("CPU {}: {:.1}%", i, cpu.cpu_usage());
        }
        println!();

        Ok(())
    }

    /// 显示内存信息
    fn show_memory_info(&self) -> HekitResult<()> {
        utils::print_banner_title("内存信息");

        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let available_memory = self.system.available_memory();

        println!(
            "总内存: {:.2} GB",
            total_memory as f64 / 1024.0 / 1024.0 / 1024.0
        );
        println!(
            "已用内存: {:.2} GB",
            used_memory as f64 / 1024.0 / 1024.0 / 1024.0
        );
        println!(
            "可用内存: {:.2} GB",
            available_memory as f64 / 1024.0 / 1024.0 / 1024.0
        );
        println!(
            "内存使用率: {:.1}%",
            (used_memory as f64 / total_memory as f64) * 100.0
        );
        println!();

        Ok(())
    }

    /// 显示磁盘信息
    fn show_disk_info(&self) -> HekitResult<()> {
        utils::print_banner_title("磁盘信息");

        for disk in self.system.disks() {
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space - available_space;

            println!("磁盘: {}", disk.name().to_string_lossy());
            println!("  文件系统: {:?}", disk.file_system());
            println!(
                "  总空间: {:.2} GB",
                total_space as f64 / 1024.0 / 1024.0 / 1024.0
            );
            println!(
                "  已用空间: {:.2} GB",
                used_space as f64 / 1024.0 / 1024.0 / 1024.0
            );
            println!(
                "  可用空间: {:.2} GB",
                available_space as f64 / 1024.0 / 1024.0 / 1024.0
            );
            println!(
                "  使用率: {:.1}%",
                (used_space as f64 / total_space as f64) * 100.0
            );
            println!();
        }

        Ok(())
    }

    /// 显示网络信息
    fn show_network_info(&self) -> HekitResult<()> {
        utils::print_banner_title("网络信息");

        // 获取本地IP地址 - 使用更可靠的方法
        let local_ips = get_local_ips();
        if !local_ips.is_empty() {
            println!("本地IP地址:");
            for ip in local_ips {
                println!("  {}", ip);
            }
        } else {
            println!("无法获取本地IP地址");
        }

        // 公网IP地址显示 - 修复显示问题
        println!("公网IP地址: 需要外部服务支持，暂不显示");

        // 显示详细的网络接口信息
        println!("\n网络接口信息:");
        let networks = self.system.networks();

        // 检查是否有网络接口
        let mut has_interfaces = false;
        for (_interface_name, _) in networks {
            has_interfaces = true;
            break;
        }

        if !has_interfaces {
            println!("  未检测到网络接口");
        } else {
            // 重新获取网络接口信息进行详细显示
            let networks = self.system.networks();
            for (interface_name, data) in networks {
                let received_mb = data.total_received() as f64 / 1024.0 / 1024.0;
                let transmitted_mb = data.total_transmitted() as f64 / 1024.0 / 1024.0;
                let packets_received = data.total_packets_received();
                let packets_transmitted = data.total_packets_transmitted();
                let errors_on_received = data.total_errors_on_received();
                let errors_on_transmitted = data.total_errors_on_transmitted();

                println!("  {}:", interface_name);
                println!(
                    "    接收数据: {:.2} MB ({} 个数据包)",
                    received_mb, packets_received
                );
                println!(
                    "    发送数据: {:.2} MB ({} 个数据包)",
                    transmitted_mb, packets_transmitted
                );

                if errors_on_received > 0 || errors_on_transmitted > 0 {
                    println!(
                        "    错误统计: 接收错误={}, 发送错误={}",
                        errors_on_received, errors_on_transmitted
                    );
                }

                // 显示实时速率（如果支持）
                if data.received() > 0 || data.transmitted() > 0 {
                    let received_kbps = data.received() as f64 / 1024.0;
                    let transmitted_kbps = data.transmitted() as f64 / 1024.0;
                    println!(
                        "    实时速率: 接收 {:.1} KB/s, 发送 {:.1} KB/s",
                        received_kbps, transmitted_kbps
                    );
                }
                println!();
            }
        }

        // 显示网络统计信息
        println!("网络统计:");
        let mut total_received = 0;
        let mut total_transmitted = 0;
        let mut total_packets_received = 0;
        let mut total_packets_transmitted = 0;

        let networks = self.system.networks();
        for (_, data) in networks {
            total_received += data.total_received();
            total_transmitted += data.total_transmitted();
            total_packets_received += data.total_packets_received();
            total_packets_transmitted += data.total_packets_transmitted();
        }

        let total_received_gb = total_received as f64 / 1024.0 / 1024.0 / 1024.0;
        let total_transmitted_gb = total_transmitted as f64 / 1024.0 / 1024.0 / 1024.0;

        println!(
            "  总接收数据: {:.3} GB ({} 个数据包)",
            total_received_gb, total_packets_received
        );
        println!(
            "  总发送数据: {:.3} GB ({} 个数据包)",
            total_transmitted_gb, total_packets_transmitted
        );
        println!();

        Ok(())
    }

    /// 显示进程信息
    fn show_processes_info(&self) -> HekitResult<()> {
        utils::print_banner_title("进程信息");

        let processes = self.system.processes();
        println!("进程总数: {}", processes.len());

        // 显示前10个内存使用最多的进程
        let mut sorted_processes: Vec<_> = processes.values().collect();
        sorted_processes.sort_by(|a, b| b.memory().cmp(&a.memory()));

        println!("\n内存使用最多的进程 (前10个):");
        println!(
            "{:<10} {:<30} {:<15} {:<10}",
            "PID", "名称", "内存使用", "CPU使用率"
        );
        println!("{}", "-".repeat(70));

        for process in sorted_processes.iter().take(10) {
            println!(
                "{:<10} {:<30} {:<15} {:<10.1}%",
                process.pid(),
                process.name(),
                format!("{:.2} MB", process.memory() as f64 / 1024.0 / 1024.0),
                process.cpu_usage()
            );
        }
        println!();

        Ok(())
    }
}

// 添加辅助函数来获取本地IP地址
fn get_local_ips() -> Vec<String> {
    let mut ips = Vec::new();

    // 方法1: 使用UDP连接外部服务器获取本地IP
    if let Ok(socket) = std::net::UdpSocket::bind("0.0.0.0:0") {
        // 尝试连接到一个公共DNS服务器来获取本地IP
        if let Ok(_) = socket.connect("8.8.8.8:80") {
            if let Ok(addr) = socket.local_addr() {
                let ip = addr.ip();
                if ip.is_ipv4() && !ip.is_loopback() && !ip.is_unspecified() {
                    ips.push(ip.to_string());
                }
            }
        }
    }

    // 方法2: 使用网络接口信息获取IP地址
    // 在Windows上，我们可以使用更简单的方法
    if let Ok(adapters) = get_network_adapters() {
        for adapter in adapters {
            ips.push(adapter);
        }
    }

    // 方法3: 使用回退方法 - 获取所有可用的网络接口
    if ips.is_empty() {
        // 尝试获取所有网络接口的IP地址
        if let Ok(interface_ips) = get_interface_ips() {
            for ip in interface_ips {
                ips.push(ip);
            }
        }
    }

    // 去重并排序
    ips.sort();
    ips.dedup();
    ips
}

// 获取网络适配器的IP地址（Windows特定）
fn get_network_adapters() -> Result<Vec<String>, std::io::Error> {
    let mut ips = Vec::new();

    // 使用标准库的方法获取网络接口信息
    // 在Windows上，我们可以尝试获取主机名对应的IP地址
    if let Ok(hostname) = hostname::get() {
        let hostname_str = hostname.to_string_lossy();

        // 尝试解析主机名获取IP地址
        // 使用标准库的ToSocketAddrs trait
        if let Ok(addrs) = format!("{}:0", hostname_str).to_socket_addrs() {
            for addr in addrs {
                let ip = addr.ip();
                if ip.is_ipv4() && !ip.is_loopback() {
                    ips.push(ip.to_string());
                }
            }
        }
    }

    Ok(ips)
}

// 获取接口IP地址的备用方法
fn get_interface_ips() -> Result<Vec<String>, std::io::Error> {
    let mut ips = Vec::new();

    // 尝试获取localhost的IP地址
    if let Ok(addrs) = "localhost:0".to_socket_addrs() {
        for addr in addrs {
            let ip = addr.ip();
            if ip.is_ipv4() && !ip.is_loopback() {
                ips.push(ip.to_string());
            }
        }
    }

    // 尝试获取127.0.0.1以外的本地IP地址
    if let Ok(addrs) = "0.0.0.0:0".to_socket_addrs() {
        for addr in addrs {
            let ip = addr.ip();
            if ip.is_ipv4() && !ip.is_loopback() && !ip.is_unspecified() {
                ips.push(ip.to_string());
            }
        }
    }

    Ok(ips)
}
