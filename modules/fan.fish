function fan -a dir

    set name (status function)

    set base_f "$dir/$name.md"

    set hw_dir "$dir/hwmon"
    set sensors_dir "$dir/sensors"
    set thermal_dir "$dir/thermal"

    mkdir -p $hw_dir $sensors_dir $thermal_dir

    # =========================
    # SAFE CACHE (FIXED)
    # =========================
    cyberdeck_env
    set cache $CYBERDECK_CACHE_DIR
    mkdir -p $cache

    # =========================
    # FAN
    # =========================
    set name fan
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
    "<div style='background:#6a0dad;color:white;padding:6px;'>🌬️ COOLING & FAN INTELLIGENCE SYSTEM</div>" \
    > $base_f

    echo "" >> $base_f
    echo "Timestamp: "(date) >> $base_f

    echo "" >> $base_f
    echo "## 🧠 SYSTEM OVERVIEW" >> $base_f
    uname -r >> $base_f

    # =========================
    # LM-SENSORS (PRIMARY DATA SOURCE)
    # =========================
    echo "" >> $base_f
    echo "## 🌡️ SENSOR STACK (lm-sensors)" >> $base_f

    if command -q sensors
        sensors >> $base_f
        sensors > $sensors_dir/sensors_raw.md
    else
        echo "lm-sensors not installed" >> $base_f
    end

    # =========================
    # HWMON FAN RPM DATA
    # =========================
    echo "" >> $base_f
    echo "## 🌪️ FAN RPM (HWMON)" >> $base_f

    if test -d /sys/class/hwmon
        for hw in /sys/class/hwmon/hwmon*
            set hwname (cat $hw/name 2>/dev/null)

            echo "" >> $base_f
            echo "### Device: $hwname" >> $base_f

            for fan in $hw/fan*_input
                if test -r $fan
                    set rpm (cat $fan)
                    set id (basename $fan)

                    echo "- $id: $rpm RPM" >> $base_f
                end
            end
        end
    end

    # =========================
    # FAN CONTROL INTERFACE (if available)
    # =========================
    echo "" >> $base_f
    echo "## ⚙️ FAN CONTROL LAYER" >> $base_f

    if test -f /etc/fancontrol
        echo "fancontrol config detected" >> $base_f
        cat /etc/fancontrol >> $base_f 2>/dev/null
    else
        echo "fancontrol not configured" >> $base_f
    end

    # =========================
    # ASUS / ACPI / WMI (important for your board)
    # =========================
    echo "" >> $base_f
    echo "## 🧩 ASUS / ACPI LAYER" >> $base_f

    if test -d /sys/class/thermal
        for t in /sys/class/thermal/thermal_zone*/temp
            if test -r $t
                set temp (math (cat $t) / 1000)
                echo "thermal zone: $temp°C" >> $base_f
            end
        end
    end

    # =========================
    # RAW SNAPSHOT
    # =========================
    echo "" >> $base_f
    echo "## 📦 RAW SYSTEM SNAPSHOT" >> $base_f

    lspci >> $base_f
    lsusb >> $base_f
    ls /sys/class/hwmon >> $hw_dir/list.md 2>/dev/null

    # =========================
    # SUMMARY
    # =========================
    echo "" >> $base_f
    echo "## 🧠 COOLING SUMMARY" >> $base_f

    if command -q sensors
        echo "✔ lm-sensors active" >> $base_f
    else
        echo "❌ lm-sensors missing" >> $base_f
    end

    if test -d /sys/class/hwmon
        echo "✔ hwmon interfaces detected" >> $base_f
    else
        echo "❌ no hwmon data found" >> $base_f
    end

end
