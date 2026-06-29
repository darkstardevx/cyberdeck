function hardware -a dir

    set f "$dir/hardware.md"

    cyberdeck_env
    set cache $CYBERDECK_CACHE_DIR
    mkdir -p $cache

    # =========================
    # HARDWARE
    # =========================
    set name hardware
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
    "<div style='background:#6a0dad;color:white;padding:6px;'>🖥️ HARDWARE CORE REPORT</div>" \
    > $base_f

    echo "" >> $base_f
    echo "Timestamp: "(date) >> $base_f

    echo "" >> $f
    echo "## 🧠 CPU" >> $f
    lscpu >> $f

    echo "" >> $f
    echo "## 🧩 PCI DEVICES" >> $f
    lspci >> $f

    echo "" >> $f
    echo "## 🔌 USB DEVICES" >> $f
    lsusb >> $f

    echo "" >> $f
    echo "## ⚙️ DETAILED HARDWARE TREE" >> $f
    sudo lshw -short >> $f 2>/dev/null

end
