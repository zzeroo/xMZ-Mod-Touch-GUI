# xMZ-Mod-Touch
## xMesszentrale mit Modbus Interface und Touch Screen

# Installation
## Abhänigkeiten
### libmodbus

    sudo apt-get install autoconf git-core build-essential

    git clone https://github.com/stephane/libmodbus.git --depth=1
    cd libmodbus
    ./autogen.sh
    ./configure --prefix=/usr
    make
    make install

### restliche Bibliotheken

    apt-get install libgtk-3-0 gsettings-desktop-schemas-dev libgee-dev libsqlite3-dev inttool
    apt-get install libgirepository1.0-dev gnome-common

