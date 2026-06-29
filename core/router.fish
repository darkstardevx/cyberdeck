function router -a module

    set base ~/.cyberdeck
    set dir "$HOME/.cyberdeck/output/$module"
    mkdir -p $dir

    # optional HUD (make sure it's NON-BLOCKING)
    fish $base/core/hud.fish $module >/dev/null 2>&1 &
    switch $module

        case hardware
            source $base/modules/hardware.fish
            hardware $dir

        case disks
            source $base/modules/disks.fish
            disks $dir

        case network
            source $base/modules/network.fish
            network $dir

        case fan
            source $base/modules/fan.fish
            fan $dir

        case bios
            source $base/modules/bios.fish
            bios $dir

        case audio
            source $base/modules/audio.fish
            audio $dir

        case ethernet
            source $base/modules/ethernet.fish
            ethernet $dir

        case memory
            source $base/modules/memory.fish
            memory $dir

        case power
            source $base/modules/power.fish
            power $dir

        case storage_ai
            source $base/modules/storage_ai.fish
            storage_ai $dir

        case thermal_ai
            source $base/modules/thermal_ai.fish
            thermal_ai $dir

    end

end
