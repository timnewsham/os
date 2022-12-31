# Goofing with OS stuff

This is a small stand alone binary written in rust that runs
in qemu in a virtual rasbpi3b.
Currently it just does a little initialization and then prints
over the uart and then exits.
It runs in qemu directly from the built ELF binary, not using
any special linker script or bootimage builder, so the execution
environment probably doesnt match anything that would work on
real hardware.

Run with `cargo run` or `cargo run -r`, with `qemu-system-aarch64` in
your path.  Scripts `dump`, `qemu`, and `gdb` assume tools are in
your path, and will dump the target disassembly, run the target in
the emulator, and attach to the target with `rust-gdb` respectively.

Qemu execution uses the unsafe `-semihosting` feature to support
exiting the vm from inside the host. 
Semihosting in qemu allows guests to access your host.
Use at your own risk.
