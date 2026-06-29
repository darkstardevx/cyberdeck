function audio -a dir

    set name (status function)
    set f "$dir/$name.md"

    set raw_dir "$dir/raw"
    set parsed_dir "$dir/parsed"
    set graph_dir "$dir/graph"

    mkdir -p $raw_dir $parsed_dir $graph_dir

    set raw_f "$raw_dir/audio_raw.md"
    set graph_f "$graph_dir/pipewire_graph.md"
    set parsed_f "$parsed_dir/audio_summary.md"

    cyberdeck_env
    set cache $CYBERDECK_CACHE_DIR
    mkdir -p $cache

    # =========================
    # AUDIO
    # =========================
    set name audio
    set base_f "$dir/$name.md"

    # hard safety check (IMPORTANT)
    if test -z "$base_f"
        echo "❌ base_f is empty - aborting module"
        return 1
    end

    # =========================
    # AUDIO
    # =========================
    cyber_write_header $base_f "PIPEWIRE AUDIO INTELLIGENCE" "🎧"

    echo "" >> $base_f
    echo "Timestamp: "(date) >> $base_f

    echo "" >> $f
    echo "## 📊 SYSTEM OVERVIEW" >> $f
    echo "- Timestamp: "(date) >> $f
    echo "- Stack: PipeWire + WirePlumber + Pulse compatibility" >> $f

    # =========================
    # PIPEWIRE STATUS
    # =========================
    echo "" >> $f
    echo "## ⚙️ PIPEWIRE SERVICES" >> $f

    systemctl --user status pipewire >> $f 2>/dev/null
    systemctl --user status wireplumber >> $f 2>/dev/null
    systemctl --user status pipewire-pulse >> $f 2>/dev/null

    # =========================
    # PULSEAUDIO COMPAT LAYER
    # =========================
    echo "" >> $f
    echo "## 🔊 PULSE COMPAT LAYER" >> $f

    if command -q pactl
        pactl info >> $f 2>/dev/null
        pactl list short sinks >> $f 2>/dev/null
        pactl list short sources >> $f 2>/dev/null
    else
        echo "pactl not available" >> $f
    end

    # =========================
    # PIPEWIRE GRAPH (CORE INTELLIGENCE)
    # =========================
    echo "" >> $f
    echo "## 🔗 PIPEWIRE NODE GRAPH" >> $f

    if command -q pw-cli
        pw-cli ls Node >> $graph_f 2>/dev/null
        pw-cli ls Link >> $graph_f 2>/dev/null

        echo "Graph exported to: $graph_f" >> $f
    else
        echo "pw-cli not available" >> $f
    end

    # =========================
    # AUDIO HARDWARE
    # =========================
    echo "" >> $f
    echo "## 🎛️ AUDIO HARDWARE" >> $f

    lspci | grep -i audio >> $f
    lsusb | grep -i audio >> $f

    # =========================
    # HYPRLAND AUDIO CENTER CHECK
    # =========================
    echo "" >> $f
    echo "## 🧿 HYPR AUDIO INTEGRATION" >> $f

    if command -q hyprpwcenter
        hyprpwcenter --version >> $f 2>/dev/null
        echo "✔ hyprpwcenter detected" >> $f
    else
        echo "hyprpwcenter not installed" >> $f
    end

    # =========================
    # RAW DUMP
    # =========================
    echo "" >> $f
    echo "## 📦 RAW DEBUG DUMP" >> $f

    lscpu >> $raw_f
    lspci >> $raw_f
    lsusb >> $raw_f

    pactl info >> $raw_f 2>/dev/null
    pw-cli ls Node >> $raw_f 2>/dev/null

    # =========================
    # PARSED SUMMARY
    # =========================
    echo "# 🎧 AUDIO SUMMARY" > $parsed_f

    if command -q pactl
        echo "## 🔊 Active Sinks" >> $parsed_f
        pactl list short sinks >> $parsed_f

        echo "" >> $parsed_f
        echo "## 🎤 Active Sources" >> $parsed_f
        pactl list short sources >> $parsed_f
    end

    # =========================
    # HEALTH CHECK
    # =========================
    echo "" >> $f
    echo "## 🧠 AUDIO HEALTH" >> $f

    if systemctl --user is-active pipewire >/dev/null
        echo "✔ PipeWire: running" >> $f
    else
        echo "❌ PipeWire: not running" >> $f
    end

    if systemctl --user is-active wireplumber >/dev/null
        echo "✔ WirePlumber: running" >> $f
    else
        echo "❌ WirePlumber: not running" >> $f
    end

end
