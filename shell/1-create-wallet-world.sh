#!/bin/bash

WORLD_NAME=world
EPIC_WALLET_BINARY=/home/jhelison/Documents/epic/testing/binaries/epic-wallet

echo "Creating a wallet world with $1 wallets"

# Create a path for the wallet world
mkdir -p $WORLD_NAME

# Iterate until we create our world
for i in $(seq 1 $1); do
    # Create the path for the wallet
    random_string=$(openssl rand -base64 32 | tr -dc 'a-zA-Z0-9')
    PATH_NAME=$i-$random_string

    echo "$EPIC_WALLET_BINARY -c "./$WORLD_NAME/$PATH_NAME" -p "$PATH_NAME" --usernet init"

    # Initialize the wallets
    $EPIC_WALLET_BINARY -c "./$WORLD_NAME/$PATH_NAME" -p "$PATH_NAME" --usernet init
done
