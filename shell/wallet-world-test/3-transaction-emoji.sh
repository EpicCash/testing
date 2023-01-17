#!/bin/bash

# Do emoji transactions between the wallets on the wallet world
# TODO: Update it to do the same as ./3-transaction-file.sh

source ./variables.sh

for DIR in ./world/*/; do
    WALLET_PASSWORD="$(basename "$DIR")"

    # Skip if the wallet has no funds
    INFO=$($EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet info)
    SPENDABLE_INFO=$(echo "$INFO" | grep -o "Currently Spendable.*" | awk '{print $NF}')
    HAS_FUNDS=$(echo "$SPENDABLE_INFO == 0" | bc -l)
    if [ "$HAS_FUNDS" -eq 1 ]; then
        echo "Skipping wallet $WALLET_PASSWORD due to no funds"
        continue
    fi

    # Do 3 transactions
    for i in $(seq 1 3); do
        # Get a new random wallet to recieve the transaction
        WALLET_DIRECTORY=$(find "./world" -mindepth 1 -maxdepth 1 -type d)
        RANDOM_DIRECTORY=$(echo "$WALLET_DIRECTORY" | shuf -n 1)
        RECIEVER_WALLET=$(basename "$RANDOM_DIRECTORY")

        # Generate a new emoji sender string
        echo "Making a emoji transaction with $WALLET_PASSWORD as sender and wallet $RECIEVER_WALLET as reciever"
        SEND_EMOJI=$($EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet send -m emoji 1 | awk '{split($0,a,"Command"); print a[1]}')

        # Recieve the Epics on the target wallet
        echo "Recieving a emoji transaction with $WALLET_PASSWORD as sender and wallet $RECIEVER_WALLET as reciever"
        RECIEVER_EMOJI=$($EPIC_WALLET_BINARY -c $RANDOM_DIRECTORY -p $RECIEVER_WALLET --usernet receive -m emoji -i $SEND_EMOJI | awk '{split($0,a,"Command"); print a[1]}')
        RECIEVER_EMOJI=$(echo $RECIEVER_EMOJI | sed 's/.*transaction://g')

        # Confirm the transaction on the sender
        $EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet finalize -m emoji -i $RECIEVER_EMOJI
    done

done
