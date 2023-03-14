#!/bin/bash

set -gx

export RUST_LOGLEVEL="debug"

cargo make --profile production run
