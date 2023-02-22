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

./1-create-wallet-world.sh 10
./2-mine-wallets.sh

./mine-purge-wallet.sh 10

./info_world.sh > ./out/info-lmdb-exp-1.txt

echo "Batch 10 transactions with 0.01 coins"

./3-transaction-qr.sh 0.01 10
./mine-purge-wallet.sh 5

./info_world.sh > ./out/info-lmdb-exp-2.txt

echo "Batch 20 transactions with 0.02 coins"

./3-transaction-qr.sh 0.02 20
./mine-purge-wallet.sh 10

./info_world.sh > ./out/info-lmdb-exp-3.txt

echo "Batch 30 transactions with 0.05 coins"

./3-transaction-qr.sh 0.05 30
./mine-purge-wallet.sh 15

./info_world.sh > ./out/info-lmdb-exp-4.txt

echo "Batch 50 transactions with 0.001 coins"

./3-transaction-qr.sh 0.001 50
./mine-purge-wallet.sh 25

./info_world.sh > ./out/info-lmdb-exp-5.txt

echo "Cleanning the blocks"
./mine-purge-wallet.sh 10

./info_world.sh > ./out/info-lmdb-exp-6.txt

# Clean files and kill all epic processes
# Kill all the processes
echo "Killing the server"
kill $SERVER_PID
