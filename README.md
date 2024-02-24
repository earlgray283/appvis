# appvis

application launch timing manager written in Rust

## Installation

※ Currently macOS is only supported

TODO

## ⚙️ Configuration

Configuration file is located on `$HOME/.config/appvis/config.toml`. Example is below.

```toml
[applications.hoge]  # application nickname
bin_path = "/Applications/HOGE.app/Contents/MacOS/HOGE"  # application binary path
trigger = { type = "WifiConnected", properties = { interval = { secs = 1, nanos = 0 } } }  # trigger setting
```

## ⚡️ Triggers

### 🌏 Network

#### `WifiConnected`

[properties](https://github.com/earlgray283/appvis/blob/main/src/trigger/network.rs#L9)

trigger when connects Wi-Fi

### 💻 System

#### `AfterAppvisLaunched`

[properties](https://github.com/earlgray283/appvis/blob/main/src/trigger/system.rs#L8)

trigger after appvis launched(same as login items of macOS)

