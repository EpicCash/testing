#!/bin/bash

# Mine the purge wallet
# A purge wallet is a wallet used to mine blocks to clean the wallet world transactions and coinbase
# The amount of blocks to be mined is recieved as the first argument

# Prepare the environment variables
source ./variables.sh

echo "------------------------Running purge wallet with miner------------------------"

# Calculate the starting chain height
CHAIN_HEIGHT=$($EPIC_SERVER_BINARY client status | grep -o "Chain height:.*" | awk '{print $NF}')
STOP_HEIGH=$(echo $CHAIN_HEIGHT+$1 | bc)

# Make the wallet list and store it's PID
WALLET_PASSWORD="$(basename "$dir")"
$EPIC_WALLET_BINARY -c "./$PURGE_WALLET" -p "$PURGE_WALLET" --usernet listen > /dev/null &
WALLET_PID=$!
echo "Wallet $PURGE_WALLET listening to miner"

# Start the miner
echo "Starting the epic-miner"
$EPIC_MINER_BINARY > /dev/null &
MINER_PID=$!

# Wait for it to mine x blocks them kill the wallet-process
while [ $CHAIN_HEIGHT -lt $STOP_HEIGH ]
do
    sleep 0.1
    CHAIN_HEIGHT=$($EPIC_SERVER_BINARY client status | grep -o "Chain height:.*" | awk '{print $NF}')
done

# Kill all the processes
kill $WALLET_PID
kill $MINER_PID
