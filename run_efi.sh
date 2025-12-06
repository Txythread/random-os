#!/bin/bash

qemu-system-aarch64 \
  -M virt,secure=off \
  -cpu cortex-a53 \
  -m 2G \
  -nographic\
  -drive file=disk.img,format=raw,if=virtio \
  -D qemu.log \
  -semihosting \
  -drive if=pflash,format=raw,readonly=on,file=$(pwd)/edk2-aarch64-code.fd \
  -drive if=pflash,format=raw,file=$(pwd)/edk2-arm-vars.fd \
