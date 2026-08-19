#!/bin/bash -e
# Setup build environment on Debian 12
# (C) 2026 Michail Krasnov <mskrasnov07@ya.ru>

if [ -f ~/.fsm_nosetup ]; then
    echo -e "\e[1;32m[debian/setup.sh] OK\e[0m"
    exit 0
fi

apt update
apt install -y \
    build-essential \
    clang \
    pkg-config \
    git \
    curl \
    ca-certificates \
    libglib2.0-dev \
    libgtk-3-dev \
    libdbus-1-dev \
    libfontconfig1-dev \
    libfreetype6-dev \
    libxkbcommon-dev \
    libwayland-dev \
    libx11-dev \
    libxcb1-dev \
    libxrandr-dev \
    libxi-dev \
    libxinerama-dev \
    libxcursor-dev \
    libxext-dev \
    libssl-dev \
    unzip \
    libfuse2 \
    wget

dpkg --add-architecture arm64
dpkg --add-architecture i686

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
source ${HOME}/.cargo/env

rustup default stable

# some checks...
cargo --version
rustc --version

cargo install cargo-deb

apt install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu libc6-dev-arm64-cross -y
rustup target add aarch64-unknown-linux-gnu

apt install gcc-10-i686-linux-gnu binutils-i686-linux-gnu binutils-i686-gnu gcc-i686-linux-gnu -y
rustup target add i686-unknown-linux-gnu

wget -O appimage-builder-x86_64.AppImage https://github.com/AppImageCrafters/appimage-builder/releases/download/v1.1.0/appimage-builder-1.1.0-x86_64.AppImage
chmod +x appimage-builder-x86_64.AppImage

mv -v appimage-builder-x86_64.AppImage /usr/local/bin/appimage-builder

echo -e "\e[1;32m[debian/setup.sh] OK\e[0m"

touch ~/.fsm_nosetup
