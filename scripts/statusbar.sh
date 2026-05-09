#!/bin/bash

while true; do
    cpu=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1)
    [ -z "$cpu" ] && cpu=0
    
    ram=$(free -h | awk '/^Mem:/ {print $3"/"$2}')
    
    if [ -f /sys/class/thermal/thermal_zone0/temp ]; then
        temp=$(cat /sys/class/thermal/thermal_zone0/temp)
        temp=$((temp / 1000))
        temp_out=" | ${temp}C"
    else
        temp_out=""
    fi
    
    time=$(date "+%H:%M")
    
    echo "{\"text\":\"CPU ${cpu}% | RAM ${ram}${temp_out} | ${time}\"}"
    
    sleep 2
done