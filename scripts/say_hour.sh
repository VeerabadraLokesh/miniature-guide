#!/usr/bin/env bash

### https://askubuntu.com/questions/686938/announce-the-time-on-the-hour
## sudo apt install espeak
my_date=$(date +'%H:%M')
padsp espeak "$my_date"
# chmod +x ~/bin/say_hour
# 0 * * * * bin/say_hour


## https://stackoverflow.com/questions/16365130/what-is-the-difference-between-usr-bin-env-bash-and-usr-bin-bash

### */15 * * * * /home/lokesh/workspace/git/miniature-guide/scripts/say_hour.sh > /dev/null 2>&1

