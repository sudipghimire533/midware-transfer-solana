#!/bin/bash

CLIENT_DIR=$PWD

echo "Make sure you are running local test validator.."

# First lets setup some basics
solana config set --url localhost

# Move to temp dir and start validator
rm -rf /tmp/test-ledger
cd /tmp
solana-test-validator 2>&1 1> /dev/null &
cd $CLIENT_DIR

# Lets wait for few seconds for validator to heat up
curl 127.0.0.1:1024
while [ $? -ne 0 ]; do
    curl 127.0.0.1:1024 1>&2 2> /dev/null
done

# Let's fund our keypairs to exist
for pair in ./*.keypair; do
    solana airdrop 100 $pair
done
solana airdrop 100

# Now lets compile and deploy our program
cargo build-bpf &&\
    solana program deploy ../../target/deploy/midware_transfer.so

# Everything's good. Start the client
yarn install
echo "-----------------------------------"
echo "-----------------------------------"
echo "====== Note balance of Alice & Bob ======="
echo "------------------------------------"
node main.js
echo "------------------------------------"

# Kill the validtor
killall solana-test-validator