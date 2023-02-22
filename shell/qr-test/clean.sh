#!/bin/bash

# File to clean all the processes and files created during a execution of the test

source ./variables.sh

# Kill the epic processes
ps aux | grep epic | awk '{print $2}' | xargs kill -9

# Clean all the files created
rm -r $WORLD_NAME server-data epic-miner.log $PURGE_WALLET $TRANSACTION_FILE $TRANSACTION_FILE.response out/

# Kill the epic processes
ps aux | grep epic | awk '{print $2}' | xargs kill -9
