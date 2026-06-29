function disks -a dir

    echo "RUNNING DISKS MODULE..." >> /tmp/cyberdeck_debug.log

    set base_f "$dir/disks.md"

    # =========================
    # SAFE CACHE (FIXED)
    # =========================
    cyberdeck_env
    set cache $CYBERDECK_CACHE_DIR
    mkdir -p $cache

    # =========================
    # OUTPUT STRUCTURE
    # =========================
    set nvme_dir "$dir/nvme"
    set ssd_dir "$dir/ssd"
    set hdd_dir "$dir/hdd"
    set usb_dir "$dir/usb"
    set media_dir "$dir/media"
    set raw_dir "$dir/raw"

    mkdir -p $nvme_dir $ssd_dir $hdd_dir $usb_dir $media_dir $raw_dir

    # =========================
    # DISKS
    # =========================
    set name disks
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
    "<div style='background:#6a0dad;color:white;padding:6px;'>💾 DISK INTELLIGENCE SYSTEM</div>" \
    > $base_f

    echo "" >> $base_f
    echo "Timestamp: "(date) >> $base_f

    # =========================
    # DEVICE ENUM (SAFE)
    # =========================
    echo "## 📦 BLOCK DEVICES" >> $base_f

    set devices (lsblk -dn -o NAME,TYPE 2>/dev/null | awk '$2=="disk"{print $1}')

    if test (count $devices) -eq 0
        echo "No disks detected" >> $base_f
        return
    end

    for d in $devices

        set dev "/dev/$d"

        set model (cat /sys/block/$d/device/model 2>/dev/null)
        set rota (cat /sys/block/$d/queue/rotational 2>/dev/null)

        set mountpoints (lsblk -nr -o MOUNTPOINT $dev 2>/dev/null)

        # =========================
        # CLASSIFICATION
        # =========================
        set out_file ""
        set label ""

        if string match -q "nvme*" $d
            set out_file "$nvme_dir/$d.md"
            set label "⚡ NVMe"

        else if test "$rota" = "0"
            set out_file "$ssd_dir/$d.md"
            set label "⚡ SSD"

        else
            set out_file "$hdd_dir/$d.md"
            set label "🪵 HDD"
        end

        if string match -q "sd*" $d
            if test -f /sys/block/$d/removable
                if test (cat /sys/block/$d/removable) = "1"
                    set out_file "$usb_dir/$d.md"
                    set label "🔌 USB"
                end
            end
        end

        # =========================
        # WRITE DEVICE FILE
        # =========================
        echo "# $label DRIVE: $dev" > $out_file
        echo "" >> $out_file
        echo "Model: $model" >> $out_file 2>/dev/null

        echo "" >> $out_file
        echo "## 📊 PARTITIONS / MOUNTS" >> $out_file
        lsblk -o NAME,SIZE,TYPE,FSTYPE,MOUNTPOINT $dev >> $out_file 2>/dev/null

        # =========================
        # MEDIA MOUNTS
        # =========================
        echo "" >> $out_file
        echo "## 📁 MEDIA MOUNTS" >> $out_file

        for m in $mountpoints
            if test -n "$m"

                if string match -q "/media/*" $m
                    echo "📌 Media mount: $m" >> $out_file
                    echo $m >> $media_dir/$d-media.md

                else if string match -q "/run/media/*" $m
                    echo "📌 User mount: $m" >> $out_file
                    echo $m >> $media_dir/$d-media.md
                end

            end
        end

        # =========================
        # SIZE
        # =========================
        if test -f /sys/block/$d/size
            set size (math (cat /sys/block/$d/size 2>/dev/null) "*" 512 / 1024 / 1024 / 1024 2>/dev/null)
            echo "" >> $out_file
            echo "## 📏 SIZE" >> $out_file
            echo "$size GB (approx)" >> $out_file
        end

        # =========================
        # SMART
        # =========================
        echo "" >> $out_file
        echo "## 🧠 SMART HEALTH" >> $out_file

        if command -q smartctl
            timeout 2 smartctl -H $dev >> $out_file 2>/dev/null
            or echo "SMART unavailable" >> $out_file
        end

    end

    # =========================
    # NVME SUMMARY
    # =========================
    echo "" >> $base_f
    echo "## ⚡ NVME SUMMARY" >> $base_f

    if command -q nvme
        timeout 2 nvme list >> $base_f 2>/dev/null
    else
        echo "nvme-cli not installed" >> $base_f
    end

    # =========================
    # RAW SNAPSHOT
    # =========================
    echo "" >> $base_f
    echo "## 📦 RAW SNAPSHOT" >> $base_f

    lsblk -f >> $base_f 2>/dev/null
    df -h >> $base_f 2>/dev/null

    echo "" >> $base_f
    echo "✔ disk scan complete" >> $base_f

    # =========================
    # CACHE EXPORT (FIXED LOCATION)
    # =========================
    cp $base_f $cache/disks.md 2>/dev/null

end
