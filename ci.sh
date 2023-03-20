#!/bin/bash
#
# Run ci checks

set -xe

cargo deny check
cargo clippy
