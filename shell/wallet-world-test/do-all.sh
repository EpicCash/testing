#!/bin/bash

# Do all the steps for a complet test
# The server is initialized and ended in this same file
source ./variables.sh

mkdir -p out
mkdir -p $SERVER_PATH

for i in {1..3}; do
    # Start the server
    mkdir -p $SERVER_PATH
    $EPIC_SERVER_BINARY --usernet --onlyrandomx > /dev/null &

    ./1-create-wallet-world.sh 10
    ./2-mine-wallets.sh

    ./mine-purge-wallet.sh 10

    ./info_world.sh > ./out/info-lmdb-exp-$i-1.txt

    echo "Exp $i Batch 10 transactions with 15 coins"

    ./3-transaction-emoji.sh 15 10
    ./mine-purge-wallet.sh 5

    ./info_world.sh > ./out/info-lmdb-exp-$i-2.txt

    echo "Exp $i Batch 20 transactions with 5 coins"

    ./3-transaction-emoji.sh 5 20
    ./mine-purge-wallet.sh 10

    ./info_world.sh > ./out/info-lmdb-exp-$i-3.txt

    echo "Exp $i Batch 30 transactions with 1 coins"

    ./3-transaction-emoji.sh 1 30
    ./mine-purge-wallet.sh 15

    ./info_world.sh > ./out/info-lmdb-exp-$i-4.txt

    echo "Exp $i Batch 50 transactions with 0.001 coins"

    ./3-transaction-emoji.sh 0.001 50
    ./mine-purge-wallet.sh 25

    ./info_world.sh > ./out/info-lmdb-exp-$i-5.txt

    echo "Cleanning the blocks"
    ./mine-purge-wallet.sh 10

    ./info_world.sh > ./out/info-lmdb-exp-$i-6.txt

    # Clean files and kill all epic processes
    ./clean.sh
done
