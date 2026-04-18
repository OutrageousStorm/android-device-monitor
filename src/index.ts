import { execSync } from "child_process";
import chalk from "chalk";

interface DeviceStats {
  battery: number;
  temperature: number;
  cpu: string;
  ram: { total: number; free: number; used: number };
  disk: { total: number; used: number; free: number };
  uptime: string;
}

class AndroidMonitor {
  private interval: number = 2000;
  private refreshing = false;

  constructor(intervalMs: number = 2000) {
    this.interval = intervalMs;
  }

  private exec(cmd: string): string {
    try {
      return execSync(`adb shell ${cmd}`, { encoding: "utf-8" }).trim();
    } catch {
      return "";
    }
  }

  private parseBattery(): number {
    const out = this.exec("dumpsys battery");
    const match = out.match(/level: (\d+)/);
    return match ? parseInt(match[1]) : 0;
  }

  private parseTemp(): number {
    const out = this.exec("cat /sys/class/thermal/thermal_zone0/temp 2>/dev/null || echo 0");
    const val = parseInt(out) || 0;
    return val / 1000; // usually in millidegrees
  }

  private parseCPU(): string {
    const out = this.exec("top -bn1 | head -3 | tail -1 | awk '{print $7, $8}'");
    return out || "N/A";
  }

  private parseRAM(): { total: number; free: number; used: number } {
    const out = this.exec("cat /proc/meminfo");
    const total = parseInt(out.match(/MemTotal:\s+(\d+)/)?.[1] || "0") / 1024;
    const free = parseInt(out.match(/MemFree:\s+(\d+)/)?.[1] || "0") / 1024;
    return { total, free, used: total - free };
  }

  private parseDisk(): { total: number; used: number; free: number } {
    const out = this.exec("df -h /data | tail -1");
    const parts = out.split(/\s+/);
    const total = parseFloat(parts[1]) || 0;
    const used = parseFloat(parts[2]) || 0;
    const free = parseFloat(parts[3]) || 0;
    return { total, used, free };
  }

  private parseUptime(): string {
    const out = this.exec("uptime");
    const match = out.match(/up\s+(.+?),\s+\d+\s+user/);
    return match ? match[1] : "N/A";
  }

  async getStats(): Promise<DeviceStats> {
    return {
      battery: this.parseBattery(),
      temperature: this.parseTemp(),
      cpu: this.parseCPU(),
      ram: this.parseRAM(),
      disk: this.parseDisk(),
      uptime: this.parseUptime(),
    };
  }

  private formatStats(stats: DeviceStats): string {
    const battery = stats.battery;
    const color =
      battery > 50 ? chalk.green : battery > 20 ? chalk.yellow : chalk.red;

    return `
${chalk.bold("📊 Android Device Monitor")}
${chalk.dim("─".repeat(40))}
🔋 Battery:     ${color(`${battery}%`)}
🌡️  Temperature: ${stats.temperature.toFixed(1)}°C
⚙️  CPU:         ${stats.cpu}
💾 RAM:         ${chalk.cyan(`${stats.ram.used.toFixed(1)}/${stats.ram.total.toFixed(1)}MB`)}
🗄️  Disk:        ${chalk.cyan(`${stats.disk.used.toFixed(1)}/${stats.disk.total.toFixed(1)}GB`)}
⏱️  Uptime:      ${stats.uptime}
${chalk.dim("─".repeat(40))}
`;
  }

  async start(): Promise<void> {
    console.log(chalk.cyan("Starting monitor... (Ctrl+C to stop)\n"));

    const loop = async () => {
      if (this.refreshing) return;
      this.refreshing = true;

      try {
        const stats = await this.getStats();
        console.clear();
        console.log(this.formatStats(stats));
      } catch (e) {
        console.error(chalk.red("Error:"), e instanceof Error ? e.message : "Unknown error");
      } finally {
        this.refreshing = false;
        setTimeout(loop, this.interval);
      }
    };

    loop();
  }
}

// Parse CLI args
const args = process.argv.slice(2);
const intervalArg = args.find((a) => a.startsWith("--interval="));
const interval = intervalArg ? parseInt(intervalArg.split("=")[1]) : 2000;

const monitor = new AndroidMonitor(interval);
monitor.start().catch(console.error);
