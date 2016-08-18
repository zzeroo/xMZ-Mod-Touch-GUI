#!/bin/bash

# Exit on error or variable unset
set -o errexit -o nounset

# Stop laufende Instanz
systemctl stop xmz-gui
# Bilde neues Release
cargo build --release
# Kopiere neu erstellte Binaries und Assets in das Dateisystem
cp target/release/xmz_mod_touch_gui /usr/bin/xmz-mod-touch-gui
cp ./src/gui/gtk3/interface.glade /usr/share/xmz-mod-touch-gui/
# Starte Instanz wieder
systemctl start xmz-mod-touch-gui.service
