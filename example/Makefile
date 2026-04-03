SRCS := $(wildcard */*.c)
KOOPAS := $(SRCS:.c=.koopa)
RISCVS := $(SRCS:.c=.S)

TARGET_DIR := $(shell cargo metadata --format-version 1 --no-deps | jq -r '.target_directory')
BIN_NAME := $(shell cargo metadata --format-version 1 --no-deps | jq -r '.packages[].targets[] \
			| select( .kind | map(. == "bin") | any ) | .name')
BINARY_PATH := $(TARGET_DIR)/release/$(BIN_NAME)

build: $(BINARY_PATH) $(KOOPAS) $(RISCVS)

$(BINARY_PATH):
	cargo build --release

%.koopa: %.c
	$(BINARY_PATH) -koopa $< -o $@

%.S: %.c
	$(BINARY_PATH) -riscv $< -o $@

.PHONY: clean
clean:
	rm -f $(KOOPAS) $(RISCVS)
