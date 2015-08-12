# xMZ-Mod-Touch-GUI
xMZ-Mod-Touch Grapical User Interface


## Programm aus den Quellen übersetzen

### Abhänigkeiten

Die Software setzt auf Vala, Gtk+-3.0, und libmodbus.

Unter Debian bzw Ubuntu können die Abhängigkeiten mit folgendem Befehl
installiert werden.

        apt-get install valac libgtk-3-0

### Übersezung
#### manuelle Kompilation

      valac --pkg gtk+-3.0 src/gtk-sample.vala

#### Kompilation mit Autotools
Zunächst muss der Quellcode auf den Computer kopiert weden.

        git clone https://github.com/zzeroo/xMZ-Mod-Touch-GUI.git

Danach wird in das Verzeichnis mit dem Quellcode gewechselt und
die Software mit den Autotools Werkzeugen übersetzt.

        cd MZ-Mod-Touch-GUI
        ./autogen.sh
        make
        sudo make install

Dem Script autogen.sh können die üblichen Parameter wie `--prefix=/usr`
übergeben weden.
