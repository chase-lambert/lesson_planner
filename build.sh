#!/bin/bash
npm install
npm run release
gzip -kfv static/main.css
cargo build --release 
