#!/bin/bash

# Create the path for the server
# The server will use the toml on the same path as this executable
# That toml is configured to store data on the SERVER_PATH variable

EPIC_SERVER_BINARY=/home/jhelison/Documents/epic/testing/binaries/epic
SERVER_PATH=./server-data

mkdir -p $SERVER_PATH

$EPIC_SERVER_BINARY --onlyrandomx
