#!/bin/bash

# Build the rust code
cargo +nightly rustc -Z build-std=core,compiler_builtins,alloc --target aarch64-unknown-uefi --release --

dd if=/dev/zero count=524288 of=disk.img
mkfs.fat -F32 disk.img
mkdir disk
hdiutil attach disk.img -mountpoint disk

cd disk
mkdir -p EFI/BOOT/
cp ../target/aarch64-unknown-uefi/release/os.efi ./EFI/BOOT/BOOTAA64.EFI
cd ..

hdiutil eject $(realpath disk)

dd if=/dev/zero of=edk2-aarch64-vars.fd bs=1M count=64
