use std::process::Command;
use std::thread;
use std::time::Duration;

fn adb_shell(cmd: &str) -> String {
    let output = Command::new("adb")
        .arg("shell")
        .arg(cmd)
        .output()
        .unwrap_or_default();
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_cpu_info() -> String {
    adb_shell("top -n 1 | head -15")
}

fn get_memory_info() -> String {
    adb_shell("cat /proc/meminfo | head -5")
}

fn get_battery_info() -> String {
    adb_shell("dumpsys battery | grep -E 'level|health|temperature'")
}

fn get_temp_info() -> String {
    adb_shell("cat /sys/class/thermal/thermal_zone0/temp 2>/dev/null || echo 'N/A'")
}

fn get_network_info() -> String {
    adb_shell("cat /proc/net/dev | grep -E 'eth|wlan' | head -3")
}

fn main() {
    println!("\n📊 Android Device Monitor — Rust Edition\n");
    
    loop {
        println!("═══════════════════════════════════════");
        println!("⏱  {}", chrono::Local::now().format("%H:%M:%S"));
        println!("═══════════════════════════════════════\n");

        println!("🖥️  CPU (top 5):");
        println!("{}", get_cpu_info());

        println!("\n💾 Memory:");
        println!("{}", get_memory_info());

        println!("\n🔋 Battery:");
        println!("{}", get_battery_info());

        println!("\n🌡️  Temperature:");
        let temp = get_temp_info();
        if let Ok(t) = temp.trim().parse::<u32>() {
            let celsius = t / 1000;
            println!("  {}°C", celsius);
            if celsius > 45 {
                println!("  ⚠️  WARNING: High temperature!");
            }
        }

        println!("\n📡 Network I/O:");
        println!("{}", get_network_info());

        println!("\n(Press Ctrl+C to stop. Updates every 3 seconds)\n");
        thread::sleep(Duration::from_secs(3));
    }
}
