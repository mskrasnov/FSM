<div align="center">
  <img src="https://raw.githubusercontent.com/mskrasnov/FSM/refs/heads/master/ferrix-app/data/com.mskrasnov.Ferrix.svg" width="200">
  <h1>Ferrix System Monitor — Swiss Knife for Linux Hardware Diagnostics</h1>
  <p><b>A simple program for getting information about computer hardware and installed software.</b></p>

  [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/) [![Iced](https://img.shields.io/badge/Made%20with-iced-blue?logo=iced)](https://iced.rs) [![GitHub Release](https://img.shields.io/github/v/release/mskrasnov/ferrix?logo=github)](https://github.com/mskrasnov/ferrix/releases) [![Star this repo!](https://img.shields.io/github/stars/mskrasnov/fsm?style=social)](https://github.com/mskrasnov/FSM/stargazers)

  <img src="https://mskrasnov.github.io/ferrix/screens/sysmon-new.png"> <img src="https://mskrasnov.github.io/ferrix/screens/proc-vuln.png">
  <small><a href="https://mskrasnov.github.io/ferrix/gallery.html">Other screenshots</a></small>
</div>

## What is Ferrix?

Ferrix System Monitor is a Rust-crate and program for obtaining information about computer hardware and software. It is designed to work in modern Linux OS distributions.

## Functions (`ferrix-lib` crate)

1. Get information about:
    - [X] CPU;
    - [X] RAM;
    - [X] Storage;
    - [X] BIOS and PC Motherboard;
    - [X] Laptop battery;
    - [X] Installed Linux distribution;
    - [ ] Desktop environment;
    - [ ] Network;
    - [X] systemd services;
    - [X] `deb`, `rpm` packages;
    - [ ] flatpak packages;
2. Convert collected data into:
    - [X] JSON;
    - [X] XML;

## Build & Install

[![](https://img.shields.io/github/downloads/mskrasnov/fsm/total?color=red)](https://github.com/mskrasnov/FSM/releases) [![](https://img.shields.io/github/downloads/mskrasnov/fsm/latest/total?color=green)](https://github.com/mskrasnov/FSM/releases/latest)

```bash
git clone https://github.com/mskrasnov/Ferrix
cd Ferrix

make build
```

If you use Debian, perform:

```bash
make deb
```

And install `deb`-package:

```bash
sudo dpkg -i ./target/${TARGET_ARCH}/debian/ferrix-app_${VERSION}-${BUILD_NUM}_${ARCH}.deb
```

If you use other Linux system, perform:

```bash
make run # to run Ferrix...
# ... or
make install # to install Ferrix.
# Perform:
make uninstall # to uninstall Ferrix from your system.
```

### Running in WSL

```bash
export XDG_SESSION_TYPE=xorg
export DISPLAY=':0'
export WAYLAND_DISPLAY=
ferrix-app
```

### Cross compilation (Debian x86_64 glibc -> i686/AArch64 glibc)

Install the cross-compilator:

```bash
sudo dpkg --add-architecture {arm64/i686}
sudo apt update

# For AArch64:
sudo apt install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu libc6-dev-arm64-cross
rustup target add aarch64-unknown-linux-gnu

# For i686:
sudo apt install gcc-12-i686-linux-gnu binutils-i686-linux-gnu
rustup target add i686-unknown-linux-gnu
```

Build Ferrix:

```bash
cargo build [--release] --target={i686/aarch64}-unknown-linux-gnu
# or:
make TARGET={i686/aarch64}-unknown-linux-gnu build
```

## Technology stack

- **OS:** Linux with `glibc`, `dbus` and `systemd`;
- **Programming language:** Rust 1.88+ (2024 edition);
- **GUI:** [`iced`](https://iced.rs);
- **Hardware:** modern PC or laptop;

## ❤️ Support Ferrix System Monitor

Developing Ferrix System Monitor takes time and passion. If you find it useful, please consider supporting its development:

- **Star ⭐ this repo!** It helps others discover FSM;
- **Write comments, questions, bug reports, or suggestions** for new functionality in [issues](https://github.com/mskrasnov/Ferrix/issues/new).
- If you are from Russia, **send me a donation 💰** in [Boosty](https://boosty.to/mskrasnov). This will help me keep my enthusiasm alive, as well as pay my internet bills so that I can continue working on FSM.
- **Spread the world!** Tell friends, post on forums.

## License

Ferrix System Monitor is free and open-source software distributed under the **GNU General Public License v3.0**. See [LICENSE](LICENSE) file for details.
