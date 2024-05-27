<div align="center">
  <img src="https://cdn.marcel.best/ShareX/2024/05/Code_xodit20vy1.png" alt="screenshot" width="1080" height="290"/>
  
  # Wuthering Waves FPS Unlocker
</div>

<div align="center">
    <a href="https://github.com/dromzeh/wuwa-fps-unlock/releases">Download</a> 
<span> · </span>
    <a href="https://github.com/dromzeh/wuwa-fps-unlocker/issues">Issues</a>
</div>

---

Rust CLI tool for FPS altering in Wuthering Waves.

Detects your primary monitor's refresh rate to streamline the process of setting the FPS values.

This works by reading/writing Wuthering Waves' LocalStorage DB file where the values are stored.

## Usage

### Download

Download `wuwa-fps-unlocker.exe` from the [Releases](https://github.com/dromzeh/wuwa-fps-unlock/releases) page and run it.

### Build Yourself

- Install [CMake](https://cmake.org/download/), [Rust](https://www.rust-lang.org/tools/install) and [Git](https://git-scm.com/downloads).

```bash
git clone https://github.com/dromzeh/wuwa-fps-unlock.git
cd wuwa-fps-unlock
cargo build --verbose --release
```