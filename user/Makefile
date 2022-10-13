# Tool definition
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Path definition
TARGET := riscv64gc-unknown-none-elf
MODE := debug
TARGET_DIR := target/$(TARGET)/$(MODE)
APP_DIR := src/bin
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
BINS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%.bin, $(APPS))

# Configs
APP_BASE_ADDRESS := 0x80400000

# commands
.PHONY:
	all bin elf run fmt clean .force

all: bin

bin: $(BINS)

elf: $(ELFS)

run: $(ELFS)
	@qemu-riscv64 $(TARGET_DIR)/hello

fmt:
	@cargo fmt

clean:
	@cargo clean

.force:


# files
$(ELFS): $(TARGET_DIR)/%: .force
	@echo "/* Generated from src/linker.lds. Do not modify. */" > src/linker.ld
	@sed '1d; s/{{BASE_ADDRESS}}/$(APP_BASE_ADDRESS);/g' src/linker.lds >> src/linker.ld
	@cargo build --bin $*
	@rm src/linker.ld

$(BINS): %.bin: % .force
	@$(OBJCOPY) $* --strip-all -O binary $@
