#!/bin/bash

# Set the name of the target here
#TARGET=aarch64
TARGET=x86_64
CLANG_TARGET=x86_64-none-elf
BUILD_DIR=target/$TARGET/release

EDK2_DIR=efi

# Move the layout.ld file temporarily
cp targets/$TARGET/layout.ld .

# Build the trampoline
mkdir -p target/$TARGET
clang --target=$CLANG_TARGET -c targets/$TARGET/trampoline.s -o target/$TARGET/boot-trampoline.o


# Optionally assemble the reset vector for the target
clang --target=$CLANG_TARGET -c targets/$TARGET/reset-vector.s -o target/$TARGET/reset-vector.o

ls target
ls target/$TARGET

# Build the rust code
cargo +nightly rustc -Z build-std=core,compiler_builtins --target targets/$TARGET/$TARGET.json --release -- --emit=obj

# Create a copy for edk2
cp $BUILD_DIR/deps/os-*.o $EDK2_DIR

# Link whatever cargo produced with whatever the trampoline
# is into one singular elf.
ld.lld -T layout.ld \
    target/$TARGET/boot-trampoline.o \
    $BUILD_DIR/deps/os-*.o \
    -o target/$TARGET/kernel.elf


# Create an image
truncate -s 64M disk.img
mkfs.fat -F 32 disk.img
DISK=$(hdiutil attach -imagekey diskimage-class=CRawDiskImage -nomount disk.img)
mkdir -p mnt
sudo mount -t msdos $DISK /Users/michaelrudolf/osdev/os/mnt


# Remove temporary files
rm layout.ld
