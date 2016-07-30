Grafische Oberfläche der 'xMZ-Mod-Touch'-Platform

Diese GUI fragt über Nanomsg Sockets den [Server][server] der 'xMZ-Mod-Touch' ab.
Außerdem ist es möglich diverse Parameter von der GUI an den Server zu senden,
zum Beispiel die Modbus Konfiguration ändern, oder neu Module anlegen.

# Anhängigkeiten installieren
## Gtk Bibliotheken

```
apt-get install libgtk-3-dev
```

# Build, Compilation auf der 'xMZ-Mod-Touch'-Hardware
Die folgenden Befehle gehen davon aus das das Meta Git Repository
['xMZ-Mod-Touch-Software'][1] im HOME Verzeichnis ausgecheckt wurde.

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-GUI
cargo build --release
```

# Installation
## Programmdateien installieren

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-GUI
cp target/release/xmz_mod_touch_gui /usr/bin/xmz-mod-touch-gui
```

## Systemd Unit File anlegen
Dieser Schritt muss nur ein mal ausgeführt werden. Im Zweifel kann der Befehl aber
immer wieder aufgerufen werden (zum Beispiel im Update Fall).

```bash
cat <<EOF >/etc/systemd/system/xmz-mod-touch-gui.service
#
# xMZ-Mod-Touch-GUI systemd unit file
#
[Unit]
Description="xMZ-Mod-Touch-GUI (Graphical User Interface) der 'xMZ-Mod-Touch'-Platform"
# Wants=syslog.target dbus.service
After=weston.service

[Service]
Environment="XDG_RUNTIME_DIR=/run/shm/wayland"
Environment="GDK_BACKEND=wayland"
Environment="XMZ_HARDWARE=0.1.0"
Environment="LANG=de_DE.UTF-8"
ExecStart=/usr/bin/xmz-mod-touch-gui
Restart=always
RestartSec=10

[Install]
Alias=xmz-gui.service
WantedBy=graphical.target
EOF
```

Danach muss der service noch aktiviert ...

```bash
systemctl enable xmz-mod-touch-gui.service
# systemctl daemon-reload # Dieser Befehl ist nur bei nachträglichen Änderungen am Unit File nötig!
```

... und gestartet werden.

```bash
systemctl restart xmz-mod-touch-gui.service
```

# Update des Entwicklungssystems
Für ein Update muss zunächst die laufende Instanz beendet werden `systemctl stop`,
danach wird in das Verzeichnis mit dem Quellcode gewechselt, der aktuelle
Softwarestand mit `git pull` herunter geladen und anschließend die Software
gebaut.
Angeschlossen wird das Ganze indem die neu erstellen Binaries nach `/usr/bin`
kopiert werden und die Software neu gestartet wird `systemctl start`.

```bash
# systemctl daemon-reload # Evtl. wenn das systemd Unit File geändert wurde
systemctl stop xmz-mod-touch-gui.service
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-GUI
git pull
cargo build --release
cp target/release/xmz_mod_touch_gui /usr/bin/xmz-mod-touch-gui

systemctl start xmz-mod-touch-gui.service
```


# Tests

Optional können auch die Tests aufgerufen werden.

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-GUI
cargo test
```


# Links

* https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software
* https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server


[1]: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software
[server]: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server
