#!/bin/bash

# File to clean all the processes and files created during a execution of the test

source ./variables.sh

# Kill the epic processes
ps aux | grep epic | awk '{print $2}' | xargs kill -9
