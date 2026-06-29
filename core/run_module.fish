#!/usr/bin/env fish
# Arguments: module_name module_dir env_file helpers_file module_file
set m $argv[1]
set dir $argv[2]
set envfile $argv[3]
set helpers $argv[4]
set file $argv[5]

# Source everything and run
source $envfile
source $helpers
source $file

if functions -q $m
    $m $dir
    exit $status
else
    echo "❌ Function $m not defined" >&2
    exit 1
end
