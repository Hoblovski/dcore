# Tool definition
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Path definition
TARGET := riscv64gc-unknown-none-elf
MODE := debug
KERNEL_ELF := target/$(TARGET)/$(MODE)/dcore-kernel
KERNEL_BIN := $(KERNEL_ELF).bin
BOOTLOADER := src/rustsbi-qemu.bin
KERNEL_ENTRY_PA := 0x80200000

# commands
.PHONY:
	all build run fmt fix clean gdb1 gdb2 objdump .force

all: build

build: $(KERNEL_BIN)

run: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

fmt:
	@cargo fmt

fix:
	@cargo fix --bin dcore-kernel

clean:
	@cargo clean

gdb1:
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-s -S

gdb2:
	@riscv64-unknown-elf-gdb -e .gdbinit

objdump:
	@rust-objdump -D $(KERNEL_ELF) > k.S
	@echo "Target objdump'ed to k.S"


.force:

# files
$(KERNEL_ELF): .force
	@cargo build

$(KERNEL_BIN): $(KERNEL_ELF) .force
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@
