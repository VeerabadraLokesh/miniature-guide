#!/bin/bash

## https://linuxize.com/post/bash-functions/
## https://stackoverflow.com/questions/12199059/how-to-check-if-an-url-exists-with-the-shell-and-probably-curl
check_url() {
    url="$1"
    if curl --output /dev/null --silent --head --fail "$url"; then
        echo "URL exists: $url"
    else
        echo "URL does not exist: $url"
    fi
}

## https://stackoverflow.com/questions/2683279/how-to-detect-if-a-script-is-being-sourced
# echo $_
# echo $0
# [[ $_ != $0 ]] && echo "Script is being sourced" || echo "Script is a subshell"

foo() {
    echo foo $1
}

main() {
    foo 1
    foo 2
}

## https://stackoverflow.com/questions/12815774/importing-functions-from-a-shell-script
if [ "${1}" != "--source-only" ]; then
    main "${@}"
fi