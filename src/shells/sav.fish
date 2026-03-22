#!/usr/bin/env fish
# Run wallsav after 90s of inactivity, stops with a single keypress

set IDLE_FILE /tmp/.last_command_time.$fish_pid
date +%s >$IDLE_FILE

# Update timer on every command
function update_idle_file --on-event fish_prompt
    date +%s >$IDLE_FILE
end

while true
    sleep 1
    set last (cat $IDLE_FILE)
    set now (date +%s)
    set idle (math $now - $last)
    if test $idle -ge 45
        # Launch wallsav in foreground, let it handle keypresses
        ~/.local/bin/sav
        # Reset idle timer after it finishes
        date +%s >$IDLE_FILE
    end
end
