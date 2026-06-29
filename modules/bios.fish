function safe_run
    set m $argv[1]
    set dir $argv[2]

    # Point to the modules directory where your .fish scripts live
    set script_path "$HOME/Projects/.cyberdeck/modules/$m.fish"

    # Check if the script exists
    if not test -f "$script_path"
        echo "❌ Script not found: $script_path"
        return 1
    end

    # Explicitly call the fish interpreter to run the script
    timeout 5s fish "$script_path" "$dir"
    set exit_code $status

    if test $exit_code -eq 124
        echo "❌ TIMEOUT: $m"
    else if test $exit_code -ne 0
        echo "❌ ERROR: $m failed with code $exit_code"
    end
end

function bios -a dir

    set name (status function)

    set base_f "$dir/$name.md"

    set cpu_dir "$dir/cpu"
    set chipset_dir "$dir/chipset"
    set board_dir "$dir/motherboard"
    set mem_dir "$dir/memory"
    set kernel_dir "$dir/kernel"
    set power_dir "$dir/power"

    mkdir -p $cpu_dir $chipset_dir $board_dir $mem_dir $kernel_dir $power_dir

    # =========================
    # SAFE CACHE (FIXED)
    # =========================
    cyberdeck_env
    set cache $CYBERDECK_CACHE_DIR
    mkdir -p $cache

    # =========================
    # BIOS
    # =========================
    set name bios
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
    "<div style='background:#6a0dad;color:white;padding:6px;'>⚙️ BIOS & SYSTEM INTELLIGENCE</div>" \
    > $base_f

    echo "" >> $base_f
    echo "Timestamp: "(date) >> $base_f

    echo "" >> $base_f
    echo "## 🧠 SYSTEM OVERVIEW" >> $base_f
    uname -a >> $base_f

    # =========================
    # BIOS
    # =========================
    echo "" >> $base_f
    echo "## ⚙️ BIOS" >> $base_f
    sudo dmidecode -t bios >> $base_f 2>/dev/null

    echo "# ⚙️ BIOS" > "$board_dir/bios.md"
    sudo dmidecode -t bios >> "$board_dir/bios.md" 2>/dev/null

    # =========================
    # MOTHERBOARD
    # =========================
    echo "" >> $base_f
    echo "## 🧩 MOTHERBOARD (ASUS PRIME B450M-A II)" >> $base_f
    sudo dmidecode -t baseboard >> $base_f 2>/dev/null

    echo "# 🧩 MOTHERBOARD" > "$board_dir/motherboard.md"
    sudo dmidecode -t baseboard >> "$board_dir/motherboard.md" 2>/dev/null

    # =========================
    # CPU
    # =========================
    echo "" >> $base_f
    echo "## 🧠 CPU" >> $base_f
    lscpu >> $base_f

    echo "# 🧠 CPU" > "$cpu_dir/cpu.md"
    lscpu >> "$cpu_dir/cpu.md"

    if test -r /proc/cpuinfo
        cat /proc/cpuinfo >> "$cpu_dir/cpu.md"
    end

    # =========================
    # CHIPSET / PCI
    # =========================
    echo "" >> $base_f
    echo "## 🔌 CHIPSET / PCI" >> $base_f
    lspci >> $base_f

    echo "# 🔌 CHIPSET" > "$chipset_dir/chipset.md"
    lspci >> "$chipset_dir/chipset.md"

    # =========================
    # MEMORY
    # =========================
    echo "" >> $base_f
    echo "## 🧠 MEMORY" >> $base_f
    free -h >> $base_f

    echo "# 🧠 MEMORY" > "$mem_dir/memory.md"
    free -h >> "$mem_dir/memory.md"
    sudo dmidecode -t memory >> "$mem_dir/memory.md" 2>/dev/null

    # =========================
    # KERNEL / BOOT
    # =========================
    echo "" >> $base_f
    echo "## 🧬 KERNEL" >> $base_f
    uname -r >> $base_f
    cat /proc/cmdline >> $base_f

    echo "# 🧬 KERNEL" > "$kernel_dir/kernel.md"
    uname -a >> "$kernel_dir/kernel.md"
    cat /proc/cmdline >> "$kernel_dir/kernel.md"

    # =========================
    # POWER / CPU GOVERNOR / AMD PSTATE
    # =========================
    echo "" >> $base_f
    echo "## 🔋 POWER & CPU CONTROL" >> $base_f

    if command -q cpupower
        echo "### ⚙️ CPUPOWER INFO" >> $base_f
        cpupower frequency-info >> $base_f 2>/dev/null
    else
        echo "cpupower not installed" >> $base_f
    end

    echo "" >> $base_f
    echo "### 🧠 AMD PSTATE" >> $base_f

    if test -f /sys/devices/system/cpu/amd_pstate/status
        cat /sys/devices/system/cpu/amd_pstate/status >> $base_f
    else
        echo "amd_pstate not active" >> $base_f
    end

    if test -f /sys/devices/system/cpu/cpu0/cpufreq/scaling_driver
        echo "" >> $base_f
        echo "### ⚡ SCALING DRIVER" >> $base_f
        cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_driver >> $base_f
    end

    if test -f /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor
        echo "" >> $base_f
        echo "### 🎯 ACTIVE GOVERNOR" >> $base_f
        cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor >> $base_f
    end

    if test -f /sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors
        echo "" >> $base_f
        echo "### 🎛️ AVAILABLE GOVERNORS" >> $base_f
        cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors >> $base_f
    end

    echo "" >> $base_f
    echo "### 📊 CPU FREQUENCY SNAPSHOT" >> $base_f

    for cpu in /sys/devices/system/cpu/cpu[0-9]*/cpufreq/scaling_cur_freq
        if test -r $cpu
            set core (string replace -r ".*/cpu([0-9]+).*" '$1' $cpu)
            set freq (math (cat $cpu) / 1000)
            echo "Core $core: $freq MHz" >> $base_f
        end
    end

    # =========================
    # POWER MODULE FILE
    # =========================
    echo "# 🔋 POWER PROFILE" > "$power_dir/power.md"

    if command -q cpupower
        cpupower frequency-info >> "$power_dir/power.md" 2>/dev/null
    end

    if test -f /sys/devices/system/cpu/amd_pstate/status
        cat /sys/devices/system/cpu/amd_pstate/status >> "$power_dir/power.md"
    end

end
