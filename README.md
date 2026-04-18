# ⚡ Android Device Monitor

Real-time monitoring CLI tool for Android devices written in Rust. Minimal overhead, fast, no GUI bloat.

## Features
- Real-time CPU, RAM, thermal readings
- Battery drain rate
- Network I/O per app
- Top processes by CPU
- Temperature warnings
- JSON export for automation

## Install
```bash
cargo install android-device-monitor
```

## Usage
```bash
# Basic monitor
adm monitor

# Top 5 processes by CPU
adm top --cpu 5

# Export to JSON every 5 seconds
adm monitor --interval 5 --output stats.json

# Filter by app package
adm monitor --filter com.facebook.katana

# Temp warning threshold
adm monitor --temp-warn 45
```
