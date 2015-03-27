#!/bin/bash

cargo build --verbose
cd src/approvals_tests
cargo test --verbose
