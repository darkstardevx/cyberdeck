#source $HOME/Projects/.cyberdeck/core/init.fish

function __cyberdeck_timestamp
    date +%Y-%m-%d_%H-%M-%S
end

# =========================
# SAFE RUN (FIXED)
# =========================
function safe_run -a m dir
    set base /home/raven/Projects/.cyberdeck
    set file "$base/modules/$m.fish"
    set envfile "$base/core/env.fish"

    echo "running: $m"

    # Run in a subshell to isolate errors
    fish -c "
        source '$envfile'
        source '$file'
        if functions -q '$m'
            '$m' '$dir'
        else
            echo '❌ Function $m not defined after sourcing $file' >&2
            exit 1
        end
    " &

    set cmd_pid (jobs -lp)
    set elapsed 0

    while kill -0 $cmd_pid 2>/dev/null
        if test $elapsed -ge 5
            kill -TERM $cmd_pid 2>/dev/null
            wait $cmd_pid 2>/dev/null
            echo "❌ TIMEOUT: $m"
            return 1
        end
        sleep 1
        set elapsed (math $elapsed + 1)
    end

    wait $cmd_pid
    if test $status -ne 0
        echo "❌ FAILED: $m"
        return 1
    end
end

# =========================
# GLOBAL CONFIG
# =========================
set -g base /home/raven/Projects/.cyberdeck
set -g modules hardware disks network fan bios audio ethernet memory power storage_ai thermal_ai full_scan
set -g reserved_modules dashboard


# =========================
# FULL SCAN
# =========================
function full_scan -a base_dir

    set ts (__cyberdeck_timestamp)
    set dir "$base_dir/output/full_scan/$ts"
    mkdir -p $dir

    set scan_modules hardware disks network fan bios audio ethernet memory power storage_ai thermal_ai dashboard

    echo "# 👁 CYBERDECK FULL SCAN REPORT" > $dir/FULL_REPORT.md
    echo "" >> $dir/FULL_REPORT.md
    echo "## 🧾 SYSTEM SUMMARY" >> $dir/FULL_REPORT.md
    echo "- Scan time: $ts" >> $dir/FULL_REPORT.md
    echo "- Status: COMPLETE" >> $dir/FULL_REPORT.md

    echo "" >> $dir/FULL_REPORT.md
    echo "## 📂 MODULE REPORTS" >> $dir/FULL_REPORT.md

    for m in $scan_modules

        echo "Running $m..."

        set mod_dir "$dir/$m"
        mkdir -p $mod_dir

        set file "$base_dir/modules/$m.fish"

        if test -f $file
            source $file
        else
            echo "❌ missing file: $m" >> $dir/FULL_REPORT.md
            continue
        end

        if functions -q $m
            echo "✔ running $m"
            safe_run $m $mod_dir

            echo "" >> $dir/FULL_REPORT.md
            echo "### 🧿 $(string upper $m)" >> $dir/FULL_REPORT.md
            echo "📄 [$m.md]($m/$m.md)" >> $dir/FULL_REPORT.md
            echo "✔ status: complete" >> $dir/FULL_REPORT.md
        else
            echo "❌ function missing: $m" >> $dir/FULL_REPORT.md
        end

        echo "------------------------" >> $dir/FULL_REPORT.md
    end

    echo "" >> $dir/FULL_REPORT.md
    echo "FULL SCAN COMPLETE" >> $dir/FULL_REPORT.md
    echo "REPORT: $dir/FULL_REPORT.md" >> $dir/FULL_REPORT.md

    echo "✔ FULL SCAN COMPLETE: $dir/FULL_REPORT.md"
end


# =========================
# MAIN UI
# =========================
function oms --description "Cyberdeck OS"
    clear

    set selected (printf "%s\n" $modules | fzf --multi --border --header "CYBERDECK SELECTOR")

    # Check if fzf was aborted (status != 0)
    if test $status -ne 0
        echo "aborted"
        return # ✅ Use return to exit function, not continue
    end

    # Loop through selected modules
    for m in $selected
        # ✅ dashboard check MUST be inside the loop
        if test "$m" = "dashboard"
            echo "🧩 dashboard is reserved (not active module yet)"
            continue # ✅ Valid here because we are inside 'for m in $selected'
        end

        echo "=============================="
        echo "MODULE: $m"
        echo "=============================="

        # FULL SCAN
        if test "$m" = "full_scan"
            full_scan $base
            continue
        end

        set ts (__cyberdeck_timestamp)
        set run_dir "$base/output/$m/$ts"
        mkdir -p $run_dir

        set file "$base/modules/$m.fish"

        if not test -f $file
            echo "❌ missing module: $m"
            continue
        end

        source $file

        if functions -q $m
            echo "▶ running: $m"
            safe_run $m $run_dir
        else
            echo "❌ function not found: $m"
        end
    end
end
