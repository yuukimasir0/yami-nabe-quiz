#!/bin/bash

mkdir -p output
> output/output_1.txt
echo "static/input.txt" | cargo run --release >> output/output_1.txt
cargo clean