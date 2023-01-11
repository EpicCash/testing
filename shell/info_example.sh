#!/bin/bash

WORLD_NAME=world
EPIC_WALLET_BINARY=/home/jhelison/Documents/epic/testing/binaries/epic-wallet

for dir in ./world/*/; do
    if [ $((RANDOM % 2)) -eq 1 ]; then
        continue
    fi

    WALLET_PASSWORD="$(basename "$dir")"

    $EPIC_WALLET_BINARY -c $dir -p $WALLET_PASSWORD --usernet info
done
