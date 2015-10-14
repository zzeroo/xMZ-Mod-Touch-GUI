
##Installation

Dieses Programm kann mit dem bekannten Dreisatz erstellt und installiert werden.

    ./autogen.sh [--prefix=/home/your_username/.local]
    make
    make install

Diese drei Befehle erstellen die folgenden Dateien.

aclocal.m4
autom4te.cache
config.log
config.status
configure
depcomp
xmz-mod-touch-gui
xmz-mod-touch-gui.c
xmz-mod-touch-gui.desktop
xmz_mod_touch_gui-xmz-mod-touch-gui.o
xmz_mod_touch_gui_vala.stamp
install-sh
missing
Makefile.in
Makefile

Der Befehl `make` linkt alle Bibliothken.

`make install`, installiert die Anwendung in /home/your_username/.local/bin und
installiert die xmz_mod_touch_gui_vala.desktop Datei nach `/home/your_username/.local/share/applications`

##Deinstallation

Für die Deinstallation bitte diesen Befehl verwenden:

    make uninstall

##Packing

Es kann auch ein tarball erstellt weden, verwende dazu diesen Befehl:

    make distcheck

# Database

    sqlite3 src/testdb << EOF
    CREATE TABLE sensors (id INT, name TEXT, value DOUBLE);
    INSERT INTO tbl VALUES (1, "Sensor 1", 0);
    INSERT INTO tbl VALUES (2, "Sensor 2", 0);
    EOF



## Development

Das Quellcode Repository ist mit Autotools konfiguriert und wird mit Capistrano verwaltet.

###Autotools

    ./autogen.sh [evtl. Parameter für configure, z.B. --prefix=/usr]

### Capistrano

    cap staging deploy

oder

    cap production deploy
