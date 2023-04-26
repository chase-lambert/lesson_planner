#!/bin/bash
gzip -kfv static/main.css
cargo build --release 
