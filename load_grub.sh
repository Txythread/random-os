git clone https://git.savannah.gnu.org/git/grub.git
wait $!

cd grub
./bootstrap
./configure --target=i386-pc --with-platform=pc --program-prefix=""
export TARGET_CC=x86_64-elf-gcc
export TARGET_OBJCOPY=x86_64-elf-objcopy
export TARGET_STRIP=x86_64-elf-strip

make -j$(sysctl -n hw.logicalcpu)

wait !
grub-mkrescue
