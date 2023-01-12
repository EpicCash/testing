#!/bin/bash

source variables.sh

for dir in ./world/*/; do
    if [ $((RANDOM % 2)) -eq 1 ]; then
        continue
    fi

    WALLET_PASSWORD="$(basename "$dir")"

    $EPIC_WALLET_BINARY -c $dir -p $WALLET_PASSWORD --usernet info
done
