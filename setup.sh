#!/bin/bash

# Define the global command directory
GLOBAL_CMD_DIR="/usr/local/bin"

# Check if the askshell file exists in the current directory
if [ ! -f "./target/debug/askshell" ]; then
	echo "askshell file not found in the current directory."
	exit 1
fi

# Move askshell to the global command directory
echo "Moving askshell to $GLOBAL_CMD_DIR..."
sudo mv ./target/debug/askshell "$GLOBAL_CMD_DIR/askshell"

# Check if the move was successful
if [ $? -ne 0 ]; then
	echo "Failed to move askshell to $GLOBAL_CMD_DIR. Please check your permissions."
	exit 1
fi

echo "askshell has been moved to $GLOBAL_CMD_DIR."
