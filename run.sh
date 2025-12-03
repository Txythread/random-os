#!/bin/bash


qemu-system-aarch64 \
    -m 2G \
    -cpu cortex-a53 \
    -M virt,secure=off \
    -device loader,file=target/aarch64/os.bin,addr=0x00000 -D qemu.log\
    -nographic \
    -d in_asm,cpu \
    -semihosting \
