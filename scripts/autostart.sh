#!/bin/bash

sleep 1

# Запускаем панель
waybar &

# Запускаем терминал
foot &

echo "DarkBox WM started on $(date)" >> /tmp/darkbox.log