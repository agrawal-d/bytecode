#!/bin/bash
clear
cargo fmt
cargo clippy --fix --allow-dirty
