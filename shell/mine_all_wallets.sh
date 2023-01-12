#!/bin/bash

WORLD_NAME=world
EPIC_WALLET_BINARY=/home/jualns/Desktop/epic-wallet/target/release/epic-wallet

for dir in ./world/*/; do
    if [ $((RANDOM % 2)) -eq 1 ]; then
        continue
    fi

    WALLET_PASSWORD="$(basename "$dir")"

    sh listen_wallet.sh $EPIC_WALLET_BINARY $dir $WALLET_PASSWORD
done
