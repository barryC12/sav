#!/usr/bin/env fish

set IDLE_FILE /tmp/.last_command_time.$fish_pid
date +%s >$IDLE_FILE

function update_idle_file
    date +%s >$IDLE_FILE
end

function idle_watcher
    while true
        sleep 1
        set last (cat $IDLE_FILE)
        set now (date +%s)
        set idle (math $now - $last)
        if test $idle -ge 45
            ~/.local/bin/sav
            update_idle_file
        end
    end
end

idle_watcher &

function track_keys --on-event fish_key
    update_idle_file
end
