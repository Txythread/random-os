#!/bin/bash


# Set the name of the target here
TARGET=aarch64
CLANG_TARGET=aarch64-none-elf
BUILD_DIR=target/$TARGET/release

# Create a temporary linker script
cp targets/$TARGET/layout.ld layout.ld

# Build the trampoline
mkdir -p target/$TARGET
clang --target=$CLANG_TARGET -c targets/$TARGET/trampoline.s -o target/$TARGET/boot-trampoline.o


# Optionally assemble the reset vector for the target
ls target
ls target/$TARGET

# Build the rust code
#cargo +nightly rustc -Z build-std=core,compiler_builtins,alloc --target targets/$TARGET/$TARGET.json --release -- --emit=obj
cargo +nightly rustc -Z build-std=core,compiler_builtins,alloc --target aarch64-unknown-uefi --release --

dd if=/dev/zero count=2097152 of=disk.img
mkfs.fat -F32 disk.img
mkdir disk
hdiutil attach disk.img -mountpoint disk

cd disk
mkdir -p EFI/BOOT/
cp ../target/aarch64-unknown-uefi/release/os.efi ./EFI/BOOT/BOOTAA64.EFI
cd ..

hdiutil eject $(realpath disk)

dd if=/dev/zero of=edk2-aarch64-vars.fd bs=1M count=64


# layout.ld only served as a temporary script as it changes
# with arch, so remove it
rm layout.ld
