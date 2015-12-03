#!/bin/bash

gource --output-custom-log master.txt --git-branch master 
gource --output-custom-log development.txt --git-branch development

cat master.txt development.txt | sort -n > combined.txt  


gource combined.txt --user-image-dir .git/avatar/ --auto-skip-seconds 1 -e 0.5 --title "xMZ-Mod-Touch - xMesszentrale mit Modbus Interface und Touchscreen" -f --loop
