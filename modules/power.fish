echo "RUNNING POWER MODULE..." >> /tmp/cyberdeck_debug.log

function power -a dir

    set name (status function)
    set base_f "$dir/$name.md"

    set cpu_dir "$dir/cpu"
    set gov_dir "$dir/governor"
    set state_dir "$dir/state"
    set energy_dir "$dir/energy"
    set raw_dir "$dir/raw"

    mkdir -p $cpu_dir $gov_dir $state_dir $energy_dir $raw_dir

    # =========================
    # SAFE CACHE (FIXED)
    # =========================
    cyberdeck_env
    set cache $CYBERDECK_CACHE_DIR
    mkdir -p $cache

    # =========================
    # POWER
    # =========================
    echo "# 🔋 POWER & PERFORMANCE INTELLIGENCE SYSTEM" > $base_f

    echo "" >> $base_f
    echo "## 🧠 SYSTEM OVERVIEW" >> $base_f
    uname -r >> $base_f
    uptime >> $base_f

    # =========================
    # CPU POWER STATE
    # =========================
    echo "" >> $base_f
    echo "## ⚡ CPU POWER STATE" >> $base_f

    lscpu >> $base_f

    echo "# ⚡ CPU INFO" > "$cpu_dir/cpu.md"
    lscpu >> "$cpu_dir/cpu.md"

    # =========================
    # GOVERNOR / SCALING
    # =========================
    echo "" >> $base_f
    echo "## 🎛️ CPU GOVERNOR / SCALING" >> $base_f

    for cpu in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
        if test -r $cpu
            set core (string replace -r ".*/cpu([0-9]+).*" '$1' $cpu)
            set gov (cat $cpu)
            echo "Core $core: $gov" >> $base_f
        end
    end

    # available governors
    if test -f /sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors
        echo "" >> $base_f
        echo "### AVAILABLE GOVERNORS" >> $base_f
        cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors >> $base_f
    end

    # =========================
    # UPOWER (FIXED SAFE BLOCK)
    # =========================
    echo "" >> $base_f
    echo "## 🔋 ENERGY STATUS" >> $base_f

    if command -q upower
        set dev (upower -e 2>/dev/null | head -n 1)

        if test -n "$dev"
            timeout 2 upower -i $dev >> $base_f 2>/dev/null
        else
            echo "upower: no devices detected" >> $base_f
        end
    else
        echo "upower not installed" >> $base_f
    end

    # =========================
    # AMD PSTATE
    # =========================
    echo "" >> $base_f
    echo "### 🧠 AMD PSTATE DRIVER" >> $base_f

    if test -f /sys/devices/system/cpu/amd_pstate/status
        cat /sys/devices/system/cpu/amd_pstate/status >> $base_f
    else
        echo "amd_pstate not active or unavailable" >> $base_f
    end

    # scaling driver
    if test -f /sys/devices/system/cpu/cpu0/cpufreq/scaling_driver
        echo "" >> $base_f
        echo "### ⚙️ SCALING DRIVER" >> $base_f
        cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_driver >> $base_f
    end

    # =========================
    # FREQUENCY SNAPSHOT
    # =========================
    echo "" >> $base_f
    echo "## 📊 FREQUENCY SNAPSHOT" >> $base_f

    for cpu in /sys/devices/system/cpu/cpu[0-9]*/cpufreq/scaling_cur_freq
        if test -r $cpu
            set core (string replace -r ".*/cpu([0-9]+).*" '$1' $cpu)
            set freq (math (cat $cpu) / 1000)
            echo "Core $core: $freq MHz" >> $base_f
        end
    end

    # structured frequency file
    echo "# 📊 FREQUENCY" > "$state_dir/frequency.md"
    for cpu in /sys/devices/system/cpu/cpu[0-9]*/cpufreq/scaling_cur_freq
        if test -r $cpu
            set core (string replace -r ".*/cpu([0-9]+).*" '$1' $cpu)
            set freq (math (cat $cpu) / 1000)
            echo "Core $core: $freq MHz" >> $state_dir/frequency.md
        end
    end

    # =========================
    # SYSTEM POWER STATE
    # =========================
    echo "" >> $base_f
    echo "## 🔌 SYSTEM POWER STATE" >> $base_f

    if test -f /sys/power/state
        cat /sys/power/state >> $base_f
    end

    # =========================
    # LOAD
    # =========================
    echo "" >> $base_f
    echo "## 📈 SYSTEM LOAD" >> $base_f

    cat /proc/loadavg >> $base_f
    vmstat 1 1 2>/dev/null | head -n 5 >> $base_f

    # =========================
    # RAW DATA
    # =========================
    echo "" >> $base_f
    echo "## 📦 RAW POWER DATA" >> $base_f

    lspci >> $base_f
    lsusb >> $base_f

    echo "# 📦 RAW POWER" > "$raw_dir/raw.md"
    cat /proc/loadavg >> "$raw_dir/raw.md"
    cat /sys/power/state >> "$raw_dir/raw.md" 2>/dev/null

    # =========================
    # SUMMARY
    # =========================
    echo "" >> $base_f
    echo "## 🧠 POWER SUMMARY" >> $base_f

    if command -q cpupower
        echo "✔ cpupower active" >> $base_f
    end

    if test -f /sys/devices/system/cpu/amd_pstate/status
        echo "✔ amd_pstate detected" >> $base_f
    end

end
