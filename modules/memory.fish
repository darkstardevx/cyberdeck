function memory -a dir

    set name (status function)

    set base_f "$dir/$name.md"

    set dimm_dir "$dir/dimm"
    set usage_dir "$dir/usage"
    set topo_dir "$dir/topology"
    set raw_dir "$dir/raw"

    mkdir -p $dimm_dir $usage_dir $topo_dir $raw_dir

    # =========================
    # SAFE CACHE (FIXED)
    # =========================
    cyberdeck_env
    set cache $CYBERDECK_CACHE_DIR
    mkdir -p $cache

    # =========================
    # MEMORY
    # =========================
    set name memory
    set base_f "$dir/$name.md"

    # hard safety check (IMPORTANT)
    if test -z "$base_f"
        echo "❌ base_f is empty - aborting module"
        return 1
    end

    # =========================
    # HEADER
    # =========================
    printf "%s\n" \
    "<div style='background:#6a0dad;color:white;padding:6px;'>🧠 MEMORY INTELLIGENCE SYSTEM (RAM / DIMM / SWAP)</div>" \
    > $base_f

    echo "" >> $base_f
    echo "Timestamp: "(date) >> $base_f

    # =========================
    # USAGE STATS
    # =========================
    echo "" >> $base_f
    echo "## 📊 MEMORY USAGE" >> $base_f

    free -h >> $base_f
    vmstat 1 1 >> $base_f 2>/dev/null

    echo "# 📊 MEMORY USAGE" > "$usage_dir/usage.md"
    free -h >> "$usage_dir/usage.md"
    vmstat 1 1 >> "$usage_dir/usage.md" 2>/dev/null

    # =========================
    # DIMM / PHYSICAL RAM
    # =========================
    echo "" >> $base_f
    echo "## 🧩 DIMM / PHYSICAL MEMORY" >> $base_f

    if command -q dmidecode
        sudo dmidecode -t memory >> $base_f 2>/dev/null
        sudo dmidecode -t memory > "$dimm_dir/dimm_raw.md" 2>/dev/null
    else
        echo "dmidecode not available" >> $base_f
    end

    # try structured slot view
    echo "" >> $base_f
    echo "## 🧬 MEMORY SLOT STRUCTURE" >> $base_f

    for i in (seq 0 31)
        if test -f /sys/devices/system/node/node0/meminfo
            break
        end
    end

    for slot in /sys/devices/system/node/node*/meminfo
        if test -r $slot
            echo "### NUMA NODE: (basename (dirname $slot))" >> $base_f
            cat $slot >> $base_f
        end
    end

    # =========================
    # TOPOLOGY
    # =========================
    echo "" >> $base_f
    echo "## 🧠 TOPOLOGY" >> $base_f

    lscpu >> $base_f

    echo "# 🧠 MEMORY TOPOLOGY" > "$topo_dir/topology.md"
    lscpu >> "$topo_dir/topology.md"

    # =========================
    # SWAP / PRESSURE
    # =========================
    echo "" >> $base_f
    echo "## 🔄 SWAP & PRESSURE" >> $base_f

    swapon --show >> $base_f 2>/dev/null
    cat /proc/meminfo >> $base_f

    echo "# 🔄 SWAP INFO" > "$usage_dir/swap.md"
    swapon --show >> "$usage_dir/swap.md" 2>/dev/null
    cat /proc/meminfo >> "$usage_dir/swap.md"

    # =========================
    # RAW DATA
    # =========================
    echo "" >> $base_f
    echo "## 📦 RAW MEMORY DATA" >> $base_f

    lspci >> $base_f
    lsusb >> $base_f

    echo "# 📦 RAW MEMORY DUMP" > "$raw_dir/raw.md"
    cat /proc/meminfo >> "$raw_dir/raw.md"
    free -h >> "$raw_dir/raw.md"

    # =========================
    # SUMMARY
    # =========================
    echo "" >> $base_f
    echo "## 🧠 MEMORY SUMMARY" >> $base_f

    if command -q free
        set total (free -h | awk '/Mem:/ {print $2}')
        set used (free -h | awk '/Mem:/ {print $3}')
        echo "- Total RAM: $total" >> $base_f
        echo "- Used RAM: $used" >> $base_f
    end

end
