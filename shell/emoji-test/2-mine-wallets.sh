#!/bin/bash

# Mine 50% of the wallet world wallets

source ./variables.sh

echo "------------------------Running the wallet miner script------------------------"

# Calculate the starting chain height
CHAIN_HEIGHT=$($EPIC_SERVER_BINARY client status | grep -o "Chain height:.*" | awk '{print $NF}')
STOP_HEIGH=$(echo $CHAIN_HEIGHT+$BLOCKS_TO_MINE | bc)

# Start the miner
echo "Starting the epic-miner"
$EPIC_MINER_BINARY > /dev/null &
MINER_PID=$!

# Select the wallets to be mined
# Only takes 50% of the wallets available
WALLET_DIRECTORY=$(find "./world" -mindepth 1 -maxdepth 1 -type d)
AMOUNT_WALLETS=$(echo "$WALLET_DIRECTORY" | wc -l)
AMOUNT_WALLETS=`echo $AMOUNT_WALLETS*0.5 | bc | awk '{print int($1)}'`
RANDOM_WALLETS=$(echo "$WALLET_DIRECTORY" | shuf -n $AMOUNT_WALLETS)

echo "Mining on $AMOUNT_WALLETS wallets"

for DIR in $RANDOM_WALLETS; do
    WALLET_PASSWORD="$(basename "$DIR")"
    echo "Wallet $WALLET_PASSWORD listening to miner"

    # Make the wallet list and store it's PID
    $EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet listen > /dev/null &
    WALLET_PID=$!

    # Mine exactly the same amount of blocks for all wallets
    while [ $CHAIN_HEIGHT -lt $STOP_HEIGH ]
    do
        sleep 0.1
        CHAIN_HEIGHT=$($EPIC_SERVER_BINARY client status | grep -o "Chain height:.*" | awk '{print $NF}')
    done

    # Kill the wallet and restart the chain height counter
    kill $WALLET_PID
    CHAIN_HEIGHT=$($EPIC_SERVER_BINARY client status | grep -o "Chain height:.*" | awk '{print $NF}')
    STOP_HEIGH=$(echo $CHAIN_HEIGHT+$BLOCKS_TO_MINE | bc)
done

# Kill all the processes
echo "Killing the miner"
kill $MINER_PID
