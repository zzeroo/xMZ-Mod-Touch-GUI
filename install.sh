#!/bin/bash
# Dieses Script sollte nur ein einziges Mal aufgerufen werden! Alle weiteren
# Aufrufe bitte mit dem Script ./update.sh durchführen.

# Exit on error or variable unset
set -o errexit -o nounset

# Programmdateien installieren
cp -v ./target/release/xmz_mod_touch_gui /usr/bin/xmz-mod-touch-gui

# Systemd Unit File anlegen

cat <<EOF | tee /etc/systemd/system/xmz-mod-touch-gui.service
#
# xMZ-Mod-Touch-GUI systemd unit file
#
[Unit]
Description="xMZ-Mod-Touch-GUI (Graphical User Interface) der 'xMZ-Mod-Touch'-Platform"
# Wants=syslog.target dbus.service
After=weston.service

[Service]
Environment=XDG_RUNTIME_DIR=/run/shm/wayland
Environment=LD_LIBRARY_PATH=/usr/lib
Environment=GDK_BACKEND=wayland
Environment=XMZ_HARDWARE=0.1.0
Environment=LANG=de_DE.UTF-8
ExecStart=/usr/bin/xmz-mod-touch-gui
Restart=always
RestartSec=10

[Install]
Alias=xmz-gui.service
WantedBy=graphical.target
EOF

# Unit aktivieren ...
systemctl enable xmz-mod-touch-gui.service

# Unit starten
systemctl restart xmz-mod-touch-gui.service
