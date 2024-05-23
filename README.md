# Wuthering Waves FPS Unlocker

Rust CLI tool which allows for altering the default FPS values in the game Wuthering Waves.

This works by reading the LocalStorage DB file and changing the FPS values to the desired ones.

## Usage

### Download

Download the latest release from the [Releases](https://github.com/dromzeh/wuwa-fps-unlock/releases) page.

Then, run the `wuwa-fps-unlocker` executable.

### Build Yourself

```bash
git clone https://github.com/dromzeh/wuwa-fps-unlock.git
cd wuwa-fps-unlock
cargo build --verbose --release
```