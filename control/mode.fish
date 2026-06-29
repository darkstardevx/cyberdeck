function cyber_mode -a mode

    set file /tmp/cyberdeck_state.json

    switch $mode

        case turbo
            echo '{"mode":"turbo"}' > $file
            echo "🔥 TURBO MODE ENABLED"
            sudo cpupower frequency-set -g performance

        case balanced
            echo '{"mode":"balanced"}' > $file
            echo "⚖ BALANCED MODE"
            sudo cpupower frequency-set -g schedutil

        case silent
            echo '{"mode":"silent"}' > $file
            echo "🌙 SILENT MODE"
            sudo cpupower frequency-set -g powersave
    end
end
