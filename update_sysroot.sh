#!/bin/bash

# Get the sysroot using rustc
SYSROOT=$(rustc --print sysroot)

# Create JSON string
JSON='{
    "sysroot_src": "",
    "crates": [
        {
            "root_module": "main.rs",
            "edition": "2021",
            "deps": []
        }
    ]
}'

# Update sysroot_src
JSON=$(echo $JSON | jq --arg SYSROOT "$SYSROOT" '.sysroot_src = $SYSROOT + "/lib/rustlib/src/rust/library"')

# Save JSON to file
echo $JSON > rust-project.json
