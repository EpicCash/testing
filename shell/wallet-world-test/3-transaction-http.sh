#!/bin/bash

source ./variables.sh

# Do HTTP transactions between the wallets on the wallet world
# TODO: Update it to do the same as ./3-transaction-file.sh
# TODO: Update it to check if the sender and the reciever are the same

echo "------------------------Running file transaction------------------------"

for DIR in ./world/*/; do
    WALLET_PASSWORD="$(basename "$DIR")"

    $EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet txs

    exit 1

    # Skip if the wallet has no funds
    INFO=$($EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet info)
    SPENDABLE_INFO=$(echo "$INFO" | grep -o "Currently Spendable.*" | awk '{print $NF}')
    HAS_FUNDS=$(echo "$SPENDABLE_INFO == 0" | bc -l)
    if [ "$HAS_FUNDS" -eq 1 ]; then
        echo "Skipping wallet $WALLET_PASSWORD due to no funds"
        continue
    fi

    # Do 3 transactions
    for i in $(seq 1 5); do
        # Get a new random wallet to recieve the transaction
        WALLET_DIRECTORY=$(find "./world" -mindepth 1 -maxdepth 1 -type d)
        RANDOM_DIRECTORY=$(echo "$WALLET_DIRECTORY" | shuf -n 1)
        RECIEVER_WALLET=$(basename "$RANDOM_DIRECTORY")

        # Make the reciever wallet listen
        $EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet listen > /dev/null &
        WALLET_PID=$!

        # Generate a new file sender string
        echo "Making a HTTP transaction with $WALLET_PASSWORD as sender and wallet $RECIEVER_WALLET as reciever"
        $EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet send -d "http://localhost:23415" 1.1 > /dev/null

        sleep 1

        kill $WALLET_PID
    done
done
