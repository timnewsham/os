#!/bin/sh
#
# Invoke rust-gdb on our target binary.
# Local .gdbinit will cause it to connect to qemu on startup.
# After completion, kill off the qemu instance.
#

TARG=./target/aarch64-unknown-none/debug/os

../../prebuilt/third_party/rust/mac-x64/bin/rust-gdb "$@" $TARG

echo "killing emu"
killall qemu-system-aarch64
