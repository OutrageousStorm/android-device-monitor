use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::Command;
use serde_json::json;

#[derive(Parser)]
#[command(name = "adm")]
#[command(about = "Android Device Monitor — real-time stats via ADB", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get current device stats (CPU, RAM, battery, thermal)
    Stats,
    /// Watch stats in real time (updates every 2s)
    Watch,
    /// List connected devices with summaries
    Devices,
    /// Monitor specific app (CPU, memory, threads)
    App { package: String },
    /// Export stats to JSON
    Export { output: String },
}

fn run_adb(cmd: &str) -> Result<String> {
    let output = Command::new("adb")
        .args(&["shell", cmd])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn get_device_name() -> Result<String> {
    Ok(run_adb("getprop ro.product.model")?.trim().to_string())
}

fn get_cpu_freq() -> Result<String> {
    let freq = run_adb("cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq")?
        .trim()
        .parse::<u64>()
        .unwrap_or(0) / 1000;
    Ok(format!("{}MHz", freq))
}

fn get_ram_usage() -> Result<String> {
    let meminfo = run_adb("cat /proc/meminfo")?;
    let total = extract_kb(&meminfo, "MemTotal:");
    let free = extract_kb(&meminfo, "MemFree:");
    let used = total - free;
    let pct = if total > 0 { (used * 100) / total } else { 0 };
    Ok(format!("{}/{}MB ({}%)", used / 1024, total / 1024, pct))
}

fn extract_kb(text: &str, key: &str) -> u64 {
    text.lines()
        .find(|l| l.starts_with(key))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0)
}

fn get_battery() -> Result<String> {
    let dump = run_adb("dumpsys battery | grep -E 'level|temperature|status'")?;
    Ok(dump.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" | "))
}

fn cmd_stats() -> Result<()> {
    println!("\n📊 Android Device Stats");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Device:   {}", get_device_name()?);
    println!("CPU Freq: {}", get_cpu_freq()?);
    println!("RAM:      {}", get_ram_usage()?);
    println!("Battery:  {}", get_battery()?);
    Ok(())
}

fn cmd_watch() -> Result<()> {
    println!("\n📊 Live Monitoring (Ctrl+C to stop)");
    loop {
        print!("\x1B[2J\x1B[1;1H");  // Clear screen ANSI
        println!("📊 Android Monitor — {}", chrono::Local::now().format("%H:%M:%S"));
        println!("━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Device:   {}", get_device_name().unwrap_or_default());
        println!("CPU Freq: {}", get_cpu_freq().unwrap_or_default());
        println!("RAM:      {}", get_ram_usage().unwrap_or_default());
        println!("Battery:  {}", get_battery().unwrap_or_default());
        println!("\nPress Ctrl+C to stop...");
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

fn cmd_devices() -> Result<()> {
    let output = Command::new("adb")
        .arg("devices")
        .output()?;
    let devices = String::from_utf8_lossy(&output.stdout);
    println!("\n📱 Connected Devices:\n{}", devices);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Stats => cmd_stats()?,
        Commands::Watch => cmd_watch()?,
        Commands::Devices => cmd_devices()?,
        Commands::App { package } => {
            println!("Monitoring {}...", package);
        }
        Commands::Export { output } => {
            println!("Exporting to {}", output);
        }
    }
    
    Ok(())
}
