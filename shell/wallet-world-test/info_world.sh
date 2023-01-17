#!/bin/bash

# Gather information about our wallet world and echo it in the same style as the wallet info
# It also prints information about the purge wallet

source ./variables.sh

CONFIRMED=0.0
IMMATURE_COINBASE=0.0
AWAITING_CONFIMATION=0.0
AWAITING_FINALIZATION=0.0
LOCKED=0.0
SPENDABLE=0.0

for dir in ./world/*/; do
    WALLET_PASSWORD="$(basename "$dir")"

    INFO=$($EPIC_WALLET_BINARY -c $dir -p $WALLET_PASSWORD --usernet info)

    CONFIRMED_INFO=$(echo "$INFO" | grep -o "Confirmed Total.*" | awk '{print $NF}')
    CONFIRMED=`echo $CONFIRMED+$CONFIRMED_INFO | bc`

    IMMATURE_COINBASE_INFO=$(echo "$INFO" | grep -o "Immature Coinbase (< 3).*" | awk '{print $NF}')
    IMMATURE_COINBASE_INFO=${IMMATURE_COINBASE_INFO:-"0"} # Check if the string is empty
    IMMATURE_COINBASE=`echo $IMMATURE_COINBASE+$IMMATURE_COINBASE_INFO | bc`

    AWAITING_CONFIMATION_INFO=$(echo "$INFO" | grep -o "Awaiting Confirmation (< 10).*" | awk '{print $NF}')
    AWAITING_CONFIMATION=`echo $AWAITING_CONFIMATION+$AWAITING_CONFIMATION_INFO | bc`

    AWAITING_FINALIZATION_INFO=$(echo "$INFO" | grep -o "Awaiting Finalization.*" | awk '{print $NF}')
    AWAITING_FINALIZATION=`echo $AWAITING_FINALIZATION+$AWAITING_FINALIZATION_INFO | bc`

    LOCKED_INFO=$(echo "$INFO" | grep -o "Locked by previous transaction.*" | awk '{print $NF}')
    LOCKED=`echo $LOCKED+$LOCKED_INFO | bc`

    SPENDABLE_INFO=$(echo "$INFO" | grep -o "Currently Spendable.*" | awk '{print $NF}')
    SPENDABLE=`echo $SPENDABLE+$SPENDABLE_INFO | bc`
done

echo "Confirmed Total                  | $CONFIRMED"
echo "Immature Coinbase (< 3)          | $IMMATURE_COINBASE"
echo "Awaiting Confirmation (< 10)     | $AWAITING_CONFIMATION"
echo "Awaiting Finalization            | $AWAITING_FINALIZATION"
echo "Locked by previous transaction   | $LOCKED"
echo "-------------------------------- | -------------"
echo "Currently Spendable              | $SPENDABLE"
echo ""
echo ""
echo "Purge wallet info:"


$EPIC_WALLET_BINARY -c "./$PURGE_WALLET" -p "$PURGE_WALLET" --usernet info
