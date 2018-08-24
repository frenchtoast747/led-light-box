#!/usr/bin/env sh

if [ ! -f ~/.led-build.sh ]; then
    echo "Create a file at ~/.led-build.sh the following variables (with the correct paths):"
    echo "RPI_WS281X_SYSROOT=C:/SysGCC/raspberry/arm-linux-gnueabihf/sysroot"
    echo "CC_arm_unknown_linux_gnueabihf=C:/SysGCC/raspberry/bin/arm-linux-gnueabihf-gcc.exe"
    echo "AR_arm_unknown_linux_gnueabihf=C:/SysGCC/raspberry/bin/arm-linux-gnueabihf-ar.exe"
    exit 1
fi

. ~/.led-build.sh
cargo build --target arm-unknown-linux-gnueabihf -p neopixels $@