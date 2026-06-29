#!/bin/bash -e
# Setup build environment on Fedora 42
# (C) 2026 Michail Krasnov <mskrasnov07@ya.ru>

if [ -f ~/.fsm_nosetup.rpm ]; then
    echo -e "\e[1;32m[fedora/setup.sh] OK\e[0m"
    exit 0
fi

# dnf config-manager --set-disabled fedora-cisco-openh264
dnf config-manager setopt fedora-cisco-openh264.enabled=0
dnf install -y \
    gcc \
    clang \
    make \
    rpm-build \
    rpmdevtools \
    pkg-config \
    openssl-devel \
    dbus-devel \
    gtk3 \
    desktop-file-utils
dnf clean all

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
source ${HOME}/.cargo/env

rustup default stable

# some checks...
cargo --version
rustc --version

cargo install cargo-generate-rpm

echo -e "\e[1;32m[fedora/setup.sh] OK\e[0m"

touch ~/.fsm_nosetup.rpm

