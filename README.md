<div align="center">
  <img src="https://raw.githubusercontent.com/mskrasnov/FSM/refs/heads/master/ferrix-app/data/com.mskrasnov.Ferrix.svg" width="200">
  <h1>Ferrix System Monitor — Swiss Army Knife for Linux Hardware Diagnostics</h1>
  <p><b>A modern program for getting information about computer hardware and installed software.</b></p>

  [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/) [![Iced](https://img.shields.io/badge/Made%20with-iced-blue?logo=iced)](https://iced.rs) [![GitHub Release](https://img.shields.io/github/v/release/mskrasnov/ferrix?logo=github&color=lightgray)](https://github.com/mskrasnov/ferrix/releases) [![Support me](https://img.shields.io/badge/Donate_me-Boosty-orange)](https://boosty.to/mskrasnov) [![All downloads](https://img.shields.io/github/downloads/mskrasnov/fsm/total)](https://github.com/mskrasnov/FSM/releases) [![Star this repo!](https://img.shields.io/github/stars/mskrasnov/fsm?style=social)](https://github.com/mskrasnov/FSM/stargazers)

  <img src="https://mskrasnov.github.io/ferrix/screens/sysmon-new.png" width="45%"> <img src="https://mskrasnov.github.io/ferrix/screens/firmware.png" width="45%">

  <p><small><a href="https://mskrasnov.github.io/ferrix/gallery.html">Other screenshots</a></small></p>
</div>

## What is FSM?

FSM is a modern system profiler. Is a program for obtaining information about computer hardware and software. It is designed to work in modern GNU/Linux systems.

<a href="https://github.com/mskrasnov/FSM/releases/download/v0.7.1/Ferrix.System.Monitor-v0.7.1-x86_64.AppImage"><img src="./fxapp/data/download-appimage-banner.svg" width="364" height="112"></a>

## Functions

- Beautiful CPU and RAM utilization charts ([System Monitor](https://mskrasnov.github.io/ferrix/screens/sysmon-new.png) page);
- Hardware:
    - Information about installed CPUs: name(s), model(s), topology, frequencies, vulnerabilities;
    - Filesystems: mount point and mount options, total and used size, file system type;
    - Memory: total, free, used memory, cached memory, buffers, swap(s), etc.;
    - Network interfaces list;
    - Some data from the DMI tables (BIOS, System, Baseboard, Processors) - WIP;
    - Installed notebook battery(es) - status, capacity, battery health, technology, voltage, power, energy, battery manufacturer, battery model and serial number;
    - Screens - supported modes and some data from EDID;
- Software:
    - Information about installed GNU/Linux system: name, version, maintainer/developer, homepage URL, etc.;
    - Users and groups list;
    - Environment variables list;
    - `systemd` services list;
    - Installed software list (only `deb` and `rpm` packages is displayed yet);
    - Linux kernel information;
    - Kernel modules list;
    - Desktop environment name;
    - UEFI Settings (tested on Lenovo ThinkBook);

## Difference from analogues

| Criteria                   | FSM | Hardinfo2 | Stacer/Nexis |
|----------------------------|-----|-----------|--------------|
| Programming language       | Rust                  | C         | C++          |
| Program type               | System profiler       | System profiler & hardware benchmark | System optimizer and monitor |
| GUI                        | `iced`                | GTK3      | Qt5/Qt6      |
| License                    | GNU GPLv3             | GNU GPLv3 | GNU GPLv3    |
| CPU&RAM utilization charts | 🟢️ | ⚪️ (charts is ugly) | 🟢️ |
| CPU Information            | 🟢️ | 🟢️ | ⚪️ |
| Memory Information         | 🟢️ | 🟢️ | ⚪️ |
| Filesystems Information    | 🟢️ | 🟢️ | 🔴️ |
| Network Information        | ⚪️ | 🟢️ | 🔴️ |
| DMI Tables contents        | ⚪️ (full information from BIOS, Baseboard, Chassis, System, Processors, Memory Devices tables) | ⚪️ (only basic information from Chassis, BIOS, Baseboard tables) | 🔴️ |
| Notebook battery(es) Info  | 🟢️ | ⚪️ ([incorrect battery information](https://raw.githubusercontent.com/mskrasnov/mskrasnov.github.io/refs/heads/master/ferrix/assets/hardinfo2.png)) | 🟢️ |
| Connected screens Info     | ⚪️ | 🟢️ | 🔴️ |
| Installed Linux system     | 🟢️ | 🟢️ | 🔴️ |
| Users and groups list      | 🟢️ | 🟢️ | 🔴️ |
| Environment variables list | 🟢️ | 🟢️ | 🔴️ |
| `systemd` services list    | 🟢️ | 🔴️ | 🔴️ |
| Linux boots list           | 🔴️ | 🟢️ | 🔴️ |
| Installed software list    | 🟢️ | 🔴️ | 🔴️ |
| Linux kernel information   | 🟢️ | 🟢️ | 🔴️ |
| Desktop environment info   | ⚪️ | ⚪️ | ⚪️ |
| Command shell info         | 🟢️ | 🔴️ | 🔴️ |
| UEFI Attributes Info       | 🟢️ | 🔴️ | 🔴️ |
| Hardware benchmarks        | 🔴️ | 🟢️ | 🔴️ |
| System cleaning tools      | 🔴️ | 🔴️ | 🟢️ |
| PCI- and USB-devices list  | 🔴️ | 🟢️ | 🔴️ |
| Supported locales list     | 🔴️ | 🟢️ | 🔴️ |
| GPU information            | 🔴️ | 🟢️ | 🔴️ |
| Sensors information        | 🔴️ | 🟢️ | ⚪️ |
| Target audience | Advanced users who need detailed information about software and hardware | Enthusiasts and overlockers interested in benchmarks and system comparisons | Beginners and regular users who want a simple tool for configuring and cleaning up their system |
| Real-time monitoring       | 🟢️ | ⚪️ | 🟢️ |
| Official AppImage builds   | 🟢️ | 🔴️ | 🟢️ |

- 🟢️ - yes;
- 🔴️ - no;
- ⚪️ - partial;

## Installation

[![](https://img.shields.io/github/downloads/mskrasnov/fsm/total?color=red)](https://github.com/mskrasnov/FSM/releases) [![](https://img.shields.io/github/downloads/mskrasnov/fsm/latest/total?color=green)](https://github.com/mskrasnov/FSM/releases/latest)

You can use the universal AppImage package (no installation required) or install the deb/rpm packages (for Debian/Ubuntu or Fedora/RHEL).

### For all systems (AppImage package)

- **Dependencies:**
    - glibc >= 2.36;
    - Xorg or Wayland;
    - Graphics drivers;

Download the AppImage package (only `amd64` is supported yet):
<br>[![](https://img.shields.io/badge/FSM_v0.7.1_AppImage-amd64-blue)](https://github.com/mskrasnov/FSM/releases/download/v0.7.1/Ferrix.System.Monitor-v0.7.1-x86_64.AppImage)

Make this package executable:

```bash
sudo chmod +x ./Ferrix.System.Monitor-v0.7.1-x86_64.AppImage
```

Run this package:

```bash
./Ferrix.System.Monitor-v0.7.1-x86_64.AppImage
```

### For Debian/Ubuntu

- **Minimum version required:** Debian 11, Ubuntu 22.04;
- **Dependencies:**
    - glibc >= 2.36;
    - dbus;
    - Xorg or Wayland;
    - Graphics drivers;

Download the package for your CPU architecture:
<br>[![](https://img.shields.io/badge/FSM_v0.7.1_Debian-amd64-yellow)](https://github.com/mskrasnov/FSM/releases/download/v0.7.1/ferrix-app_0.7.1-1_amd64.deb) [![](https://img.shields.io/badge/FSM_v0.7.1_Debian-i386-orange)](https://github.com/mskrasnov/FSM/releases/download/v0.7.1/ferrix-app_0.7.1-1_i386.deb) [![](https://img.shields.io/badge/FSM_v0.7.1_Debian-ARM64-red)](https://github.com/mskrasnov/FSM/releases/download/v0.7.1/ferrix-app_0.7.1-1_arm64.deb)

Install using `apt`:

```bash
sudo apt install ./ferrix-app_0.7.1-1_${your architecture}.deb
```

### For Fedora/RHEL

- **Minimum version required:** Fedora 42;
- **Dependencies:**
    - glibc >= 2.36;
    - dbus;
    - Xorg or Wayland;
    - Graphics drivers;

Download the package (only `amd64` is supported yet):
<br>[![](https://img.shields.io/badge/FSM_v0.7.1_Fedora-amd64-purple)](https://github.com/mskrasnov/FSM/releases/download/v0.7.1/ferrix-app-0.7.1-1.x86_64.rpm)

Install using `dnf`:

```bash
sudo dnf install ./ferrix-app-0.7.1-1.x86_64.rpm
```

## Building from the source code

- **Dependencies:**
    - glibc >= 2.36;
    - GNU make;
    - GNU coreutils;
    - `rustc`, `cargo` >= 1.96;
    - [optional] `dpkg`, `dpkg-dev`, `liblzma-dev` for building the `deb` package;
    - [optional] `docker`, `buildx-plugin` for docker;
    - [optional] `libfuse2`, `wget` for appimage build;

Clone this repository:

```bash
git clone https://github.com/mskrasnov/FSM
cd        ./FSM
```

### Building on the host

**1. Prepare your host system:**

```bash
# If you use Debian/Ubuntu and its derivatives:
sudo bash ./packaging/debian/setup.sh

# If you use Fedora/RHEL:
sudo bash ./packaging/fedora/setup.sh
```

This script will install all build dependencies, including the Rust programming language toolchain (`rustup`, `cargo`, `rustc`).

**2. Build the package:**

- `debug`-profile without optimizations and LTO:

```bash
make debug
```

- `release`-profile with optimizations and LTO and without debug symbols:

```bash
make build
```

If you want to build Debian package, run this (on Debian):

```bash
cargo install cargo-deb
make deb
```

If you want to build Fedora/RHEL package, run this (on Fedora):

```bash
cargo install cargo-generate-rpm
cargo generate-rpm --target-dir=${PWD}/target/
```

### Cross compilation (Debian 12 x86_64 glibc -> i686/AArch64 glibc)

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
#   debug-profile:
make TARGET={i686/aarch64}-unknown-linux-gnu debug
# or
#   release-profile:
make TARGET={i686/aarch64}-unknown-linux-gnu build
```

### Docker build

> **Note 1:** In the Docker container you can build the Debian or Fedora/RHEL packages and AppImage.

> **Note 2:** Each of the built packages will be placed in the `builds/` directory.

**1. Some preparations.**

**1.1. To build `deb` or `AppImage` packages:**

```bash
docker build -t fsm-debian -f ./packaging/debian/Dockerfile .
```

**1.2. To build `rpm` package:**

```bash
docker build -t fsm-fedora -f ./packaging/fedora/Dockerfile .
```

**2. Build packages.**

**2.1. Build `deb`-packages:**

```bash
docker run --rm            \
    -v "${PWD}:/workspace" \
    fsm-debian             \
    packaging/debian/build.sh
```

> **Note:** This script will build 3 packages: for the `amd64`, `i386`, and `aarch64` architectures.

**2.2. Build `AppImage` package (for `amd64`):**

```bash
docker run --rm            \
    -v "${PWD}:/workspace" \
    fsm-debian             \
    packaging/debian/appimage.sh
```

**2.3. Build `rpm` package (for `amd64`):**

```bash
docker run -- rm           \
    -v "${PWD}:/workspace" \
    fsm-fedora             \
    packaging/fedora/build.sh
```

## Running in WSL

```bash
export XDG_SESSION_TYPE=xorg
export DISPLAY=':0'
export WAYLAND_DISPLAY=
ferrix-app
```

## Technology stack

- **OS:** Linux with `glibc`, `dbus` and `systemd`;
- **Programming language:** Rust 1.96+ (2024 edition);
- **GUI:** [`iced`](https://iced.rs);
- **Hardware:** modern PC or laptop;

## ❤️ Support Ferrix System Monitor

Developing Ferrix System Monitor takes time and passion. If you find it useful, please consider supporting its development:

- **Star ⭐ this repo!** It helps others discover FSM;
- **Write comments, questions, bug reports, or suggestions** for new functionality in [issues](https://github.com/mskrasnov/Ferrix/issues/new).
- If you are from Russia, **send me a donation 💰** in [Boosty](https://boosty.to/mskrasnov). This will help me keep my enthusiasm alive, as well as pay my internet bills so that I can continue working on FSM.
- **Spread the world!** Tell friends, post on forums.

## License

Ferrix System Monitor is free and open-source software distributed under the GNU General Public License v3.0. See [LICENSE](LICENSE) file for details.
