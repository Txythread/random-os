#!/bin/bash

# Set the name of the target here
TARGET=aarch64
#TARGET=x86_64
CLANG_TARGET=aarch64-none-elf
#CLANG_TARGET=x86_64-none-elf
BUILD_DIR=target/$TARGET/release

EDK2_DIR=efi

# Build the trampoline
mkdir -p target/$TARGET
clang --target=$CLANG_TARGET -c targets/$TARGET/trampoline.s -o target/$TARGET/boot-trampoline.o


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
    -o kernel.elf



# Extrapolate the raw binary from the elf
llvm-objcopy -O binary kernel.elf \
             ./os.bin
