#!/bin/sh
#
# Invoke rust-gdb on our target binary.
# Local .gdbinit will cause it to connect to qemu on startup.
# After completion, kill off the qemu instance.
#

BUILD=${BUILD:-debug}
TARG=./target/aarch64-unknown-none/$BUILD/os

rust-gdb "$@" $TARG

echo "killing emu"
killall qemu-system-aarch64
