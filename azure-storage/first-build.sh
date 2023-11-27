#!/bin/bash
sudo apt update && \
sudo apt install -y pkg-config libssl-dev && \
cargo build
