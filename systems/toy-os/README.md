# Toy OS

Operating system built by using the [Philip Oppermann](https://os.phil-opp.com/) tutorial, updated and modified.

It contains a bootloader which initializes memory, handles hardware interrupts and faults, handles dynamic memory allocation, and implements an asyncrnous task execution system.

## Setup

Install `qemu` and `rustup` then run `rustup install`.
Typing `cargo run` should download, compile, and run the OS in a `qemu` VM.
