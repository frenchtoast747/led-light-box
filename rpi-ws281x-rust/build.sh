#!/usr/bin/env sh

if [ ! -f ~/.led-build.sh ]; then
    echo "Create a file at ~/.led-build.sh exporting the CC and AR variables"
    echo "pointing to the GCC ARM toolchain."
    exit 1
fi

. ~/.led-build.sh
cargo build $@
#cargo doc $@
