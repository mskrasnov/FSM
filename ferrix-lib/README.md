# ferrix-lib

> **NOTE 1:** visit our [GitHub repository](https://github.com/mskrasnov/fsm) to get more information about this crate.
>
> **NOTE 2:** this crate is a part of [ferrix-app](https://crates.io/crates/ferrix-app) crate.

Crate to get information about PC's hardware and software. Only for Linux. Some features are requires `d-bus` and `systemd`. Supported features: get information about:

- CPU (`/proc/cpuinfo`);
- RAM (`/proc/meminfo`) and swaps (`/proc/swaps`);
- Linux kernel information (version, architecture, cmdline);
- Kernel modules list;
- Users and groups;
- Environment variables for current user;
- `systemd` services;
- Infrormation from DMI tables (BIOS, motherboard, chassis/enclosure, processor, RAM);
- Information from EDID (basic info);
- Supported resolutions for monitor;
- Hardware resources;
- UEFI Settings;
- Mounted partitions;
- Installed software (`deb`, `rpm` packages);
- Network statistics (RX/TX Bytes, Packets, Errors, Drops);
- Command interpreter (name, version, path);
- Desktop environment name and version;
- Notebook battery;

TODO:

- [ ] Get more info from EDID;
- [ ] Get information about installed software (`flatpak`, `deb`, `rpm`);
- [X] Get information about notebook battery;
- [ ] Get information about audio;
- [ ] Get information about GUI (desktop environment, session type (Wayland or X.org), etc.);
- [ ] Backup and reset `gsettings` settins;

## Features

Now, `ferrix-lib` has a modular structure in which each module depends on a specific enabled feature. Since `ferrix-lib` is part of the [FSM](https://mskrasnov.github.io/fsm/) project by default, all features are enabled by default (`features.default`).

Features list:

- `battery`;
- `cpu`;
- `cpu_freq`;
- `desktop`;
- `dmi`;
- `drm`;
- `firmware`;
- `init` (DBus is needed);
- `mem`;
- `net`;
- `parts` (`glibc` is needed);
- `resources`;
- `soft`;
- `sys`;
- `vulnerabilities`;

Using `ferrix-lib` with specific features:

```bash
cargo add ferrix-lib    \
  --no-default-features \
  --features=feature1,feature2,...,featuren
```

## License

`ferrix-lib` is distributed under the GNU GPL v3 license.
