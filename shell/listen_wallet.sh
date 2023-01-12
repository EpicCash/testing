#!/bin/bash

EPIC_WALLET_BINARY=$1
dir=$2
WALLET_PASSWORD=$3

$EPIC_WALLET_BINARY -c $dir -p $WALLET_PASSWORD --usernet listen & sleep 30; kill $!

