function cyber_fan -a level

    # WARNING: depends on hardware support
    set pwm "/sys/class/hwmon/hwmon0/pwm1"

    if test -e $pwm

        switch $level
            case low
                echo 80 | sudo tee $pwm

            case mid
                echo 160 | sudo tee $pwm

            case high
                echo 255 | sudo tee $pwm
        end

        echo "FAN SET TO $level"
    else
        echo "NO PWM FAN CONTROL DETECTED"
    end
end
