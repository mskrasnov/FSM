# Ferrix German translation
# (C) 2026 Michail Krasnov <mskrasnov07@ya.ru>

# SIDEBAR
sidebar-export = Exportieren
sidebar-settings = Einstellungen
sidebar-about = Über
sidebar-basic = Allgemein
sidebar-hardware = Hardware
sidebar-network = Netzwerk
sidebar-admin = Administration
sidebar-system = System
sidebar-manage = Verwaltung

# PAGES
page-dashboard = Dashboard
page-procs = Prozessoren
page-cpufreq = CPU-Frequenzen
page-vuln = CPU-Schwachstellen
page-memory = Arbeitsspeicher
page-fsystems = Dateisysteme
page-net = Netzwerkschnittstellen
page-nstat = Netzwerkstatistiken
page-dmi = DMI-Tabellen
page-battery = Akku
page-screen = Displays
page-distro = Distribution
page-users = Benutzer
page-groups = Gruppen
page-sysmgr = System-Manager
page-sysmon = Systemmonitor
page-software = Installierte Software
page-env = Umgebung
page-sensors = Sensoren
page-kernel = Kernel
page-kmods = Kernel-Module
page-frmwr = Firmware
page-dev = Entwicklung
page-sysmisc = Verschiedenes
page-settings = Einstellungen
page-about = Über
page-export = Export-Manager
page-todo = Nicht implementierte Funktion

page-todo-msg = Diese Funktion wurde noch nicht implementiert.

# ABOUT PAGE
about-hdr = FSM — ein weiterer Systemprofiler für Linux
about-ferrix = Ferrix-Systemmonitor-Version
about-flib = ferrix-lib-Version
about-sum = Zusammenfassung
about-author-hdr = Autor:
about-feedback-hdr = Feedback:
about-source-hdr = Quellcode:
about-blog = Blog:
about-author = (C) 2025, 2026 Michail Krasnov
about-donate = Können Sie mich unterstützen?
about-donate-lbl = Spenden Sie auf Boosty für mich!
about-support = Unterstützen Sie mich!

# BATTERY PAGE
bat-header = Akku {$name}
bat-unknown-name = <unbekannter Name>
bat-chtypes = Ladetypen
bat-status = Status
bat-status-ful = Voll
bat-status-dis = Entlädt
bat-status-cha = Lädt
bat-status-noc = Lädt nicht
bat-status-non = Keine
bat-status-unknown = Unbekannt ({$status})
bat-status-isnpresent = Status ist nicht vorhanden!
bat-capacity = Kapazitätsstufe
bat-estimated = Geschätzte Zeit
bat-es-hours = Stunden
bat-lvl-ful = Voll
bat-lvl-nor = Normal
bat-lvl-hig = Hoch
bat-lvl-low = Niedrig
bat-lvl-cri = Kritisch!
bat-lvl-non = Keine
bat-lvl-unk = Unbekannt ({$lbl})
bat-health = Gesundheitszustand, %
bat-tech = Technologie
bat-cycle-cnt = Zyklenanzahl
bat-volt-min-des = Minimale Auslegungsspannung, V
bat-volt-now = Aktuelle Spannung, V
bat-power-now = Aktuelle Leistung
bat-energy-full-des = Vollständige Auslegungsenergie, Wh
bat-energy-full = Volle Energie, Wh
bat-energy-now = Aktuelle Energie, Wh
bat-model = Akkumodell
bat-manufact = Hersteller
bat-serial = Seriennummer
bat-not-found = Es sind keine angeschlossenen Akkus vorhanden

# TABLE HEADERS
hdr-param = Parameter
hdr-value = Wert

# Boolean values
bool-true = JA
bool-false = NEIN

# LOADING PAGE
ldr-page-tooltip = Daten werden geladen...

# ERROR PAGE
err-page-tooltip = Fehler beim Laden der Daten!
err-page-update = Aktualisieren
err-page-backend = Backend-Antwort (zum Kopieren hier klicken):

# CPU PAGE
cpu-vendor = Hersteller
cpu-family = Familie
cpu-model = Modell
cpu-stepping = Stepping
cpu-microcode = Mikrocode
cpu-freq = Frequenz
cpu-cache = L3-Cachegröße
cpu-physical-id = Physische ID
cpu-siblings = Geschwister
cpu-core-id = Kern-ID
cpu-cpu-cores = Anzahl der CPU-Kerne
cpu-apicid = APIC-ID
cpu-iapicid = Initiale APIC-ID
cpu-fpu = FPU
cpu-fpu-e = FPU-Ausnahme
cpu-cpuid-lvl = CPUID-Level
cpu-wp = WP
cpu-flags = Flags
cpu-bugs = Bugs
cpu-bogomips = BogoMIPS
cpu-clflush = clflush-Größe
cpu-cache-align = Cache-Ausrichtung
cpu-address-size = Adressgrößen
cpu-power = Energieverwaltung
cpu-processor_no = Prozessor #{$proc_no}
cpu-impl = CPU-Implementierer
cpu-arch = Architektur
cpu-var = Variante
cpu-part = Teil
cpu-rev = Revision
cpu-see-freq = Siehe Seite „CPU-Frequenzen“

# CPU FREQUENCY PAGE
cpufreq-tboost = CPU-Turbo-Boost-Unterstützung
cpufreq-flist = CPU-Frequenzliste
cpufreq-notfound = Keine CPU-Richtlinienliste gefunden.
cpufreq-summary = Zusammenfassung
cpufreq-bios-limit = BIOS-Limit
cpufreq-cpb = Core Performance Boost
cpufreq-cpu_max_freq = Maximale Hardwarefrequenz
cpufreq-cpu_min_freq = Minimale Hardwarefrequenz
cpufreq-scaling_min = Skalierung min.
cpufreq-scaling_max = Skalierung max.
cpufreq-scaling_cur = Aktuelle Frequenz
cpufreq-scaling_gov = Governor
cpufreq-avail_gov = Verfügbare Governors
cpufreq-avail_freq = Verfügbare Frequenzen
cpufreq-scaling_drv = Skalierungstreiber
cpufreq-trans_lat = Übergangslatenz
cpufreq-set_speed = Geschwindigkeit festlegen

# DASHBOARD PAGE
dash-proc = Prozessor
dash-mem = Arbeitsspeicher
dash-sys = System
dash-host = Rechnername
dash-proc-info = {$name}{$threads} Threads
dash-mem-used = Belegt: {$used}
dash-mem-total = Gesamt: {$total}
dash-proc-usage = CPU-Auslastung
dash-proc-usg_label = Gesamtauslastung: {$usage}%
dash-swap = Swap
dash-bat = Akku
dash-unk-bat = Kein Name
dash-root-part = Root-Partition
dash-home-part = Home-Partition
dash-unk-part = Unbekannte Partition

# DISTRO PAGE
distro-name = Betriebssystemname
distro-id = ID
distro-like = Abgeleitet von
distro-cpe = CPE-Name
distro-variant = Revision/Variante
distro-version = Version
distro-codename = Codename
distro-build-id = Build-ID
distro-image-id = Image-ID
distro-image-ver = Image-Version
distro-homepage = Homepage
distro-docs = Dokumentation
distro-support = Unterstützung
distro-bugtracker = Bugtracker
distro-privacy-policy = Datenschutzrichtlinie
distro-logo = Logo
distro-def-host = Standard-Rechnername
distro-sysext-lvl = Systemerweiterungsebene

# DRM PAGE
drm-fpanel = Videogeräte
drm-title = Bildschirm #{$idx}
drm-summary = Zusammenfassung
drm-vparams = Videoparameter
drm-edid-not-found = EDID-Daten für Bildschirm #{$idx} nicht gefunden!
drm-not-enabled = Bildschirm #{$idx} ist nicht aktiviert!
drm-modes = Unterstützte Modi
drm-mode = Modus
drm-manufacturer = Hersteller
drm-pcode = Produktcode
drm-snum = Seriennummer
drm-date = Woche/Jahr
drm-edid-ver = EDID-Version
drm-edid-rev = EDID-Revision
drm-size = Bildschirmgröße, cm
drm-gamma = Display-Gamma (Standard)
drm-signal = Signaltyp
drm-digital = Digital
drm-analog = Analog
drm-bit-depth = Farbtiefe
drm-interface = Videoschnittstelle
drm-is-empty = Keine Bildschirme gefunden
drm-disabled = <deaktiviert>
drm-unknown = <unbekannt>
drm-model = Modell
drm-resol = Auflösung, max.
drm-aspratio = Seitenverhältnis
drm-diag = Diagonale
drm-pixclck = Pixeltakt
drm-extblcks = Erweiterungsblöcke
drm-cksum = Prüfsumme
drm-edid-raw = EDID-Rohwert
drm-no-dtb = Keine DTB-Blöcke
drm-h-active = Horizontale Auflösung
drm-v-active = Vertikale Auflösung
drm-h-blanking = Horizontales Austastintervall
drm-v-blanking = Vertikales Austastintervall
drm-h-front-porch = Front-Porch, horizontal
drm-h-sync-pulse = Horizontale Sync-Impulsbreite
drm-v-front-porch = Front-Porch, vertikal
drm-v-sync-pulse = Vertikale Sync-Impulsbreite
drm-h-back-porch = Back-Porch, horizontal
drm-v-back-porch = Back-Porch, vertikal
drm-h-sync-pos = Horizontaler Sync-Impuls ist positiv polarisiert
drm-v-sync-pos = Vertikaler Sync-Impuls ist positiv polarisiert
drm-dtdb = Detaillierter Timing-Descriptor-Block #{$idx}
drm-rl = Bereichsgrenzen
drm-no-rl = Keine Bereichsgrenzen gefunden
drm-min-v-freq = Minimale vertikale Feldfrequenz
drm-max-v-freq = Maximale vertikale Feldfrequenz
drm-min-h-freq = Minimale horizontale Zeilenfrequenz
drm-max-h-freq = Maximale horizontale Zeilenfrequenz
drm-max-pixclck = Maximal unterstützter Pixeltakt

# GROUPS PAGE
groups-group = Gruppe #{$group_no}
groups-name = Gruppenname
groups-id = Gruppen-ID
groups-members = Gruppenmitglieder

# KERNEL PAGE
kmod-name = Name
kmod-size = Größe
kmod-instances = Inst.
kmod-depends = Abhängigkeiten
kmod-state = Status
kmod-addrs = Adressen
kernel-summary = Zusammenfassung
kernel-cmdline = Befehlszeile
kernel-arch = Architektur
kernel-version = Version
kernel-build = Build
kernel-pid-max = Prozesse, max.
kernel-threads-max = Threads, max.
kernel-user-evs = Benutzerereignisse, max.
kernel-avail-enthropy = Verfügbare Entropie
kernel-mods-hdr = Geladene Kernel-Module
kernel-mods-is-empty = Kernel-Module sind nicht geladen

# FIRMWARE PAGE
frmwr-name = Name
frmwr-val = Wert
frmwr-pval = Mögliche Werte
frmwr-type = Typ
frmwr-drv = Treiber
frmwr-gen = Allgemein
frmwr-params = Parameter

# NETWORK PAGE
net-adp = Schnittstelle: {$adp}
net-os = Betriebszustand
net-addr = MAC-Adresse
net-bcast = Broadcast
net-mtu = MTU
net-int = Schnittstelle

# RAM PAGE
ram-total = Gesamt
ram-free = Frei
ram-available = Verfügbar
ram-buffers = Puffer
ram-cached = Zwischengespeichert
ram-swap-cached = Swap-Zwischenspeicher
ram-active = Aktiv
ram-inactive = Inaktiv
ram-active-anon = Aktiv (anonym)
ram-inactive-anon = Inaktiv (anonym)
ram-active-file = Aktiv (Datei)
ram-inactive-file = Inaktiv (Datei)
ram-unevictable = Nicht verdrängbar
ram-locked = Gesperrt
ram-swap-total = Swap gesamt
ram-swap-free = Swap frei
ram-zswap = ZSwap gesamt
ram-zswapped = ZSwapped
ram-dirty = Schmutzige Seiten
ram-writeback = Writeback
ram-anon-pages = Anonyme Seiten
ram-mapped = Zugeordneter Speicher
ram-shmem = Gemeinsamer Speicher
ram-kreclaimable = Vom Kernel zurückforderbar
ram-slab = Slab
ram-sreclaimable = Slab zurückforderbar
ram-sunreclaim = Slab nicht zurückforderbar
ram-kernel-stack = Kernel-Stack
ram-page-tables = Seitentabellen
ram-sec-page-tables = Sekundäre Seitentabellen
ram-nfs-unstable = NFS instabil
ram-bounce = Bounce-Puffer
ram-writeback-tmp = Temporäre Puffer (für FUSE)
ram-commit-limit = Commit-Limit (max.)
ram-swp = Swap {$name}
ram-swp-size = Gesamtgröße
ram-swp-used = Belegt
ram-swp-prior = Priorität
ram-hdr = Allgemeine Informationen
ram-swp-hdr = Swap-Informationen
ram-swp-not-found = Keine Swap-Dateien/-Partitionen gefunden.

# SETTINGS PAGE
settings-update-period = Aktualisierungsintervall
settings-uperiod-tip = Geben Sie das Datenaktualisierungsintervall (in Sekunden) an. Je höher das Intervall, desto geringer die PC-Belastung.
settings-uper-main = Hauptdaten
settings-look = Erscheinungsbild
settings-look-tip = Der Designstil beeinflusst die Farben der Oberfläche und Schrift. Wählen Sie, was Ihnen gefällt.
settings-look-thick = Diagrammlinienstärke, px.
settings-look-select = Stil
settings-save = Speichern

# STORAGES PAGE
storage-dev = Gerät
storage-fs = Dateisystem
storage-total = Gesamt
storage-free = Frei
storage-used = Belegt
storage-usage = Nutzung

# STYLE LABELS
style-dark = Dunkel
style-light = Hell

# SYSTEM MISC PAGE
misc-hostname = Rechnername
misc-loadavg = Durchschnittslast
misc-uptime = Betriebszeit
misc-uptime-val = Betriebszeit: {$up}, Ausfallzeit: {$down}
misc-de = Desktop
misc-lang = Sprache
misc-user = Aktueller Benutzer
misc-shell = Befehls-Shell

# SYSTEM MONITOR PAGE
sysmon-x-axis = Anzahl der Werte auf der X-Achse:
sysmon-toggle = Legende anzeigen
sysmon-cpu-hdr = CPU-Auslastung
sysmon-ram-hdr = RAM-Auslastung
sysmon-cpu-unk = CPU-Auslastungsstatistiken sind unbekannt!
sysmon-cpu-brk = CPU-Auslastungsstatistiken sind fehlerhaft!

# SYSTEMD PAGE
sysd-hdr-name = Name
sysd-hdr-descr = Beschreibung
sysd-hdr-load = Geladen
sysd-hdr-actv = Aktiv
sysd-hdr-work = Arbeit
sysd-total = Dienste gesamt: {$total}
sysd-btime = Start abgeschlossen in {$firm} (Firmware) + {$ldr} (Bootloader) + {$krn} (Kernel) + {$uspc} (Userspace)
sysd-btime-ldng = Startzeit: wird geladen...
sysd-btime-err = Startzeit: Fehler: {$err}
sysd-btime-unk = Startzeit: unbekannt

# SOFTWARE PAGE
soft-hdr-name = Name
soft-hdr-ver = Version
soft-hdr-arch = Architektur
soft-hdr-type = Typ
soft-total = Pakete gesamt: {$total}

# USERS PAGE
users-name = Benutzername
users-id = Benutzer-ID
users-gid = Gruppen-ID
users-gecos = GECOS
users-home = Home-Verzeichnis
users-shell = Login-Shell
users-hdr = Benutzer #{$id}

# CPU VULNERABILITY PAGE
vuln-hdr-name = Name
vuln-hdr-descr = Beschreibung

# LINE THICKNESS LABELS
lthick-one = Eins
lthick-two = Zwei

# EXPORT PAGE
export-fmt-lbl = Exportformat
export-data-lbl = Exportdatentyp
export-btn = Exportieren
export-data-list = Wählen Sie die zu exportierenden Informationen aus
export-st-pending = Status: Ausstehend...
export-st-load = Status: Daten werden geladen...
export-st-lerr = Status: Fehler beim Laden der Daten: {$err}
export-st-ser = Status: Daten werden verarbeitet...
export-st-serr = Status: Datenfehler: {$err}
export-st-wr = Status: Daten werden geschrieben...
export-st-werr = Status: Schreibfehler: {$err}
