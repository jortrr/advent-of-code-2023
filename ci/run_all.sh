#!/usr/bin/env bash

# Exit on errors
set -e

# Change directory into script directory, then to root of project
cd "$(dirname "$0")/.." || exit
WORK_DIR=$(pwd)

run_all() {
  for i in {1..25}
  do
    printf "Run day%02d\n" "$i"
    cargo run --release --bin "$(printf "day%02d\n" "$i")"
  done
}

cargo build --release && time run_all
