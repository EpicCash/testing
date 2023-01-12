#!/bin/bash

source variables.sh

for dir in ./world/*/; do
    if [ $((RANDOM % 2)) -eq 1 ]; then
        continue
    fi

    WALLET_PASSWORD="$(basename "$dir")"

    echo "Wallet $WALLET_PASSWORD listening to mine"
    sh listen_wallet.sh $EPIC_WALLET_BINARY $dir $WALLET_PASSWORD > /dev/null

    sleep 5
done
