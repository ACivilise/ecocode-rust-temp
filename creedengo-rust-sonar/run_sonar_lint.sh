#!/usr/bin/env bash

cargo clippy --message-format=json &> ./target/clippy-report.json
