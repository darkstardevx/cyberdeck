function dashboard -a dir

    clear

    echo "===================================="
    echo "     👁 CYBERDECK DASHBOARD"
    echo "===================================="

    echo ""

    echo "🧠 SYSTEM STATUS"
    uptime
    echo ""

    echo "🔥 CPU"
    lscpu | grep "Model name"
    echo ""

    echo "🌡 THERMALS"
    if command -q sensors
        sensors | head -n 10
    end

    echo ""

    echo "💾 DISKS"
    lsblk -d -o NAME,SIZE,MODEL

    echo ""

    echo "🌐 NETWORK"
    ip -br a

    echo ""

    echo "🔋 POWER"
    if command -q upower
        upower -i (upower -e | head -n 1)
    end

    echo ""
    echo "===================================="
    echo "LIVE MODE ACTIVE"
    echo "===================================="

end

