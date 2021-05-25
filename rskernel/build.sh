#!/bin/sh

# For macOS only.
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
