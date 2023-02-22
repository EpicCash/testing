#!/bin/bash

# Create a new wallet world
# The number of wallets needs to be passed as arguments as $1

source ./variables.sh

echo "------------------------Running the create wallet world------------------------"

echo "Creating a world with 2 wallets"

# Create a path for the wallet world
mkdir -p $WORLD_NAME

# Create a purge wallet for mining
$EPIC_WALLET_BINARY -c "./$PURGE_WALLET" -p "$PURGE_WALLET" --usernet init > /dev/null
echo "Purge wallet created"

# Iterate until we create our world
echo "Creating the wallet world"

# Create the path for the wallet in old
random_string="version-3-3-2"
PATH_NAME=$random_string

# Initialize the wallets
$EPIC_WALLET_332_BINARY -c "./$WORLD_NAME/$PATH_NAME" -p "$PATH_NAME" --usernet init > /dev/null
echo "Wallet $PATH_NAME created"

# Create the path for the wallet in old
random_string="latest-version"
PATH_NAME=$random_string

# Initialize the wallets
$EPIC_WALLET_BINARY -c "./$WORLD_NAME/$PATH_NAME" -p "$PATH_NAME" --usernet init > /dev/null
echo "Wallet $PATH_NAME created"

