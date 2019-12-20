#!/usr/bin/env sh

scp -i ~/.ssh/id_rsa ./target/arm-unknown-linux-gnueabihf/release/neopixels pi@192.168.1.27:led-light-box/neopixels