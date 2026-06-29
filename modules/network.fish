function network -a dir

    set name (status function)

    set base_f "$dir/$name.md"

    set iface_dir "$dir/interfaces"
    set route_dir "$dir/routes"
    set dns_dir "$dir/dns"
    set socket_dir "$dir/sockets"
    set raw_dir "$dir/raw"

    mkdir -p $iface_dir $route_dir $dns_dir $socket_dir $raw_dir

    # =========================
    # SAFE CACHE (FIXED)
    # =========================
    cyberdeck_env
    set cache $CYBERDECK_CACHE_DIR
    mkdir -p $cache

    # =========================
    # NETWORK
    # =========================
    set name network
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
    "<div style='background:#6a0dad;color:white;padding:6px;'>🌐 NETWORK INTELLIGENCE SYSTEM (NIC / ROUTES / SOCKETS)</div>" \
    > $base_f

    echo "" >> $base_f
    echo "Timestamp: "(date) >> $base_f

    # =========================
    # NETWORK STACK SNAPSHOT
    # =========================
    echo "" >> $base_f
    echo "## 📦 NETWORK STACK SNAPSHOT" >> $base_f

    ip a >> $base_f 2>/dev/null
    ip link >> $base_f 2>/dev/null
    ip route >> $base_f 2>/dev/null
    ss -tulpn >> $base_f 2>/dev/null

    # =========================
    # INTERFACES (FIXED SYSFS ITERATION)
    # =========================
    echo "" >> $base_f
    echo "## 🔌 INTERFACES (DETAILED)" >> $base_f

    for p in /sys/class/net/*
        set i (basename $p)

        # safety guard (prevents weird sysfs edge cases)
        if test -z "$i"
            continue
        end

        set f "$iface_dir/$i.md"

        echo "# 🔌 INTERFACE: $i" > $f

        echo "" >> $f
        echo "## 📡 IP CONFIG" >> $f
        ip addr show $i >> $f 2>/dev/null

        echo "" >> $f
        echo "## ⚙️ STATE" >> $f
        cat /sys/class/net/$i/operstate >> $f 2>/dev/null

        if test -f /sys/class/net/$i/speed
            echo "" >> $f
            echo "## 🚀 LINK SPEED (Mbps)" >> $f
            cat /sys/class/net/$i/speed >> $f 2>/dev/null
        end

        if test -f /sys/class/net/$i/duplex
            echo "" >> $f
            echo "## 🔗 DUPLEX" >> $f
            cat /sys/class/net/$i/duplex >> $f 2>/dev/null
        end

        if command -q ethtool
            echo "" >> $f
            echo "## 🧠 ETH TOOL INFO" >> $f
            timeout 2 ethtool $i >> $f 2>/dev/null
        end

    end

    # =========================
    # ROUTING LAYER
    # =========================
    echo "" >> $base_f
    echo "## 🧭 ROUTING TABLE" >> $base_f

    ip route >> $base_f 2>/dev/null

    echo "# 🧭 ROUTES" > "$route_dir/routes.md"
    ip route >> "$route_dir/routes.md" 2>/dev/null

    # =========================
    # DNS / RESOLVER
    # =========================
    echo "" >> $base_f
    echo "## 🌍 DNS / RESOLVER" >> $base_f

    if test -f /etc/resolv.conf
        cat /etc/resolv.conf >> $base_f
        cat /etc/resolv.conf > "$dns_dir/resolv.conf"
    else
        echo "no resolv.conf found" >> $base_f
    end

    if command -q resolvectl
        echo "" >> $base_f
        echo "### SYSTEMD RESOLVED STATUS" >> $base_f
        resolvectl status >> $base_f 2>/dev/null
    end

    # =========================
    # SOCKETS
    # =========================
    echo "" >> $base_f
    echo "## 🔥 ACTIVE SOCKETS" >> $base_f

    ss -tulpn >> $base_f 2>/dev/null

    echo "# 🔥 SOCKETS" > "$socket_dir/sockets.md"
    ss -tulpn >> "$socket_dir/sockets.md" 2>/dev/null

    # =========================
    # HARDWARE LAYER
    # =========================
    echo "" >> $base_f
    echo "## 🧩 NETWORK HARDWARE" >> $base_f

    lspci >> $base_f 2>/dev/null
    lsusb >> $base_f 2>/dev/null

    # =========================
    # DRIVER STATUS
    # =========================
    echo "" >> $base_f
    echo "## 🧠 NIC DRIVER STATUS" >> $base_f

    if lsmod | grep -q r8125
        echo "✔ r8125 driver active (Realtek 2.5GbE)" >> $base_f
    end

    if lsmod | grep -q r8169
        echo "⚠ r8169 also loaded (possible conflict)" >> $base_f
    end

    # =========================
    # RAW DUMP
    # =========================
    echo "" >> $base_f
    echo "## 📦 RAW NETWORK DATA" >> $base_f

    ip a >> $base_f 2>/dev/null
    ip link >> $base_f 2>/dev/null
    ip route >> $base_f 2>/dev/null
    ss -tulpn >> $base_f 2>/dev/null

    echo "# 📦 RAW NETWORK" > "$raw_dir/raw.md"
    ip a >> "$raw_dir/raw.md" 2>/dev/null
    ip route >> "$raw_dir/raw.md" 2>/dev/null

end
