#!/bin/bash


read -r -d '' USAGE << 'EOF'
usage: clicksnap-screenshot.sh [options] [appname]

    appname:
        name of appliance

        default: auto-detect when run from inside a products directory

    -h/--help
        output this information and exit

    -i/--inithooks-conf "path"
        inithooks config, containing password, admin email, etc.

        envvar: TKLDEV_DOCKER_INITHOOKS_CONF
        default: auto-detect if tkldev-dockerize is installed

    -c/--clicksnap-bin "path"
        clicksnap binary to use

        envvar: CLICKSNAP_BIN
        default: auto-detect if clicksnap is installed, or in standard turnkey source path

    -p/--screenshots-path "path"
        path to directory where screenshots should be put

        envvar: SCREENSHOTS_PATH_ABSOLUTE
        default: <unset>
        conflicts: -r/--screenshots-root

    -r/--screenshots-root "path"
        path to directory where screenshot subdirectories should be put

        envvar: SCREENSHOTS_PATH_ROOT
        default: /tmp/screenshots
        conflicts: -p/--screenshots-path

    -u/--url "url"
        url to point clicksnap towards

        envvar: APPLIANCE_URL
        default: auto-detect based off first ip from `hostname -I`
EOF

if [[ -t 1 ]] && [[ "$(tput colors 2>/dev/null)" -ge 8 ]]; then
    FG_RED="\033[0;31m"
    FG_YELLOW="\033[0;33m"
    T_RESET="\033[0m"
else
    FG_RED=
    FG_YELLOW=
    T_RESET=
fi

function warn() {
    echo -e "${FG_YELLOW}warn: $*$T_RESET"
}

function fatal() {
    echo
    echo -e "${FG_RED}error: $*$T_RESET"
    echo "$USAGE"
    exit 1
}

# ENVVARS:
#
# - TKLDEV_DOCKER_INITHOOKS_CONF
# - CLICKSNAP_BIN
# - SCREENSHOTS_PATH_ABSOLUTE
# - SCREENSHOTS_PATH_ROOT
# - APPLIANCE_URL

if [[ -n "$SCREENSHOTS_PATH_ABSOLUTE" ]] && [[ -n "$SCREENSHOTS_PATH_ROOT" ]]; then
    fatal 'only one of "SCREENSHOTS_PATH_ABSOLUTE" OR "SCREENSHOTS_PATH_ROOT" may be set, not both'
fi

if [[ -z "$SCREENSHOTS_PATH_ROOT" ]]; then
    SCREENSHOTS_PATH_ROOT=/tmp/screenshots
fi

SCREENSHOTS_PATH_ABSOLUTE_SET=
SCREENSHOTS_PATH_ROOT_SET=

POSITIONAL_ARGS=()

while [[ $# -gt 0 ]]; do
    case $1 in 
        -i|--inithooks-conf)
            TKLDEV_DOCKER_INITHOOKS_CONF="$2"
            shift
            shift
            ;;
        -c|--clicksnap-bin)
            CLICKSNAP_BIN="$2"
            shift
            shift
            ;;
        -p|--screenshots-path)
            SCREENSHOTS_PATH_ABSOLUTE="$2"
            SCREENSHOTS_PATH_ABSOLUTE_SET=1
            if [[ -n "$SCREENSHOTS_PATH_ROOT_SET" ]]; then
                fatal 'only one of "-p/--screenshots-path" OR "-r/--screenshot-root" may be set, not both'
            fi
            shift
            shift
            ;;
        -r|--screenshots-root)
            SCREENSHOTS_PATH_ROOT="$2"
            SCREENSHOTS_PATH_ROOT_SET=1
            if [[ -n "$SCREENSHOTS_PATH_ABSOLUTE_SET" ]]; then
                fatal 'only one of "-p/--screenshots-path" OR "-r/--screenshot-root" may be set, not both'
            fi
            shift
            shift
            ;;
        -u|--url)
            APPLIANCE_URL="$2"
            shift
            shift
            ;;
        -h|--help)
            echo "$USAGE"
            exit
            ;;
        --*|-*)
            fatal "unknown optional argument \"$1\""
            ;;
        *)
            POSITIONAL_ARGS+=("$1")
            shift
            ;;
    esac
done

set -- "${POSITIONAL_ARGS[@]}"

if [[ -z "$CLICKSNAP_BIN" ]]; then
    echo "clicksnap binary not specified, defaulting to auto detect"
    CLICKSNAP_BIN="$(which clicksnap)"
    CLICKSNAP_DEBUG_BIN="/turnkey/public/clicksnap/target/debug/clicksnap"
    CLICKSNAP_RELEASE_BIN="/turnkey/public/clicksnap/target/debug/clicksnap"

    if [[ ! -f "$CLICKSNAP_BIN" ]]; then
        warn "clicksnap not installed, checking for repo"
        if [[ -f "$CLICKSNAP_DEBUG_BIN" ]] && [[ -f "$CLICKSNAP_RELEASE_BIN" ]]; then
            warn "multiple built clicksnap instances found, choosing newest"
            if [[ "$CLICKSNAP_DEBUG_BIN" -nt "$CLICKSNAP_RELEASE_BIN" ]]; then
                CLICKSNAP_BIN="$CLICKSNAP_DEBUG_BIN"
            else
                CLICKSNAP_BIN="$CLICKSNAP_RELEASE_BIN"
            fi
        elif [[ -f "$CLICKSNAP_DEBUG_BIN" ]]; then
            CLICKSNAP_BIN="$CLICKSNAP_DEBUG_BIN"
        elif [[ -f "$CLICKSNAP_RELEASE_BIN" ]]; then
            CLICKSNAP_BIN="$CLICKSNAP_RELEASE_BIN"
        else
            fatal "no clicksnap binary found, please install and/or build clicksnap"
        fi
    fi
    echo "  detected \"$CLICKSNAP_BIN\""
fi
if [[ ! -f "$CLICKSNAP_BIN" ]]; then
    fatal "clicksnap binary \"$CLICKSNAP_BIN\" does not exist"
fi

if [[ -z "$TKLDEV_DOCKER_INITHOOKS_CONF" ]]; then
    TKLDEV_DOCKER_INITHOOKS_CONF="$(dirname "$(realpath "$(command -v dockerize)")")/inithooks.conf"

    echo "inithooks not specified, defaulting to tkldev-docker's inithooks.conf"

    echo "  detected \"$TKLDEV_DOCKER_INITHOOKS_CONF\""
fi
if [[ ! -f "$TKLDEV_DOCKER_INITHOOKS_CONF" ]]; then
    fatal "could not find dockerize (tkldev-docker)'s inithooks.conf"
fi

APPNAME="$1"

if [[ -z "$APPNAME" ]]; then
    echo "appliance name not specified, defaulting to auto detect"
    if [[ "$(pwd)" == "/turnkey/fab/products/*" ]]; then
        APPNAME="$(pwd | sed -E 's|^/turnkey/fab/products/([a-zA-Z0-9]+)(/.*)?|\1|')"
        echo "appname not specified, defaulting to \"$APPNAME\""
    else
        fatal "could not determine which appliance to screenshot"
    fi
    echo "  detected \"$APPNAME\""
fi


if [[ -z "$APPLIANCE_URL" ]]; then
    echo "appliance url not specified, defaulting to auto detect (may not work depending on how many interfaces you have)"

    APPLIANCE_URL="https://$(hostname -I | cut -d' ' -f1)"

    echo "  detected \"$APPLIANCE_URL\""
fi

if [[ -n "$SCREENSHOTS_PATH_ABSOLUTE_SET" ]]; then
    export TKL_SCREENSHOT_PATH="$SCREENSHOTS_PATH_ABSOLUTE"
else
    export TKL_SCREENSHOT_PATH="$SCREENSHOTS_PATH_ROOT/$APPNAME"
fi

mkdir -p "$TKL_SCREENSHOT_PATH"

. "$TKLDEV_DOCKER_INITHOOKS_CONF"
"$CLICKSNAP_BIN" test "$APPNAME" "$APPLIANCE_URL"
