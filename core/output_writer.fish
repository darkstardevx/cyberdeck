#!/usr/bin/env fish

# =========================================================
# CYBERDECK OUTPUT WRITER (CORE SYSTEM)
# =========================================================

function cyber_write_header -a file title icon color
    set color (or $color "#6a0dad")

    printf "%s\n" \
    "<div style='background:$color;color:white;padding:8px;font-weight:bold;border-radius:4px;'>"\
    "$icon $title"\
    "</div>" > $file
end


function cyber_write_kv -a file key value state
    # Update any usage of $state inside the function to use $state
    echo "$key=$value" >> $file
    # Example: if test "$state" = "ok" ...
end




function cyber_write_section -a file title
    echo "" >> $file
    echo "## $title" >> $file
end


function cyber_write_raw -a file label content
    echo "" >> $file
    echo "<div style='background:#1e1e1e;color:#00ff9f;padding:6px;border-radius:4px;'>" >> $file
    echo "$label" >> $file
    echo "</div>" >> $file
    echo "" >> $file
    echo $content >> $file
end
