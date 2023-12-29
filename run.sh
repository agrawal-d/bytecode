#!/bin/bash
clear
export RUST_BACKTRACE=1
cargo run -q --features tracing
exit_code=$?
echo -e "Exited with code $exit_code"