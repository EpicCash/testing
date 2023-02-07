#!/bin/bash

# Create the path for the server
# The server will use the toml on the same path as this executable
# That toml is configured to store data on the SERVER_PATH variable

source ./variables.sh

mkdir -p $SERVER_PATH

$EPIC_SERVER_BINARY --usernet --onlyrandomx
