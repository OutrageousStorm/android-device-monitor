#!/usr/bin/env python3
"""
monitor.py -- Real-time Android device monitoring
Tracks CPU, RAM, temperature, battery, and network in live dashboard.
Usage: python3 monitor.py [--interval 2]
"""
import subprocess, time, os, re, argparse
from datetime import datetime

def adb(cmd):
    return subprocess.run(f"adb shell {cmd}", shell=True, capture_output=True, text=True).stdout.strip()

class DeviceMonitor:
    def __init__(self, interval=2):
        self.interval = interval
        self.start_time = time.time()

    def get_cpu_usage(self):
        out = adb("top -n 1 | head -3")
        m = re.search(r'(\d+)%', out)
        return m.group(1) if m else "N/A"

    def get_ram_usage(self):
        meminfo = adb("cat /proc/meminfo")
        total = int(re.search(r'MemTotal:\s+(\d+)', meminfo).group(1)) // 1024
        available = int(re.search(r'MemAvailable:\s+(\d+)', meminfo).group(1)) // 1024
        used = total - available
        pct = int((used / total) * 100)
        return f"{used}MB/{total}MB ({pct}%)"

    def get_temp(self):
        temps = adb("cat /sys/class/thermal/thermal_zone*/temp 2>/dev/null")
        if not temps:
            return "N/A"
        temps_c = [int(t) // 1000 for t in temps.split() if t.isdigit()]
        return f"{max(temps_c) if temps_c else 0}°C" if temps_c else "N/A"

    def get_battery(self):
        bat = adb("dumpsys battery | grep -E 'level|temperature'")
        level = re.search(r'level: (\d+)', bat)
        temp = re.search(r'temperature: (\d+)', bat)
        return f"{level.group(1)}% / {int(temp.group(1))//10}°C" if level and temp else "N/A"

    def get_network(self):
        net = adb("ifconfig wlan0 | grep 'RX packets'")
        rx = re.search(r'RX packets:(\d+)', net)
        tx = re.search(r'TX packets:(\d+)', net)
        return f"RX:{rx.group(1)} TX:{tx.group(1)}" if rx and tx else "N/A"

    def display(self):
        os.system("clear" if os.name != "nt" else "cls")
        uptime = int(time.time() - self.start_time)
        print(f"\n📊 Android Device Monitor — {datetime.now().strftime('%H:%M:%S')} | Uptime: {uptime}s\n")
        print(f"  CPU Usage:     {self.get_cpu_usage()}")
        print(f"  RAM Usage:     {self.get_ram_usage()}")
        print(f"  Temperature:   {self.get_temp()}")
        print(f"  Battery:       {self.get_battery()}")
        print(f"  Network:       {self.get_network()}")
        print("\n  Press Ctrl+C to stop\n")

    def run(self):
        try:
            while True:
                self.display()
                time.sleep(self.interval)
        except KeyboardInterrupt:
            print("Stopped.")

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--interval", type=int, default=2)
    args = parser.parse_args()
    DeviceMonitor(args.interval).run()
