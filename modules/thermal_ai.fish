# ~/Projects/.cyberdeck/modules/thermal_ai.fish

# 👇 ADD THIS LINE
source /home/raven/Projects/.cyberdeck/core/output_writer.fish

function thermal_ai -a base_dir
    # ... rest of your code ...
    cyber_write_header $base_f "THERMAL INTELLIGENCE" "🌡️" # Now this will work
    # ...
end

function thermal_ai -a dir

    # =========================
    # STRUCTURE
    # =========================
    set raw_dir "$dir/raw"
    set parsed_dir "$dir/parsed"

    mkdir -p $raw_dir
    mkdir -p $parsed_dir

    set f "$dir/thermal_ai.md"
    set raw_f "$raw_dir/raw_temps.md"
    set parsed_f "$parsed_dir/parsed_temps.md"

    # =========================
    # SAFE CACHE (FIXED)
    # =========================
    cyberdeck_env
    set cache $CYBERDECK_CACHE_DIR
    mkdir -p $cache

    # =========================
    # THERMAL AI
    # =========================
    set name thermal_ai
    set base_f "$dir/$name.md"

    # hard safety check (IMPORTANT)
    if test -z "$base_f"
        echo "❌ base_f is empty - aborting module"
        return 1
    end

    # =========================
    # HEADER
    # =========================
    cyber_write_header $base_f "THERMAL INTELLIGENCE" "🌡️"

    echo "" >> $base_f
    echo "Timestamp: "(date) >> $base_f

    # =========================
    # SAFE SENSOR DISCOVERY
    # =========================
    set zones /sys/class/thermal/thermal_zone*/temp

    # IMPORTANT: detect real expansion (Fish-safe)
    if test -e $zones[1] 2>/dev/null; and test -r $zones[1] 2>/dev/null

        set max 0
        set index 1

        echo "" >> $f
        cyber_write_section $base_f "RAW SENSOR DATA" "📡"

        for t in $zones

            if test -r $t

                set raw (cat $t 2>/dev/null)

                # skip invalid values
                if test -z "$raw"
                    continue
                end

                set val (math "$raw / 1000")

                echo "Zone $index: $val°C" >> $f
                echo "$val" >> $raw_f

                # max tracking (SAFE FISH COMPARISON)
                if test (math "$val > $max")
                    set max $val
                end

                set index (math "$index + 1")
            end
        end

    else
        echo "" >> $f
        echo "## ⚠️ Thermal zones not found" >> $f

        if command -q sensors
            echo "" >> $f
            cyber_write_section $base_f "LM-SENSORS OUTPUT" "🧪"
            sensors >> $f 2>/dev/null
            sensors > $raw_f 2>/dev/null
        else
            echo "No thermal sensors available" >> $f
        end

        set max 0
    end

    # =========================
    # PARSED SUMMARY
    # =========================
    echo "# 🌡️ PARSED TEMPERATURE SUMMARY" > $parsed_f

    if set -q max
        echo "- Peak temperature: $max°C" >> $parsed_f
    else
        set max 0
        echo "- Peak temperature: unknown" >> $parsed_f
    end

    echo "- Sensors detected: "(count $zones 2>/dev/null) >> $parsed_f

    # =========================
    # FINAL SUMMARY
    # =========================
    echo "" >> $f
    cyber_write_section $base_f "THERMAL SUMMARY" "🧠"
    echo "- Peak temperature: $max°C" >> $f
    echo "- Sensor count: "(count $zones 2>/dev/null) >> $f

end
