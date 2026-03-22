#!/usr/bin/env zsh
# Run wallsav after 90s of inactivity, stops with a single keypress

IDLE_FILE="/tmp/.last_command_time.$$"
date +%s > "$IDLE_FILE"

# Update timer on every command
function zsh_precmd() {
    date +%s > "$IDLE_FILE"
}
autoload -Uz add-zsh-hook
add-zsh-hook precmd zsh_precmd

while true; do
    sleep 1
    last=$(cat "$IDLE_FILE")
    now=$(date +%s)
    idle=$((now - last))
    if (( idle >= 90 )); then
        # Launch wallsav in foreground, let it handle keypresses
        ~/.local/bin/sav
        # Reset idle timer after it finishes
        date +%s > "$IDLE_FILE"
    fi
done
