#!/usr/bin/env bash

# Exit on errors
set -e

# Change directory into script directory, then to root of project
cd "$(dirname "$0")/.." || exit
WORK_DIR=$(pwd)

cargo run --release