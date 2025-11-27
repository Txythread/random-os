qemu-system-aarch64 \
    -m 2G \
    -cpu cortex-a53 \
    -M virt,secure=off \
    -device loader,file=os.bin,addr=0x00000 \
    -nographic \
    -d in_asm,cpu \
    -semihosting
