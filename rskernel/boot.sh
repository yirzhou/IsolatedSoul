#!/bin/sh
qemu-system-x86_64 -drive format=raw,file=target/x86_64-rskernel/debug/bootimage-rskernel.bin
