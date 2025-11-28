qemu-system-x86_64 \
    -machine q35,accel=tcg \
    -cpu base \
    -m 2G \
    -nodefaults \
    -nographic \
    -monitor none \
    -device loader,file=os.bin,addr=0x00000000 \
    -device loader,addr=0xFFFFFFF0,data=0xEA00000000,data-len=8 \
    -d in_asm,cpu

