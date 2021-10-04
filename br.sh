#!/bin/bash
cargo build --target armv7-unknown-linux-gnueabihf
rsync -avP . skrcka@10.0.0.7:./controller-rpi/
ssh -t skrcka@10.0.0.7 'cd /home/skrcka/controller-rpi; cargo run'