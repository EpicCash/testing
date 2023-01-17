#!/bin/bash

# Create a new wallet world
# The number of wallets needs to be passed as arguments as $1

source ./variables.sh

echo "------------------------Running the create wallet world------------------------"

echo "Creating a wallet world with $1 wallets"

# Create a path for the wallet world
mkdir -p $WORLD_NAME

# Create a purge wallet for mining
$EPIC_WALLET_BINARY -c "./$PURGE_WALLET" -p "$PURGE_WALLET" --usernet init > /dev/null
echo "Purge wallet created"

# Iterate until we create our world
echo "Creating the wallet world"
for i in $(seq 1 $1); do
    # Create the path for the wallet
    random_string=$(openssl rand -base64 32 | tr -dc 'a-zA-Z0-9')
    PATH_NAME=$i-$random_string

    # Initialize the wallets
    $EPIC_WALLET_BINARY -c "./$WORLD_NAME/$PATH_NAME" -p "$PATH_NAME" --usernet init > /dev/null
    echo "Wallet $PATH_NAME created"
done
