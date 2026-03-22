#!/usr/bin/env bash
# Run wallsav after 30s of inactivity, stops with a single keypress

IDLE_FILE="/tmp/.last_command_time.$$"
date +%s >"$IDLE_FILE"

# Update timer on command
PROMPT_COMMAND='date +%s >"$IDLE_FILE"'

while true; do
  sleep 1
  last=$(cat "$IDLE_FILE")
  now=$(date +%s)
  idle=$((now - last))
  if ((idle >= 45)); then
    # Launch wallsav in foreground, let it handle keypresses
    ~/.local/bin/sav
    # Reset idle timer after it finishes
    date +%s >"$IDLE_FILE"
  fi
done
