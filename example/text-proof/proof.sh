#!/bin/bash
if [ -z $1 ]; then
    echo "Usage: $0 <text>"
    exit 1
fi
./target/release/cli proof $(./target/release/cli address) $1
