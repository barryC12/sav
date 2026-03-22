#!/usr/bin/env bash
IDLE_FILE="/tmp/.last_command_time.$$"
date +%s >"$IDLE_FILE"

update_idle_file() {
    date +%s >"$IDLE_FILE"
}

idle_watcher() {
    while true; do
        sleep 1
        last=$(cat "$IDLE_FILE")
        now=$(date +%s)
        idle=$((now - last))
        if [ "$idle" -ge 45 ]; then
            ~/.local/bin/sav
            update_idle_file
        fi
    done
}

idle_watcher &

trap update_idle_file DEBUG
