#!/bin/bash -e
# Build FSM from source
# (C) 2026 Michail Krasnov <mskrasnov07@ya.ru>

echo -e "\e[1;32m[debian/build.sh] Start building...\e[0m"

export PATH=${PATH}:${HOME}/.cargo/bin/

for arch in "x86_64"  "aarch64" "i686"; do
  echo -e "\e[1;32m[debian/build.sh] Build for\e[0m ${arch}\e[1;32m architecture...\e[0m"
  make TARGET="${arch}-unknown-linux-gnu" deb
done

mkdir -pv ./builds/
cp -v ./target/debian/*.deb ./builds/

echo -e "\e[1;32m[debian/build.sh] OK\e[0m"
