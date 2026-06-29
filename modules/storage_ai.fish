function storage_ai -a dir

    set name (status function)

    set base_f "$dir/$name.md"

    set nvme_dir "$dir/nvme"
    set ssd_dir "$dir/ssd"
    set hdd_dir "$dir/hdd"
    set usb_dir "$dir/usb"
    set raw_dir "$dir/raw"
    set fs_dir "$dir/filesystems"
    set smart_dir "$dir/smart"

    mkdir -p $nvme_dir $ssd_dir $hdd_dir $usb_dir $raw_dir $fs_dir $smart_dir

    # =========================
    # SAFE CACHE (FIXED)
    # =========================
    cyberdeck_env
    set cache $CYBERDECK_CACHE_DIR
    mkdir -p $cache

    # =====================================
    # STORAGE INTELLIGENCE SYSTEM (AI MODE)
    # =====================================
    set name storage_ai
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
    "<div style='background:#6a0dad;color:white;padding:6px;'>💾 STORAGE INTELLIGENCE SYSTEM (AI MODE)</div>" \
    > $base_f

    echo "" >> $base_f
    echo "Timestamp: "(date) >> $base_f

    # =========================
    # BLOCK DEVICE MAP
    # =========================
    echo "" >> $base_f
    echo "## 🧩 BLOCK DEVICE MAP" >> $base_f

    lsblk -o NAME,SIZE,MODEL,TYPE,MOUNTPOINT,FSTYPE >> $base_f 2>/dev/null
    lsblk -J > "$raw_dir/lsblk.json" 2>/dev/null

    # =========================
    # FILESYSTEM USAGE
    # =========================
    echo "" >> $base_f
    echo "## 📊 FILESYSTEM USAGE" >> $base_f

    df -h >> $base_f 2>/dev/null
    df -h > "$fs_dir/df.md" 2>/dev/null

    # =========================
    # DEVICE CLASSIFICATION (SAFE ENUM)
    # =========================
    echo "" >> $base_f
    echo "## 🧠 DEVICE CLASSIFICATION" >> $base_f

    for p in /sys/block/*
        set d (basename $p)

        if test -z "$d"
            continue
        end

        set dev "/dev/$d"
        set model (cat /sys/block/$d/device/model 2>/dev/null)
        set rota (cat /sys/block/$d/queue/rotational 2>/dev/null)

        set out_file ""

        if test "$rota" = "0"
            set out_file "$ssd_dir/$d.md"
            echo "⚡ SSD/NVMe: $d" >> $base_f
        else if test "$rota" = "1"
            set out_file "$hdd_dir/$d.md"
            echo "🪵 HDD: $d" >> $base_f
        else
            set out_file "$usb_dir/$d.md"
            echo "🔌 UNKNOWN: $d" >> $base_f
        end

        echo "# 💽 DRIVE: $d" > $out_file
        echo "" >> $out_file
        echo "MODEL: $model" >> $out_file 2>/dev/null

        lsblk -o NAME,SIZE,TYPE,FSTYPE,MOUNTPOINT $dev >> $out_file 2>/dev/null

        # =========================
        # SMART DATA (NON-BLOCKING FIX)
        # =========================
        echo "" >> $out_file
        echo "## 🧠 SMART DATA" >> $out_file

        if command -q smartctl
            timeout 3 smartctl -a $dev >> $out_file 2>/dev/null
        else
            echo "smartctl not installed" >> $out_file
        end

    end

    # =========================
    # NVMe SPECIAL HANDLING
    # =========================
    echo "" >> $base_f
    echo "## ⚡ NVMe INTELLIGENCE" >> $base_f

    if command -q nvme
        timeout 3 nvme list >> $base_f 2>/dev/null
        nvme list > "$nvme_dir/nvme_list.md" 2>/dev/null
    else
        echo "nvme-cli not installed" >> $base_f
    end

    # =========================
    # ENCRYPTION LAYER
    # =========================
    echo "" >> $base_f
    echo "## 🔐 ENCRYPTION LAYER (LUKS)" >> $base_f

    lsblk -f >> $base_f 2>/dev/null
    ls /dev/mapper >> $base_f 2>/dev/null

    # =========================
    # RAW DATA
    # =========================
    echo "" >> $base_f
    echo "## 📦 RAW STORAGE DATA" >> $base_f

    lspci >> $base_f 2>/dev/null
    lsusb >> $base_f 2>/dev/null
    lsblk -f >> $base_f 2>/dev/null

    lsblk -f > "$raw_dir/lsblk.md" 2>/dev/null

    # =========================
    # SUMMARY
    # =========================
    echo "" >> $base_f
    echo "## 🧠 STORAGE SUMMARY" >> $base_f

    set total (df -h --total 2>/dev/null | tail -n 1 | awk '{print $2}')
    set used (df -h --total 2>/dev/null | tail -n 1 | awk '{print $3}')

    if test -n "$total"
        echo "- Total Storage: $total" >> $base_f
        echo "- Used Storage: $used" >> $base_f
    end

end
