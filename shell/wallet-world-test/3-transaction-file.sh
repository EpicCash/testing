#!/bin/bash

# Do a file transactions between the wallets on the wallet world
# Has two arguments
#   The amount of coins to be transactioned as $1
#   The number of transactions to be done as $2

source ./variables.sh

# Save the arguments as variables for better context
COINS_PER_TRANSACTION=$1
NUMBER_TRANSACTIONS=$2

echo "------------------------Running file transaction------------------------"

# Try to do transactions until we reach the amount we expect
TXS_COUNT=1
while true; do
    # Break if we exceed the count
    if [ "$TXS_COUNT" -gt $NUMBER_TRANSACTIONS ]; then
        break
    fi

    # Choose a random wallet
    WALLET_DIRECTORY=$(find "./world" -mindepth 1 -maxdepth 1 -type d)
    DIR=$(echo "$WALLET_DIRECTORY" | shuf -n 1)
    WALLET_PASSWORD=$(basename "$DIR")

    # Check if the wallet has funds
    INFO=$($EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet info)
    SPENDABLE_INFO=$(echo "$INFO" | grep -o "Currently Spendable.*" | awk '{print $NF}')
    HAS_FUNDS=$(echo "$SPENDABLE_INFO == 0" | bc -l)
    if [ "$HAS_FUNDS" -eq 1 ]; then
        echo "Skipping wallet $WALLET_PASSWORD due to no funds"
        continue
    fi

    # Get a new random wallet to recieve the transaction
    WALLET_DIRECTORY=$(find "./world" -mindepth 1 -maxdepth 1 -type d)
    RANDOM_DIRECTORY=$(echo "$WALLET_DIRECTORY" | shuf -n 1)
    RECIEVER_WALLET=$(basename "$RANDOM_DIRECTORY")

    # Generate a new file sender string
    echo "Transaction with $WALLET_PASSWORD and $RECIEVER_WALLET"
    $EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet send -d $TRANSACTION_FILE -m file $COINS_PER_TRANSACTION > /dev/null

    # Recieve the Epics on the target wallet
    $EPIC_WALLET_BINARY -c $RANDOM_DIRECTORY -p $RECIEVER_WALLET --usernet receive -i $TRANSACTION_FILE > /dev/null

    # Confirm the transaction on the sender
    $EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet finalize -i $TRANSACTION_FILE.response > /dev/null

    # This deletes the transaction files
    # At the same time, serves as a check if a transaction was processed
    eval rm $TRANSACTION_FILE $TRANSACTION_FILE.response
    if [ $? -eq 0 ]; then
        echo "Transaction number $TXS_COUNT done"
        TXS_COUNT=$(echo $TXS_COUNT + 1 | bc)
    else
        echo "Error on doing the transaction"
    fi
done
