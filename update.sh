#!/bin/bash
# Dieses Script beinhaltet die einzelnen Schritte die in der Regel nötig sind
# um die Software auf den aktuellsten Stand zu bringen.
# Es ist ausdrücklich nicht Aufgabe dieses Scriptes die Git Versionskontrolle
# zu steuern! Dies ist Aufgabe des Entwicklers. Da heißt vor Aufruf des Scriptes
# muss der Softwarezweig mit z.B. `git checkout` und `git pull` auf den
# gewünschten Stand gebracht werden.

# Exit on error or variable unset
set -o errexit -o nounset

# Stop laufende Instanz
systemctl stop xmz-mod-touch-gui.service
# Bilde neues Release
cargo build --release
# Kopiere neu erstellte Binaries und Assets in das Dateisystem
cp -v ./target/release/xmz_mod_touch_gui /usr/bin/xmz-mod-touch-gui
## Erstelle Assets Verzeichnis
[ -d "/usr/share/xmz-mod-touch-gui/" ] || mkdir "/usr/share/xmz-mod-touch-gui/"
cp -v ./src/gui/gtk3/interface.glade /usr/share/xmz-mod-touch-gui/
# Starte Instanz wieder
systemctl start xmz-mod-touch-gui.service
