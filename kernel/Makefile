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
	all build run fmt clean

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

clean:
	@cargo clean

# files
$(KERNEL_ELF):
	@cargo build

$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@
