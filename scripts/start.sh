#!/usr/bin/env bash
set -e

# Trap Ctrl+C (SIGINT) to kill all child processes
trap 'pkill -f revolt-' SIGINT

# Run the compiled binaries directly (no cargo!)
./revolt-delta &
./revolt-bonfire &
./revolt-autumn &
./revolt-january

# Wait so container doesn't immediately exit
wait
