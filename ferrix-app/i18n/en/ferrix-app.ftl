# Ferrix English translation
# (C) 2025-2026 Michail Krasnov <mskrasnov07@ya.ru>

# SIDEBAR
sidebar-export = Export
sidebar-settings = Settings
sidebar-about = About
sidebar-basic = Basic
sidebar-hardware = Hardware
sidebar-admin = Administration
sidebar-system = System
sidebar-manage = Management

# PAGES
page-dashboard = Dashboard
page-procs = Processors
page-cpufreq = CPU Frequencies
page-vuln = CPU Vulnerabilities
page-memory = Memory
page-fsystems = Filesystems
page-net = Network Interfaces
page-nstat = Network Statistics
page-dmi = DMI Tables
page-battery = Battery
page-screen = Screen
page-distro = Distro
page-users = Users
page-groups = Groups
page-sysmgr = System Manager
page-sysmon = System Monitor
page-software = Installed software
page-env = Environment
page-sensors = Sensors
page-kernel = Kernel
page-kmods = Kernel Modules
page-frmwr = Firmware
page-dev = Development
page-sysmisc = Misc
page-settings = Settings
page-about = About
page-export = Export Manager
page-todo = Not implemented functionality

page-todo-msg = This functionality has not been implemented yet

# ABOUT PAGE
about-hdr = FSM — yet another system profiler for Linux
about-ferrix = Ferrix System Monitor version
about-flib = ferrix-lib version
about-sum = Summary
about-author-hdr = Author:
about-feedback-hdr = Feedback:
about-source-hdr = Source code:
about-blog = Blog:
about-author = (C) 2025, 2026 Michail Krasnov
about-donate = Can you support me?
about-donate-lbl = Donate to me on Boosty!
about-support = Support me!

# BATTERY PAGE
bat-header = Battery {$name}
bat-unknown-name = <unknown name>
bat-chtypes = Charge Types
bat-status = Status
bat-status-ful = Full
bat-status-dis = Discharging
bat-status-cha = Charging
bat-status-noc = Not charging
bat-status-non = None
bat-status-unknown = Unknown ({$status})
bat-status-isnpresent = Status is not present!
bat-capacity = Capacity Level
bat-estimated = Estimated Time
bat-es-hours = hours
bat-lvl-ful = Full
bat-lvl-nor = Normal
bat-lvl-hig = High
bat-lvl-low = Low
bat-lvl-cri = Critical!
bat-lvl-non = None
bat-lvl-unk = Unknown ({$lbl})
bat-health = Health level, %
bat-tech = Technology
bat-cycle-cnt = Cycle count
bat-volt-min-des = Minimal designed voltage, V
bat-volt-now = Current voltage, V
bat-power-now = Current power
bat-energy-full-des = Full designed energy, Wh
bat-energy-full = Full energy, Wh
bat-energy-now = Current energy, Wh
bat-model = Battery model
bat-manufact = Manufacturer
bat-serial = Serial number
bat-not-found = There are no connected batteries

# TABLE HEADERS
hdr-param = Parameter
hdr-value = Value

# Boolean values
bool-true = YES
bool-false = NO

# LOADING PAGE
ldr-page-tooltip = Loading data...

# ERROR PAGE
err-page-tooltip = Data loading error!

# CPU PAGE
cpu-vendor = Vendor
cpu-family = Family
cpu-model = Model
cpu-stepping = Stepping
cpu-microcode = Microcode
cpu-freq = Frequency
cpu-cache = L3 Cache Size
cpu-physical-id = Physical ID
cpu-siblings = Siblings
cpu-core-id = Core ID
cpu-cpu-cores = CPU Cores Count
cpu-apicid = APIC ID
cpu-iapicid = Initial APIC ID
cpu-fpu = FPU
cpu-fpu-e = FPU Exception
cpu-cpuid-lvl = CPUID Level
cpu-wp = WP
cpu-flags = Flags
cpu-bugs = Bugs
cpu-bogomips = BogoMIPS
cpu-clflush = clflush size
cpu-cache-align = Cache alignment
cpu-address-size = Addresses sizes
cpu-power = Power management
cpu-processor_no = Processor #{$proc_no}
cpu-impl = CPU Implementer
cpu-arch = Architecture
cpu-var = Variant
cpu-part = Part
cpu-rev = Revision
cpu-see-freq = See "CPU Frequencies" page

# CPU FREQUENCY PAGE
cpufreq-tboost = CPU Turbo Boost support
cpufreq-flist = CPU Frequency List
cpufreq-notfound = No CPU policy list found.
cpufreq-sum = CPU #{$cpu} frequency
cpufreq-summary = Summary
cpufreq-bios-limit = BIOS Limit
cpufreq-cpb = Core Performance Boost
cpufreq-cpu_max_freq = Hardware max. frequency
cpufreq-cpu_min_freq = Hardware min. frequency
cpufreq-scaling_min = Scaling min.
cpufreq-scaling_max = Scaling max.
cpufreq-scaling_cur = Current Frequency
cpufreq-scaling_gov = Governor
cpufreq-avail_gov = Available Governors
cpufreq-avail_freq = Available Frequencies
cpufreq-scaling_drv = Scaling Driver
cpufreq-trans_lat = Transition Latency
cpufreq-set_speed = Set Speed
cpufreq-policy = Frequency Policy for CPU #{$cpu}

# DASHBOARD PAGE
dash-proc = Processor
dash-mem = Memory
dash-sys = System
dash-host = Hostname
dash-proc-info = {$name}{$threads} threads
dash-mem-used = Used: {$used}
dash-mem-total = Total: {$total}
dash-proc-usage = CPU Usage
dash-proc-usg_label = Total usage: {$usage}%
dash-swap = Swap
dash-bat = Battery
dash-unk-bat = No name
dash-root-part = Root Partition
dash-home-part = Home Partition
dash-unk-part = Unknown Partition

# DISTRO PAGE
distro-name = OS Name
distro-id = ID
distro-like = Derivative from
distro-cpe = CPE Name
distro-variant = Revision/Variant
distro-version = Version
distro-codename = Codename
distro-build-id = Build ID
distro-image-id = Image ID
distro-image-ver = Image version
distro-homepage = Homepage
distro-docs = Documentation
distro-support = Support
distro-bugtracker = Bugtracker
distro-privacy-policy = Privacy policy
distro-logo = Logo
distro-def-host = Default hostname
distro-sysext-lvl = System extensions level

# DRM PAGE
drm-fpanel = Video devices
drm-title = Screen #{$idx}
drm-summary = Summary
drm-vparams = Video params
drm-edid-not-found = EDID data for screen #{$idx} not found!
drm-not-enabled = Screen #{$idx} isn't enabled!
drm-modes = Support modes
drm-mode = Mode
drm-manufacturer = Manufacturer
drm-pcode = Product code
drm-snum = Serial number
drm-date = Week/Year
drm-edid-ver = EDID Version
drm-edid-rev = EDID Revision
drm-size = Screen size, cm
drm-gamma = Display gamma (default)
drm-signal = Signal type
drm-digital = Digital
drm-analog = Analogue
drm-bit-depth = Bit depth
drm-interface = Video interface
drm-is-empty = Screens not found
drm-disabled = <disabled>
drm-unknown = <unknown>

# GROUPS PAGE
groups-group = Group #{$group_no}
groups-name = Group name
groups-id = Group ID
groups-members = Group members

# KERNEL PAGE
kmod-name = Name
kmod-size = Size
kmod-instances = Inst.
kmod-depends = Dependencies
kmod-state = State
kmod-addrs = Addresses
kernel-summary = Summary
kernel-cmdline = Command line
kernel-arch = Architecture
kernel-version = Version
kernel-build = Build
kernel-pid-max = Processes, max.
kernel-threads-max = Threads, max.
kernel-user-evs = User events, max.
kernel-avail-enthropy = Available enthropy
kernel-mods-hdr = Loaded kernel modules
kernel-mods-is-empty = Kernel modules are not loaded

# FIRMWARE PAGE
frmwr-name = Name
frmwr-val = Value
frmwr-pval = Possible values
frmwr-type = Type
frmwr-drv = Driver
frmwr-gen = General
frmwr-params = Parameters

# NETWORK PAGE
net-adp = Interface: {$adp}
net-os = Operstate
net-addr = MAC Address
net-bcast = Broadcast
net-mtu = MTU
net-int = Interface

# RAM PAGE
ram-total = Total
ram-free = Free
ram-available = Available
ram-buffers = Buffers
ram-cached = Cached
ram-swap-cached = Swap cached
ram-active = Active
ram-inactive = Inactive
ram-active-anon = Active (anon)
ram-inactive-anon = Inactive (anon)
ram-active-file = Active (file)
ram-inactive-file = Inactive (file)
ram-unevictable = Unevictable
ram-locked = Locked
ram-swap-total = Total swap
ram-swap-free = Swap free
ram-zswap = Total ZSwap
ram-zswapped = ZSwapped
ram-dirty = Dirty pages
ram-writeback = Writeback
ram-anon-pages = Anon pages
ram-mapped = Mapped memory
ram-shmem = Shared memory
ram-kreclaimable = Kernel reclaimable
ram-slab = slab
ram-sreclaimable = slab reclaimable
ram-sunreclaim = slab unreclaimable
ram-kernel-stack = Kernel stack
ram-page-tables = Page tables
ram-sec-page-tables = Secondary page tables
ram-nfs-unstable = NFS Unstable
ram-bounce = Bounce buffers
ram-writeback-tmp = Temporary buffers (for FUSE)
ram-commit-limit = Commit limit (max.)
ram-swp = Swap {$name}
ram-swp-size = Total size
ram-swp-used = Used
ram-swp-prior = Priority
ram-hdr = RAM Info
ram-swp-hdr = Swaps Info
ram-swp-not-found = No swaps files/partitions found.

# SETTINGS PAGE
settings-update-period = Update period
settings-uperiod-tip = Specify the data update period (in secs). The higher the update period, the lower the load on the PC.
settings-uper-main = Main data
settings-look = Look and feel
settings-look-tip = The design style affects the interface and font colors. Choose what you like.
settings-look-thick = Chart line thickness, px.
settings-look-select = Style
settings-save = Save

# STORAGES PAGE
storage-dev = Device
storage-fs = Filesystem
storage-total = Total
storage-free = Free
storage-used = Used
storage-usage = Usage

# STYLE LABELS
style-dark = Dark
style-light = Light

# SYSTEM MISC PAGE
misc-hostname = Host name
misc-loadavg = Load average
misc-uptime = Uptime
misc-uptime-val = uptime: {$up}, downtime: {$down}
misc-de = Desktop
misc-lang = Language
misc-user = Current user
misc-shell = Command shell

# SYSTEM MONITOR PAGE
sysmon-x-axis = Number of counts on the X-axis:
sysmon-toggle = Show legend
sysmon-cpu-hdr = CPU Usage
sysmon-ram-hdr = RAM Usage
sysmon-cpu-unk = CPU usage statistics are unknown!
sysmon-cpu-brk = CPU usage statistics are broken!

# SYSTEMD PAGE
sysd-hdr-name = Name
sysd-hdr-descr = Description
sysd-hdr-load = Loaded
sysd-hdr-actv = Active
sysd-hdr-work = Work
sysd-total = Total services: {$total}
sysd-btime = Startup finished in {$firm} (firmware) + {$ldr} (loader) + {$krn} (kernel) + {$uspc} (userspace) = {$total}
sysd-btime-ldng = Startup time: loading...
sysd-btime-err = Startup time: error: {$err}
sysd-btime-unk = Startup time: unknown

# SOFTWARE PAGE
soft-hdr-name = Name
soft-hdr-ver = Version
soft-hdr-arch = Arch
soft-hdr-type = Type
soft-total = Total packages: {$total}

# USERS PAGE
users-name = User name
users-id = User ID
users-gid = Group ID
users-gecos = GECOS
users-home = Home directory
users-shell = Login shell
users-hdr = User #{$id}

# CPU VULNERABILITY PAGE
vuln-hdr-name = Name
vuln-hdr-descr = Description

# LINE THICKNESS LABELS
lthick-one = One
lthick-two = Two

# EXPORT PAGE
export-fmt-lbl = Export format
export-data-lbl = Export data type
export-btn = Export
export-data-list = Choose the necessary information to export
export-st-pending = Status: Pending...
export-st-load = Status: Loading data...
export-st-lerr = Status: Loading data error: {$err}
export-st-ser = Status: Processing data...
export-st-serr = Status: Data error: {$err}
export-st-wr = Status: Writing data...
export-st-werr = Status: Write error: {$err}
