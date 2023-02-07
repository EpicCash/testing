#!/bin/bash

# Do all the steps for a complet test
# The server is initialized and ended in this same file
source ./variables.sh

mkdir -p out
mkdir -p $SERVER_PATH



# Start the server
mkdir -p $SERVER_PATH
$EPIC_SERVER_BINARY --usernet --onlyrandomx > /dev/null &
SERVER_PID=$!

./1-create-wallet-world.sh
./2-mine-wallets.sh

./mine-purge-wallet.sh 10

# Create the path for the wallet in old
random_string_332="version-3-3-2"
PATH_NAME_332=$random_string_332
DIR_332="./$WORLD_NAME/$PATH_NAME_332"
WALLET_PASSWORD_332=$(basename "$DIR_332")
COINS_PER_TRANSACTION=1


random_string="latest-version"
PATH_NAME=$random_string
DIR="./$WORLD_NAME/$PATH_NAME"
WALLET_PASSWORD=$(basename "$DIR")

echo "Transaction from 3-3-2 to new version"

# Generate a new emoji sender string
echo "Making a emoji transaction with $WALLET_PASSWORD_332 as sender and wallet $WALLET_PASSWORD as reciever"
SEND_EMOJI=$($EPIC_WALLET_332_BINARY -c $DIR_332 -p $WALLET_PASSWORD_332 --usernet send -m emoji $COINS_PER_TRANSACTION | awk '{split($0,a,"Command"); print a[1]}')

# Recieve the Epics on the target wallet
echo "Recieving a emoji transaction with $WALLET_PASSWORD_332 as sender and wallet $WALLET_PASSWORD as reciever"

RECIEVER_EMOJI=$($EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet receive -m emoji -i $SEND_EMOJI | awk '{split($0,a,"Command"); print a[1]}')
RECIEVER_EMOJI=$(echo $RECIEVER_EMOJI | sed 's/.*transaction://g')
# Confirm the transaction on the sender
$EPIC_WALLET_332_BINARY -c $DIR_332 -p $WALLET_PASSWORD_332 --usernet finalize -m emoji -i $RECIEVER_EMOJI

echo "Transaction from new version to 3-3-2 (note we expect an error)"

# Generate a new emoji sender string
echo "Making a emoji transaction with $WALLET_PASSWORD as sender and wallet $WALLET_PASSWORD_332 as reciever"
SEND_EMOJI=$($EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet send -m emoji $COINS_PER_TRANSACTION | awk '{split($0,a,"Command"); print a[1]}')

# Recieve the Epics on the target wallet
echo "Recieving a emoji transaction with $WALLET_PASSWORD as sender and wallet $WALLET_PASSWORD_332 as reciever"
RECIEVER_EMOJI=$($EPIC_WALLET_332_BINARY -c $DIR_332 -p $WALLET_PASSWORD_332 --usernet receive -m emoji -i $SEND_EMOJI | awk '{split($0,a,"Command"); print a[1]}')
RECIEVER_EMOJI=$(echo $RECIEVER_EMOJI | sed 's/.*transaction://g')
# Confirm the transaction on the sender
$EPIC_WALLET_BINARY -c $DIR -p $WALLET_PASSWORD --usernet finalize -m emoji -i $RECIEVER_EMOJI


echo "Cleanning the blocks"
./mine-purge-wallet.sh 10

# Clean files and kill all epic processes
# Kill all the processes
echo "Killing the server"
kill $SERVER_PID
