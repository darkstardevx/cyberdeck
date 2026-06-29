function cyberdeck_hud

    set file /tmp/cyberdeck_metrics.json

    while true
        clear

        echo "===================================="
        echo " 👁 CYBERDECK OS LEVEL 2 HUD"
        echo "===================================="

        if test -f $file
            cat $file | jq '.state'
            echo ""
            cat $file | jq '.mem'
            echo ""
            cat $file | jq '.temp' | head -n 20
        else
            echo "waiting for daemon..."
        end

        sleep 1
    end
end
