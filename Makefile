DEPS = $(wildcard src/*.rs) $(wildcard src/sbi/*.rs) $(wildcard src/*.S) $(wildcard src/linker.ld)
RUST_TARGET = target/riscv64gc-unknown-none-elf/debug/rust-os

all: $(DEPS)
	cargo build
	cp $(RUST_TARGET) ./vmlinux
	riscv64-unknown-elf-objdump -d vmlinux > kernel.asm
	nm vmlinux > System.map

run: all
	qemu-system-riscv64 -nographic -machine virt -kernel vmlinux -bios default

clean:
	rm -f kernel.asm System.map vmlinux