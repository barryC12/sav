# Screensaver on inactivity — runs ~/.local/bin/sav after every 45s idle window.
# Install: add `source /path/to/idle_sav.fish` to ~/.config/fish/config.fish

set -g _idle_threshold 45
set -U idle_last_activity (date +%s)

function _idle_reset --on-event fish_postexec
    set -U idle_last_activity (date +%s)
end

function _idle_start_watcher
    if test -n "$_idle_watcher_pid"
        kill $_idle_watcher_pid 2>/dev/null
    end

    fish -c '
        set threshold '"$_idle_threshold"'
        set last_fired 0

        while true
            sleep 1
            set now (date +%s)
            set last $idle_last_activity
            if test -z "$last"; continue; end

            set elapsed (math "$now - $last")

            # Fire once per threshold-sized idle window.
            # last_fired is local to this process so a new command
            # (which updates idle_last_activity) resets the cooldown naturally.
            if test "$elapsed" -ge "$threshold"
                set since_fired (math "$now - $last_fired")
                if test "$since_fired" -ge "$threshold"
                    set last_fired $now
                    ~/.local/bin/sav
                end
            end
        end
    ' &

    set -g _idle_watcher_pid $last_pid
end

_idle_start_watcher
