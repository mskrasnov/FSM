#!/bin/bash -e
# Build FSM from source
# (C) 2026 Michail Krasnov <mskrasnov07@ya.ru>

echo -e "\e[1;32m[fedora/build.sh] Start building...\e[0m"

export PATH=${PATH}:${HOME}/.cargo/bin/

make build

mkdir -pv ./builds/
cd ferrix-app

cargo generate-rpm --target-dir=../target/
cp -v ../target/generate-rpm/*.rpm ../builds/

cargo clean

echo -e "\e[1;32m[fedora/build.sh] OK\e[0m"

