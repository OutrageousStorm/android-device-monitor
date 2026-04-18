// main.rs - Android Device Monitor in Rust (using ADB)
// Minimal dependencies: just Command for running adb
use std::process::Command;
use std::time::Duration;
use std::thread;

fn adb(cmd: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("adb shell {}", cmd))
        .output()
        .expect("Failed to run adb");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_device_info() {
    println!("\n📱 Android Device Monitor\n");
    
    let model = adb("getprop ro.product.model").trim().to_string();
    let android = adb("getprop ro.build.version.release").trim().to_string();
    let serial = adb("getprop ro.serialno").trim().to_string();
    
    println!("Device:  {}", model);
    println!("Android: {}", android);
    println!("Serial:  {}\n", serial);
}

fn monitor_cpu() {
    let freq = adb("cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq");
    let cores = adb("nproc");
    println!("CPU Freq: {} kHz | Cores: {}", freq.trim(), cores.trim());
}

fn monitor_memory() {
    let mem = adb("cat /proc/meminfo | head -3");
    println!("Memory:\n{}", mem);
}

fn monitor_battery() {
    let level = adb("dumpsys battery | grep 'level:'");
    let status = adb("dumpsys battery | grep 'status:'");
    let temp = adb("dumpsys battery | grep 'temperature:'");
    println!("Battery: {}{}{}", level.trim(), status.trim(), temp.trim());
}

fn monitor_network() {
    let wifi = adb("dumpsys wifi | grep 'mWifiInfo' | grep -oP 'SSID: \\K[^,]+' | head -1");
    let ip = adb("ip route | grep src | awk '{print $NF}' | head -1");
    println!("Network: WiFi={} | IP={}", wifi.trim(), ip.trim());
}

fn main() {
    get_device_info();
    
    loop {
        println!("\n{}", "─".repeat(50));
        println!("Time: {}", chrono::Local::now().format("%H:%M:%S"));
        println!("{}", "─".repeat(50));
        
        monitor_cpu();
        monitor_memory();
        monitor_battery();
        monitor_network();
        
        println!("\nPress Ctrl+C to stop");
        thread::sleep(Duration::from_secs(2));
    }
}
