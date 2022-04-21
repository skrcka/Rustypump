#!/bin/bash
rsync -arvP ./target/aarch64-unknown-linux-gnu/debug/controller-rpi skrcka@10.0.28.171:.
#rsync -avP ./config.ini skrcka@10.0.28.171:.