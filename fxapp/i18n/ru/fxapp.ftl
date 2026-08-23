# Ferrix Russian translation
# (C) 2025-2026 Michail Krasnov <mskrasnov07@ya.ru>

# SIDEBAR
sidebar-export = Экспорт
sidebar-settings = Настройки
sidebar-about = О программе
sidebar-basic = Основное
sidebar-hardware = Оборудование
sidebar-network = Сеть
sidebar-admin = Администрирование
sidebar-system = Система
sidebar-manage = Обслуживание

# PAGES
page-dashboard = Паспорт системы
page-procs = Процессоры
page-cpufreq = Частоты ЦП
page-vuln = Уязвимости ЦП
page-memory = Память
page-fsystems = Файловые системы
page-net = Сетевые интерфейсы
page-nstat = Статистика сети
page-dmi = Таблицы DMI
page-battery = Аккумулятор
page-screen = Дисплеи
page-distro = Дистрибутив
page-users = Пользователи
page-groups = Группы
page-sysmgr = Системный менеджер
page-sysmon = Системный монитор
page-software = Установленное ПО
page-env = Окружение
page-sensors = Сенсоры
page-kernel = Ядро
page-kmods = Модули ядра
page-frmwr = Прошивка
page-dev = Разработка
page-sysmisc = Разное
page-settings = Настройки
page-about = О программе
page-export = Мастер экспорта
page-todo = Не реализованный функционал

page-todo-msg = Этот функционал пока не реализован

# ABOUT PAGE
about-hdr = FSM — ещё один системный профайлер для Linux
about-ferrix = Версия Ferrix System Monitor
about-flib = Версия ferrix-lib
about-sum = Общая информация
about-author-hdr = Автор:
about-feedback-hdr = Фидбек:
about-source-hdr = Исходный код:
about-blog = Блог:
about-author = (C) 2025, 2026 Михаил Краснов
about-donate = Вы можете отправить мне донат на карту: 2202 2062 5233 5406 (Сбер; Россия). Спасибо!
about-donate-lbl = Закиньте донат на мой Boosty!
about-support = Поддержать меня!

# BATTERY PAGE
bat-header = Аккумулятор {$name}
bat-unknown-name = <неизвестное имя>
bat-chtypes = Типы зарядки
bat-status = Статус
bat-status-ful = Заряжен полностью
bat-status-dis = Разряжается
bat-status-cha = Заряжается
bat-status-noc = Не заряжается
bat-status-non = None
bat-status-unknown = Неизвестно ({$status})
bat-status-isnpresent = Статус не указан!
bat-capacity = Уровень заряда
bat-estimated = Оставшееся время
bat-es-hours = час.
bat-lvl-ful = Заряжен полностью
bat-lvl-nor = Нормальный заряд
bat-lvl-hig = Высокий заряд
bat-lvl-low = Низкий заряд
bat-lvl-cri = Критический заряд!
bat-lvl-non = None
bat-lvl-unk = Неизвестно ({$lbl})
bat-health = Уровень здоровья, %
bat-tech = Технология
bat-cycle-cnt = Количество циклов
bat-volt-min-des = Мин. проектное напряжение, В
bat-volt-now = Текущее напряжение
bat-power-now = Текущая мощность
bat-energy-full-des = Полная проектная энергия, Вт/ч
bat-energy-full = Полная энергия, Вт/ч
bat-energy-now = Текущая энергия, Вт/ч
bat-model = Модель аккумулятора
bat-manufact = Производитель
bat-serial = Серийный номер
bat-not-found = Отсутствуют подключенные аккумуляторы

# TABLE HEADERS
hdr-param = Параметр
hdr-value = Значение

# Boolean values
bool-true = ДА
bool-false = НЕТ

# LOADING PAGE
ldr-page-tooltip = Загрузка данных...

# ERROR PAGE
err-page-tooltip = Ошибка загрузки данных!
err-page-update = Обновить
err-page-backend = Ответ от бекенда (нажать сюда, чтобы скопировать):

# CPU PAGE
cpu-vendor = Производитель
cpu-family = Семейство
cpu-model = Модель
cpu-stepping = Stepping
cpu-microcode = Микрокод
cpu-freq = Частота
cpu-cache = Размер кеша L3
cpu-physical-id = Физический ID
cpu-siblings = Siblings
cpu-core-id = ID ядра
cpu-cpu-cores = Число ядер
cpu-apicid = APIC ID
cpu-iapicid = Initial APIC ID
cpu-fpu = FPU
cpu-fpu-e = FPU Exception
cpu-cpuid-lvl = CPUID Level
cpu-wp = WP
cpu-flags = Флаги
cpu-bugs = Баги
cpu-bogomips = BogoMIPS
cpu-clflush = Размер clflush
cpu-cache-align = Выравнивание кеша
cpu-address-size = Размер адресов
cpu-power = Управление питанием
cpu-processor_no = Процессор №{$proc_no}
cpu-impl = Разработчик ЦП
cpu-arch = Архитектура
cpu-var = Вариант
cpu-part = Тип ядра
cpu-rev = Ревизия
cpu-see-freq = См. страницу «Частоты ЦП»

# CPU FREQUENCY PAGE
cpufreq-tboost = Поддержка CPU Turbo Boost
cpufreq-flist = Список частот ЦП
cpufreq-notfound = Отсутствует информация о частотах ЦП.
cpufreq-summary = Обзор
cpufreq-bios-limit = Лимит BIOS
cpufreq-cpb = Core Performance Boost
cpufreq-cpu_max_freq = Макс. аппаратная частота
cpufreq-cpu_min_freq = Мин. аппаратная частота
cpufreq-scaling_min = Мин. текущая частота
cpufreq-scaling_max = Мкс. текущая частота
cpufreq-scaling_cur = Текущая частота
cpufreq-scaling_gov = Регулятор
cpufreq-avail_gov = Доступные регуляторы
cpufreq-avail_freq = Доступные частоты
cpufreq-scaling_drv = Драйвер
cpufreq-trans_lat = Задержка переключения
cpufreq-set_speed = Заданная частота

# DASHBOARD PAGE
dash-proc = Процессор
dash-mem = Память
dash-sys = Система
dash-host = Имя хоста
dash-proc-info = {$name}{$threads} потоков
dash-mem-used = Использ.: {$used}
dash-mem-total = Всего: {$total}
dash-proc-usage = Нагрузка на ЦП
dash-proc-usg_label = Общая: {$usage}%
dash-swap = Подкачка
dash-bat = Аккумулятор
dash-unk-bat = Без имени
dash-root-part = Корневой раздел
dash-home-part = Домашний раздел
dash-unk-part = Неизвестный раздел

# DISTRO PAGE
distro-name = Имя ОС
distro-id = Идентификатор
distro-like = Дериватив от
distro-cpe = Имя CPE
distro-variant = Редакция/вариант
distro-version = Версия
distro-codename = Кодовое имя
distro-build-id = ID сборки
distro-image-id = ID образа
distro-image-ver = Версия образа
distro-homepage = Домашняя страница
distro-docs = Документация
distro-support = Поддержка
distro-bugtracker = Багтрекер
distro-privacy-policy = Политика конфиденциальности
distro-logo = Логотип
distro-def-host = Стандартное имя хоста
distro-sysext-lvl = Уровень поддержки расширений

# DRM PAGE
drm-fpanel = Видеоустройства
drm-title = Экран №{$idx}
drm-summary = Общая информация
drm-vparams = Параметры видео
drm-edid-not-found = Данные EDID для экрана №{$idx} не существуют!
drm-not-enabled = Экран №{$idx} выключен!
drm-modes = Поддерживаемые разрешения
drm-mode = Разрешение
drm-manufacturer = Производитель
drm-pcode = Код продукта
drm-snum = Серийный номер
drm-date = Неделя/Год
drm-edid-ver = Версия EDID
drm-edid-rev = Ревизия EDID
drm-size = Размер экрана, см
drm-gamma = Гамма дисплея (стандартная)
drm-signal = Тип сигнала
drm-digital = Цифровой
drm-analog = Аналоговый
drm-bit-depth = Глубина цвета
drm-interface = Видеоинтерфейс
drm-is-empty = Экраны не найдены
drm-disabled = <отключен>
drm-unknown = <неизвестен>
drm-model = Модель
drm-resol = Разрешение, макс.
drm-aspratio = Соотношение сторон
drm-diag = Диагональ
drm-pixclck = Пиксельная частота
drm-extblcks = Блоки расширения
drm-cksum = Контрольная сумма
drm-edid-raw = Необработанные данные EDID
drm-no-dtb = Нет блоков DTB
drm-h-active = Горизонтальное разрешение
drm-v-active = Вертикальное разрешение
drm-h-blanking = Горизонтальный интервал гашения
drm-v-blanking = Вертикальный интервал гашения
drm-h-front-porch = Горизонтальная передняя площадка
drm-h-sync-pulse = Ширина горизонтального синхроимпульса
drm-v-front-porch = Вертикальная передняя площадка
drm-v-sync-pulse = Ширина вертикального синхроимпульса
drm-h-back-porch = Горизонтальная задняя площадка
drm-v-back-porch = Вертикальная задняя площадка
drm-h-sync-pos = Горизонтальный синхроимпульс имеет положительную полярность
drm-v-sync-pos = Вертикальный синхроимпульс имеет положительную полярность
drm-dtdb = Блок подробного описания таймингов #{$idx}
drm-rl = Ограничения диапазона
drm-no-rl = Ограничения диапазона не найдены
drm-min-v-freq = Минимальная частота вертикальной развёртки
drm-max-v-freq = Максимальная частота вертикальной развёртки
drm-min-h-freq = Минимальная частота строчной развёртки
drm-max-h-freq = Максимальная частота строчной развёртки
drm-max-pixclck = Максимально поддерживаемая пиксельная частота

# GROUPS PAGE
groups-group = Группа №{$group_no}
groups-name = Имя группы
groups-id = ID группы
groups-members = Члены группы

# KERNEL PAGE
kmod-name = Имя
kmod-size = Размер
kmod-instances = Экз.
kmod-depends = Зависимости
kmod-state = Состояние
kmod-addrs = Адреса
kernel-summary = Обзор
kernel-cmdline = Командная строка
kernel-arch = Архитектура
kernel-version = Версия
kernel-build = Сборка
kernel-pid-max = Макс. число процессов
kernel-threads-max = Макс. число потоков
kernel-user-evs = Макс. число user events
kernel-avail-enthropy = Доступная энтропия
kernel-mods-hdr = Загруженные модули ядра
kernel-mods-is-empty = Модули ядра не загружены

# FIRMWARE PAGE
frmwr-name = Имя
frmwr-val = Значение
frmwr-pval = Возможные значения
frmwr-type = Тип
frmwr-drv = Драйвер
frmwr-gen = Общее
frmwr-params = Параметры

# NETWORK PAGE
net-adp = Интерфейс: {$adp}
net-os = Состояние
net-addr = MAC адрес
net-bcast = Broadcast
net-mtu = MTU
net-int = Интерфейс

# RAM PAGE
ram-total = Всего
ram-free = Свободно
ram-available = Доступно
ram-buffers = Буферы
ram-cached = В кеше
ram-swap-cached = В кеше подкачки
ram-active = Активная
ram-inactive = Неактивная
ram-active-anon = Активная (анонимные)
ram-inactive-anon = Неактивная (анонимные)
ram-active-file = Активная (файл)
ram-inactive-file = Неактивная (файл)
ram-unevictable = Невыгружаемая
ram-locked = Заблокированная
ram-swap-total = Подкачки всего
ram-swap-free = Подкачки свободно
ram-zswap = ZSwap всего
ram-zswapped = ZSwap'пировано
ram-dirty = Грязные страницы
ram-writeback = Активно пишется на диск
ram-anon-pages = Анонимные страницы
ram-mapped = Отображённая память
ram-shmem = Разделяемая память
ram-kreclaimable = Восстанавливаемая ядром
ram-slab = slab
ram-sreclaimable = Востанавливаемый slab
ram-sunreclaim = Невостанавливаемый slab
ram-kernel-stack = Стек ядра
ram-page-tables = Таблицы страниц
ram-sec-page-tables = Доп. таблицы страниц
ram-nfs-unstable = Нестабильный NFS
ram-bounce = Bounce буферы
ram-writeback-tmp = Временные буферы (для FUSE)
ram-commit-limit = Можно выделить (max.)
ram-swp = Подкачка {$name}
ram-swp-size = Объём
ram-swp-used = Занятый объём
ram-swp-prior = Приоритет
ram-hdr = Общее
ram-swp-hdr = Подкачка
ram-swp-not-found = Не найдены файлы/разделы подкачки.

# SETTINGS PAGE
settings-update-period = Период обновления
settings-uperiod-tip = Укажите период обновления данных (в сек.). Чем выше период обновления, тем ниже нагрузка на ПК.
settings-uper-main = Основные данные
settings-look = Оформление программы
settings-look-tip = Стиль оформления влияет на цвета интерфейса и шрифта. Выберите то, что нравится вам.
settings-look-thick = Толщина линий графика, пикс.
settings-look-select = Стиль оформления
settings-save = Сохранить

# STORAGES PAGE
storage-dev = Устройство
storage-fs = ФС
storage-total = Всего
storage-free = Свободно
storage-used = Использ.
storage-usage = Использование

# STYLE LABELS
style-dark = Тёмный
style-light = Светлый

# SYSTEM MISC PAGE
misc-hostname = Имя хоста
misc-loadavg = Средняя нагрузка
misc-uptime = Время работы
misc-uptime-val = работы: {$up}, простоя: {$down}
misc-de = Рабочее окружение
misc-lang = Язык
misc-user = Текущий пользователь
misc-shell = Командный интерпретатор

# SYSTEM MONITOR PAGE
sysmon-x-axis = Число отсчётов по оси X:
sysmon-toggle = Показывать легенду
sysmon-cpu-hdr = Использование ЦП
sysmon-ram-hdr = Использование ОЗУ
sysmon-cpu-unk = Статистика использования ЦП неизвестна!
sysmon-cpu-brk = Статистика использования ЦП повреждена!

# SYSTEMD PAGE
sysd-hdr-name = Имя
sysd-hdr-descr = Описание
sysd-hdr-load = Загружен
sysd-hdr-actv = Активен
sysd-hdr-work = Работает
sysd-total = Всего сервисов: {$total}
sysd-btime = Загрузка завершена за {$firm} (прошивка) + {$ldr} (загрузчик) + {$krn} (ядро) + {$uspc} (польз. простр-во)
sysd-btime-ldng = Время загрузки: вычисление...
sysd-btime-err = Время загрузки: ошибка: {$err}
sysd-btime-unk = Время загрузки: неизвестно

# SOFTWARE PAGE
soft-hdr-name = Имя
soft-hdr-ver = Версия
soft-hdr-arch = Архитектура
soft-hdr-type = Тип
soft-total = Всего пакетов: {$total}

# USERS PAGE
users-name = Имя пользователя
users-id = ID пользователя
users-gid = ID группы
users-gecos = GECOS
users-home = Домашний каталог
users-shell = Оболочка входа
users-hdr = Пользователь №{$id}

# CPU VULNERABILITY PAGE
vuln-hdr-name = Имя
vuln-hdr-descr = Описание

# LINE THICKNESS LABELS
lthick-one = Один
lthick-two = Два

# EXPORT PAGE
export-fmt-lbl = Формат экспорта
export-data-lbl = Тип экспортируемых данных
export-btn = Экспорт
export-data-list = Выберите необходимые данные для экспорта
export-st-pending = Статус: Ожидание...
export-st-load = Статус: Загрузка данных...
export-st-lerr = Статус: Ошибка загрузки: {$err}
export-st-ser = Статус: Обработка данных...
export-st-serr = Статус: Ошибка обработки: {$err}
export-st-wr = Статус: Запись данных...
export-st-werr = Статус: Ошибка записи: {$err}
