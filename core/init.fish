function safe_run -a m dir
    set base /home/raven/Projects/.cyberdeck
    set envfile "$base/core/env.fish"
    set helpers "$base/core/output_writer.fish"
    set module_file "$base/modules/$m.fish"

    echo "running: $m"

    # Run everything in ONE subshell command.
    # Variables expand HERE in the parent, then the paths are baked into the command string.
    timeout 5 fish -c "
        source $envfile
        source $helpers
        source $module_file
        if functions -q '$m'
            '$m' '$dir'
        else
            echo '❌ Function $m not defined' >&2
            exit 1
        end
    "

    set exit_code $status
    if test $exit_code -eq 124
        echo "❌ TIMEOUT: $m"
        return 1
    elseif test $exit_code -ne 0
        echo "❌ FAILED: $m (exit code: $exit_code)"
        return 1
    end
end


