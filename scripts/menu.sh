#!/bin/bash
# DarkBox WM - System Menu

cpu=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1)
[ -z "$cpu" ] && cpu=0

ram=$(free -h | awk '/^Mem:/ {print $3"/"$2}')
ram_percent=$(free | awk '/^Mem:/ {printf "%.1f", $3/$2 * 100}')

disk=$(df -h / | awk 'NR==2 {print $3"/"$2" ("$5")"}')
uptime=$(uptime -p | sed 's/up //')
load=$(uptime | awk -F'load average:' '{print $2}')
kernel=$(uname -r)

menu_text="╔════════════════════════════╗
║      DARKBOX SYSTEM INFO      ║
╠════════════════════════════════╣
║ CPU:   ${cpu}%                 ║
║ RAM:   ${ram} (${ram_percent}%)       ║
║ DISK:  ${disk}              ║
║ UPTIME: ${uptime}              ║
║ LOAD:${load}                   ║
║ KERNEL: ${kernel:0:20}        ║
╚════════════════════════════════╝

[ ESC to close ]"

echo "$menu_text" | fuzzel --dmenu --lines 13 --width 46