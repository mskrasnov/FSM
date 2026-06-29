<div align="center">
  <img src="https://raw.githubusercontent.com/mskrasnov/FSM/refs/heads/master/ferrix-app/data/com.mskrasnov.Ferrix.svg" width="200">
  <h1>Ferrix System Monitor — Swiss Knife for Linux Hardware Diagnostics</h1>
  <p><b>A modern program for getting information about computer hardware and installed software.</b></p>

  [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/) [![Iced](https://img.shields.io/badge/Made%20with-iced-blue?logo=iced)](https://iced.rs) [![GitHub Release](https://img.shields.io/github/v/release/mskrasnov/ferrix?logo=github)](https://github.com/mskrasnov/ferrix/releases) [![Star this repo!](https://img.shields.io/github/stars/mskrasnov/fsm?style=social)](https://github.com/mskrasnov/FSM/stargazers)

  <img src="https://mskrasnov.github.io/ferrix/screens/sysmon-new.png"> <img src="https://mskrasnov.github.io/ferrix/screens/firmware.png">
  <small><a href="https://mskrasnov.github.io/ferrix/gallery.html">Other screenshots</a></small>
</div>

## What is FSM?

FSM is a modern system profiler. Is a program for obtaining information about computer hardware and software. It is designed to work in modern GNU/Linux systems.

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

**TODO:**

- [ ] More information about environment (name and version of DE, WM, DM);
- [ ] Information about media: name of video- and soundcard, information about Pulseaudio/PipeWire and Xorg/Wayland;
- [ ] I/O utilization charts;
- [ ] Process monitor;
- [ ] More information about network;
- [ ] Sensors information;

## Difference from analogues

| Criteria              | Ferrix System Monitor | Hardinfo2 | Stacer/Nexis |
|-----------------------|-----------------------|-----------|--------------|
| Programming language  | Rust                  | C         | C++          |
| Program type          | System profiler       | System profiler & hardware benchmark | System optimizer and monitor |
| GUI                   | `iced`                | GTK3      | Qt5/Qt6      |
| License               | GNU GPLv3             | GNU GPLv3 | GNU GPLv3    |
| Key features          | <ul><li><b>Deep system analisys:</b> systemd services, installed packages, DMI, EDID, etc.;</li><li>Beautiful CPU and RAM utilization charts;</li><li>Simple and clean UI;</li></ul> | <ul><li><b>Hardware benchmarks:</b> CPU, GPU, disks, memory;</li><li>Hardware ratings;</li><li>Export data to HTML/plain text;</li></ul> | <ul><li>System cleaner (caches, logs, packages);</li><li>Real-time resource monitoring;</li><li><tt>systemd</tt>-services management;</li></ul> |
| Data accuracy         | 🟢️ | ⚪️ ([incorrect battery information](https://raw.githubusercontent.com/mskrasnov/mskrasnov.github.io/refs/heads/master/ferrix/assets/hardinfo2.png)) | ⚪️ (stacer is outdated software) |
| Target audience       | Advanced users who need detailed information about software and hardware | Enthusiasts and overlockers interested in benchmarks and system comparisons | Beginners and regular users who want a simple tool for configuring and cleaning up their system |
| Processor topology    | 🟢️                    | 🟢️        | 🔴️           |
| Processor frequencies | 🟢️                    | ⚪️        | ?            |
| Processor vulnerabilities | 🟢️                | 🟢️        | 🔴️           |
| Real-time monitoring  | 🟢️                    | ⚪️        | 🟢️           |
| Hardware info panel   | 🟢️                    | 🟢️        | 🟢️           |
| Battery health        | 🟢️                    | 🔴️        | 🟢️           |
| systemd services list | 🟢️                    | 🔴️        | 🟢️           |
| DMI Tables            | ⚪️ (more data than Hardinfo) | ⚪️ (less data than FSM) | 🔴️ |
| UEFI Settings         | 🟢️                    | 🔴️        | 🔴️           |
| Official AppImage builds | 🟢️                 | 🔴️        | 🟢️           |

- 🟢️ - yes;
- 🔴️ - no;
- ⚪️ - partial;

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
cargo build [--release] --target={i686/aarch64}-unknown-linux-gnu
# or:
make TARGET={i686/aarch64}-unknown-linux-gnu build
```

### Docker build

Prepare:

```bash
docker build -t fsm-builder .
```

Build `*.deb` packages for `amd64`, `i686` and `aarch64` targets:

```bash
docker run --rm            \
    -v "${PWD}:/workspace" \
    fsm-builder            \
    packaging/debian/build.sh
```

Build `*.AppImage` package (only for `amd64` target):

```bash
docker run --rm            \
    -v "${PWD}:/workspace" \
    fsm-builder            \
    packaging/debian/appimage.sh
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
