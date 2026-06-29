function hud -a label

    for i in (seq 1 3)

        printf "\r\e[38;5;129m SCANNING %-15s ▓▒▒" $label
        sleep 0.03

        printf "\r\e[38;5;213m SCANNING %-15s ▓▓▒" $label
        sleep 0.03

        printf "\r\e[38;5;51m SCANNING %-15s ▓▓▓\e[0m" $label
        sleep 0.03
    end

    printf "\r\e[38;5;70m COMPLETE %-15s ✓\e[0m\n" $label
end

