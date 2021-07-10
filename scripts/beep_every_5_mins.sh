#!/usr/bin/env bash

# PROGNAME=$0

# bash script get arguments with hyphen

# https://unix.stackexchange.com/questions/321126/dash-arguments-to-shell-scripts

# https://www.baeldung.com/linux/use-command-line-arguments-in-bash-script

# usage() {
#   cat << EOF >&2
# Usage: $PROGNAME [-v <level>] [-d <dir>] [-f <file>]
#        -v: verbose on <level (0, 1, 2, 3)>
# EOF
#   exit 1
# }

verbose_level=0


function log_debug () {
    if [[ $verbose_level -ge 1 ]]; then
        echo "$@"
    fi
}

# t=$(date +'%H:%M')
echo "This program will beep every 5th minute, sox needs to be installed for this to work"
echo "Press <CTRL+C> to exit."
while  :
do
    minuts=$(date +'%M')
    secs=$(date +'%S')
    log_debug "$minuts:$secs"
    sleep_time=60
    if (( 10#$minuts%5 == 0 ))
    then
        play -q -n synth 0.5 sin 880 > /dev/null 2>&1
        sleep_time=$((300 - 10#$secs + 1))
    else
        rem_minuts=$((5 - 10#$minuts%5))
        log_debug $rem_minuts
        sleep_time=$((10#$rem_minuts*60 - 10#$secs + 1))
        log_debug $sleep_time
    fi
    sleep $sleep_time
done
