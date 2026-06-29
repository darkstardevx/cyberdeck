function cyberdeck_env

    # =========================
    # ROOT PATH (SOURCE OF TRUTH)
    # =========================
    set -gx CYBERDECK_ROOT /home/raven/Projects/.cyberdeck

    # =========================
    # OUTPUT + CACHE PATHS
    # =========================
    set -gx CYBERDECK_OUTPUT "$CYBERDECK_ROOT/output"
    set -gx CYBERDECK_CACHE_DIR "$CYBERDECK_ROOT/cache"
    set -gx CYBERDECK_MODULES "$CYBERDECK_ROOT/modules"

    # =========================
    # ENSURE DIRECTORIES EXIST
    # =========================
    mkdir -p $CYBERDECK_OUTPUT
    mkdir -p $CYBERDECK_CACHE_DIR

end
