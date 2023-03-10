#!/bin/bash

set -gx

export RUST_LOGLEVEL="debug"

sqlx database setup
cargo run --release
