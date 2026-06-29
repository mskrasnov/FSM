#!/bin/bash -e
# Build FSM AppImage Package
# (C) 2026 Michail Krasnov <mskrasnov07@ya.ru>

function prepare() {
    if [ -f ~/.fsm_noappimage ]; then
        return 0
    fi

    echo -e "\e[1;32m[debian/appimage.sh] Some preparations...\e[0m"

    wget -O appimage-builder-x86_64.AppImage https://github.com/AppImageCrafters/appimage-builder/releases/download/v1.1.0/appimage-builder-1.1.0-x86_64.AppImage
    chmod +x appimage-builder-x86_64.AppImage

    mv -v appimage-builder-x86_64.AppImage /usr/local/bin/appimage-builder

    touch ~/.fsm_noappimage
}

prepare

export PATH=${PATH}:${HOME}/.cargo/bin/

echo -e "\e[1;32m[debian/appimage.sh] Start building...\e[0m"

make appimage
make DESTDIR=${PWD}/AppDir install

echo -e "\e[1;32m[debian/appimage.sh] Generate appimage...\e[0m"

appimage-builder --recipe ./AppImageBuilder.yml

mv -v *.AppImage   ./builds/
mv -v *.AppImage.* ./builds/

echo -e "\e[1;32m[debian/appimage.sh] OK...\e[0m"
