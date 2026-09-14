//
use std::net::Ipv4Addr;
use std::process::Command;
//定义Trait 接口
pub trait NetworkProvider {
    //获取网卡地址
    fn get_interfaces(&self) -> Result<Vec<InterfaceInfo>, String>;
    ///检查wifi连接
    fn cmd_wifi_connect(&self) -> bool;
    //扫描wifi
    fn scan_wifi(&self) -> Result<Vec<Wifi>, String>;
}

#[derive(Debug, Clone)]
pub struct InterfaceInfo {
    pub name: String,
    pub ipv4: Ipv4Addr,
    pub prefix_len: u8,
}

#[derive(Debug, Clone, Default)]
pub struct Wifi {
    pub ssid: String,
    pub signal: String,
    pub connected: bool,
}

#[cfg(target_os = "linux")]
#[derive(Default)]
pub struct LinuxNetwork;

#[cfg(target_os = "linux")]
impl NetworkProvider for LinuxNetwork {
    fn get_interfaces(&self) -> Result<Vec<InterfaceInfo>, String> {
        // let output = Command::new('ip')
        //     .args(["-4", "-o", "addr", "show", "eth0"])
        //     .output()
        //     .map_err(|e| format!("Failed to execute ip command: {}", e))?;
        //ip addr show
        Ok(vec![InterfaceInfo {
            name: "eth0".into(),
            ipv4: Ipv4Addr::new(192, 168, 33, 30),
            prefix_len: 24,
        }])
    }
    fn cmd_wifi_connect(&self) -> bool {
        true
    }
    fn scan_wifi(&self) -> Result<Vec<Wifi>, String> {
        Ok(vec![Wifi {
            ssid: "Home-WiFi".into(),
            signal: 85,
            connected: true,
        }])
    }
}
#[cfg(target_os = "windows")]
#[derive(Default)]
pub struct WindowsNetwork;

#[cfg(target_os = "windows")]
impl NetworkProvider for WindowsNetwork {
    fn get_interfaces(&self) -> Result<Vec<InterfaceInfo>, String> {
        Ok(vec![
            InterfaceInfo {
                name: "Wi-Fi".into(),
                ipv4: Ipv4Addr::new(192, 168, 1, 100),
                prefix_len: 24,
            },
            InterfaceInfo {
                name: "以太网".into(),
                ipv4: Ipv4Addr::new(192, 168, 33, 30),
                prefix_len: 24,
            },
        ])
    }

    fn cmd_wifi_connect(&self) -> bool {
        true
    }

    fn scan_wifi(&self) -> Result<Vec<Wifi>, String> {
        Ok(vec![
            Wifi {
                ssid: "Home-WiFi".into(),
                signal: 88,
                connected: true,
            },
            Wifi {
                ssid: "Neighbor-5G".into(),
                signal: 55,
                connected: false,
            },
        ])
    }
}
#[cfg(not(any(target_os = "linux", target_os = "windows")))]
#[derive(Default)]
pub struct StubNetwork;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
impl NetworkProvider for StubNetwork {
    fn get_interfaces(&self) -> Result<Vec<InterfaceInfo>, String> {
        Err("当前平台不支持".into())
    }

    fn cmd_wifi_connect(&self) -> bool {
        false
    }

    fn scan_wifi(&self) -> Result<Vec<Wifi>, String> {
        Err("当前平台不支持".into())
    }
}

#[cfg(target_os = "linux")]
pub type Network = LinuxNetwork;

#[cfg(target_os = "windows")]
pub type Network = WindowsNetwork;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub type Network = StubNetwork;

/// 工厂函数：创建一个网络对象（自动是当前平台的实现）
pub fn network() -> Network {
    Network::default()
}

fn main() {
    // 业务代码只用 network()，不管底层是 Linux 还是 Windows
    let net = network();
    let connected = net.cmd_wifi_connect();

    match net.scan_wifi() {
        Ok(list) => {
            for w in &list {
                println!(
                    "  - {} (信号: {}%, 已连接: {})",
                    w.ssid, w.signal, w.connected
                );
            }
        }
        Err(e) => println!("  失败：{e}"),
    }
    println!();

    match net.get_interfaces() {
        Ok(list) => {
            for iface in &list {
                println!("  - {}: {}/{}", iface.name, iface.ipv4, iface.prefix_len);
            }
        }
        Err(e) => println!("  失败：{e}"),
    }
}
