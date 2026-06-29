function ethernet -a dir

    set name (status function)

    set base_f "$dir/$name.md"

    set if_dir "$dir/interfaces"
    set drv_dir "$dir/driver"
    set net_dir "$dir/network"
    set stats_dir "$dir/stats"

    mkdir -p $if_dir $drv_dir $net_dir $stats_dir

    # =========================
    # SAFE CACHE (FIXED)
    # =========================
    cyberdeck_env
    set cache $CYBERDECK_CACHE_DIR
    mkdir -p $cache

    # =========================
    # ETHERNET
    # =========================
    set name ethernet
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
    "<div style='background:#6a0dad;color:white;padding:6px;'>🌐 ETHERNET INTELLIGENCE (RTL8125 / r8125)</div>" \
    > $base_f

    echo "" >> $base_f
    echo "Timestamp: "(date) >> $base_f

    echo "" >> $base_f
    echo "## 🧠 SYSTEM OVERVIEW" >> $base_f
    hostname >> $base_f 2>/dev/null

    # =========================
    # NETWORK STACK
    # =========================
    echo "" >> $base_f
    echo "## 🌐 NETWORK STACK" >> $base_f

    ip a >> $base_f 2>/dev/null
    ip link >> $base_f 2>/dev/null
    ip route >> $base_f 2>/dev/null

    # ==========================================================
    # SAFE INTERFACE ENUMERATION (SYSFS HARDENED)
    # ==========================================================
    echo "" >> $base_f
    echo "## 🔌 INTERFACES" >> $base_f

    for p in /sys/class/net/*
        set i (basename $p)

        # 🧠 HARDEN: ignore junk entries if any appear
        if test -z "$i"
            continue
        end

        set safe_i (string replace -a "/" "_" $i)
        set f "$if_dir/$safe_i.md"

        echo "# 🔌 INTERFACE: $i" > $f

        echo "" >> $f
        echo "## 📊 IP CONFIG" >> $f
        ip addr show $i >> $f 2>/dev/null

        echo "" >> $f
        echo "## ⚙️ LINK STATUS" >> $f
        cat /sys/class/net/$i/operstate >> $f 2>/dev/null

        # =========================
        # SPEED
        # =========================
        if test -f /sys/class/net/$i/speed
            echo "" >> $f
            echo "## 🚀 LINK SPEED (Mbps)" >> $f
            cat /sys/class/net/$i/speed >> $f 2>/dev/null
        end

        # =========================
        # ETHTOOL
        # =========================
        if command -q ethtool
            echo "" >> $f
            echo "## 🔗 ETH TOOL STATUS" >> $f
            timeout 2 ethtool $i >> $f 2>/dev/null
        end
    end

    # =========================
    # DRIVER LAYER
    # =========================
    echo "" >> $base_f
    echo "## 🧩 DRIVER LAYER (r8125)" >> $base_f

    for p in /sys/class/net/*
        set i (basename $p)

        if test -L /sys/class/net/$i/device/driver
            echo "### $i driver:" >> $base_f
            readlink /sys/class/net/$i/device/driver >> $base_f 2>/dev/null
        end
    end

    # =========================
    # KERNEL MODULES
    # =========================
    echo "" >> $base_f
    echo "## 🧠 LOADED KERNEL MODULES (NIC RELATED)" >> $base_f

    lsmod | grep -E "r8125|r8169|realtek|e1000|igc" >> $base_f 2>/dev/null

    # =========================
    # CONFLICT CHECK
    # =========================
    echo "" >> $base_f
    echo "## ⚠️ DRIVER CONFLICT CHECK" >> $base_f

    if lsmod | grep -q r8169
        echo "⚠️ r8169 detected (may conflict with r8125)" >> $base_f
    else
        echo "✔ no r8169 conflict detected" >> $base_f
    end

    # =========================
    # RAW DATA
    # =========================
    echo "" >> $base_f
    echo "## 📦 RAW NETWORK DATA" >> $base_f

    lspci >> $base_f 2>/dev/null
    lsusb >> $base_f 2>/dev/null

    # =========================
    # INTERFACE STATS
    # =========================
    echo "" >> $base_f
    echo "## 📊 INTERFACE STATS" >> $base_f

    for p in /sys/class/net/*
        set i (basename $p)

        if test -z "$i"
            continue
        end

        set safe_i (string replace -a "/" "_" $i)
        set f "$stats_dir/$safe_i-stats.md"

        echo "# 📊 STATS: $i" > $f

        if test -d /sys/class/net/$i/statistics

            echo "## RX/TX PACKETS" >> $f
            cat /sys/class/net/$i/statistics/rx_packets >> $f 2>/dev/null
            cat /sys/class/net/$i/statistics/tx_packets >> $f 2>/dev/null

            echo "" >> $f
            echo "## RX/TX BYTES" >> $f
            cat /sys/class/net/$i/statistics/rx_bytes >> $f 2>/dev/null
            cat /sys/class/net/$i/statistics/tx_bytes >> $f 2>/dev/null
        end
    end

end
