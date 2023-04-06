TARGET ?= riscv64imac-unknown-none-elf
RISCV_BASE ?= riscv64-elf

#GDB ?= $(RISCV_BASE)-gdb
GDB = gdb-multiarch
OBJDUMP ?= $(RISCV_BASE)-objdump
GDB_PORT ?= 3333

BINARY_NAME = strail
RELEASE_BIN = target/$(TARGET)/release/$(BINARY_NAME)
DEBUG_DIR = target/$(TARGET)/debug
DEBUG_BIN = $(DEBUG_DIR)/$(BINARY_NAME)

CARGO_FLAGS ?=
QEMU_DEFAULT_BINARY ?= $(DEBUG_BIN)

IMAGE_GEN ?= scripts/image_gen
IMAGE_VHDL_PATH ?= $(DEBUG_DIR)/$(BINARY_NAME).vhd
NEORV32_PATH ?= $(HOME)/workspace/neorv32

all: run
r: all
b: build
d: build-debug


clean:
	cargo clean

build:
	cargo build --target $(TARGET) $(CARGO_FLAGS) --release

build-debug:
	cargo build --target $(TARGET) $(CARGO_FLAGS)

qemu: | build-debug
	echo "CTRL-a-x to quit Qemu when running -nographic"
	qemu-system-riscv64 \
	-machine virt \
	-m 128M \
	-kernel $(QEMU_DEFAULT_BINARY) \
	-bios none \
	-nographic \
        -gdb tcp::$(GDB_PORT) \
        -S 

.PHONY: gdb
gdb:
	$(GDB) \
	-q \
	-ex 'file $(DEBUG_BIN)' \
	-ex 'target remote localhost:$(GDB_PORT)' \
	-ex "b main" \
	-ex "b switch_to_user"
	-ex "b trap_handler"

run: run-debug

run-debug:
	cargo run --target $(TARGET) $(CARGO_FLAGS)

run-release:
	cargo run --target $(TARGET) $(CARGO_FLAGS) --release

test:
	cargo test --target $(TARGET) $(CARGO_FLAGS)

docs: clean
	cargo doc
	rm -rf docs/
	cp -rf target/$(TARGET)/doc ./docs
	echo '<meta http-equiv=refresh content=0;url=strail/index.html>' > docs/index.html

objdump:
	$(OBJDUMP) -D $(DEBUG_BIN) | less

install-linter:
	rustup component add rustfmt
	rustup component add clippy

lint: | install-linter
	cargo fmt -v
	cargo clippy --all-targets --all-features -- -D warnings -v

rust-install:
	rustup target add riscv64imac-unknown-none-elf
	$(MAKE) install-linter

neorv32-image:
	g++ -Wall -O -g $(IMAGE_GEN).c -o $(IMAGE_GEN)
	$(IMAGE_GEN) -app_img $(DEBUG_BIN) $(IMAGE_VHDL_PATH) $(DEBUG_BIN).bin

neorv32-sim: neorv32-image
	cp -v $(IMAGE_VHDL_PATH) $(NEORV32_PATH)/rtl/core/neorv32_application_image.vhd
	bash $(NEORV32_PATH)/sim/simple/ghdl.run.sh

debug-deps:
	cargo install cargo-binutils
	cargo install cargo-bloat

	rustup component add llvm-tools-preview
