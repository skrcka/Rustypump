#!/bin/bash
rsync -avP ./target/armv7-unknown-linux-gnueabihf/debug/controller-rpi skrcka@10.0.0.7:.
ssh -t skrcka@10.0.0.7 './controller-rpi'