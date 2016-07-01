

# Compilation auf der xMZ-Mod-Touch Hardware
## Vorbereitungen
### Installation Rust

Leider geht das Standard Komando `curl https://sh.rustup.rs -sSf | sh` nicht
mit den nötigen Parametern `--default-toolchain nightly` und `-y`
Die Installation ist aber mit einer lokalen Kopie von rustup.sh möglich.
```
curl https://sh.rustup.rs -sSf > rustup.sh
chmod +x rustup.sh
./rustup.sh --default-toolchain nightly -y
rm rustup.sh
```

Zum Aktivieren einfach das `env` File aus dem `.cargo` Verzeichnis sourcen.
```
source ~/.cargo/env
```

### Gtk Bibliotheken

```
apt-get install libgtk-3-dev
```

# Auschecken des Quellcodes und Compilation

```
git clone https://github.com/zzeroo/xMZ-Mod-Touch-GUI.git
cd xMZ-Mod-Touch-GUI
cargo build --release
```

# Installation
```
cp target/release/xmz_mod_touch_gui /usr/bin/xmz_mod_touch_gui
```

## Systemd
### GUI Systemd Unit file

```
cat <<EOF >/etc/systemd/system/xmz-mod-touch-GUI.service
#
# xMZ-Mod-Touch-GUI systemd service unit file
#

[Unit]
Description=xMZ-Mod-Touch Graphical User Interface (GUI)
# Wants=syslog.target dbus.service
After=weston.service

[Service]
Environment="XDG_RUNTIME_DIR=/run/shm/wayland"
Environment="GDK_BACKEND=wayland"
Environment="XMZ_HARDWARE=0.1.0"
Environment="LANG=de_DE.UTF-8"
ExecStart=/usr/bin/xmz_mod_touch_gui
Restart=always
RestartSec=10

[Install]
Alias=xmz.service
WantedBy=graphical.target
EOF
```

```
systemctl daemon-reload
```

```
systemctl enable xmz-mod-touch-GUI.service
```

```
systemctl start xmz-mod-touch-GUI.service
```


# Update Code auf der xMZ-Mod-Touch Hardware

```
cd xMZ-Mod-Touch-GUI
git pull
cargo build --release
systemctl stop xmz-mod-touch-GUI.service
cp target/release/xmz_mod_touch_gui /usr/bin/xmz_mod_touch_gui
systemctl start xmz-mod-touch-GUI.service
```



# Links
## Rust Dokumentation lokal
* file:///mnt/src/xMZ-Mod-Touch-Software/xMZ-Mod-Touch-GUI/target/doc/gtk/trait.WindowExt.html#tymethod.fullscreen
* file:///home/smueller/src/Rust/gtk/target/doc/gtk/index.html
