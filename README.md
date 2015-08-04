# xMZ-Mod-Touch-GUI
xMZ-Mod-Touch Grapical User Interface


## Programm aus den Quellen übersetzen

### Abhänigkeiten

Die Software setzt auf Gtk+-3.0, libmodbus und wird mit den Autotools
und dem GNU C compiler übersetzt. Diese Programme und Bibliotheken müssen
installiert sein.

Unter Debian bzw Ubuntu können die Abhängigkeiten mit folgendem Befehl 
installiert werden.

        sudo apt-get install build-essential libgtk-3-0 libmodbus

### Übersezung

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
