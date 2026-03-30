#!/bin/bash

my_address="$(./target/release/cli address)"
pair_address="$1"

previous_log=""
reload_display() {
    log=$(IFS=$'\n'; for item in $(./target/release/cli find-by-pair "$my_address" "$pair_address" | jq -c '.[]'); do
        is_first="$(echo $item | jq .is_first)"
        echo "[$([ $is_first = "true" ] && echo "mine" || echo "other")] $(echo $item | jq .content)"
    done)
    if [ "$log" != "$previous_log" ]; then
        clear
        echo "$log"
        previous_log="$log"
    fi
}

clear
reload_display

while true;
do
    read -t 1 message
    if [ $? -ne 0 ]; then
        reload_display
    else
        target/release/cli proof "$pair_address" "$(echo -n $message)" > /dev/null
        previous_log=""
        reload_display
    fi
done
