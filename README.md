# 📊 Android Device Monitor

Real-time monitoring dashboard for Android devices via ADB.

## Install
```bash
npm install
npm run build
npm start
```

## Usage
```bash
npm run monitor                    # 2s refresh
node dist/index.js --interval=1000 # 1s refresh
```

Shows: battery %, CPU, RAM, disk space, temperature, uptime
