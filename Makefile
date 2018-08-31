export BUILD_DIR := target/kernel
export MOD_NAME := ramdev

PWD := $(shell pwd)

export BINDING_DIR := src/os

export KERNEL_SOURCE := /lib/modules/$(shell uname -r)/build

default: modules

all modules:
	@mkdir -p ${BUILD_DIR}/src
	cp "Makefile.stage2" "$(BUILD_DIR)/Makefile"
	$(MAKE) -C $(KERNEL_SOURCE) M=$(PWD)/$(BUILD_DIR) modules
	cp $(BUILD_DIR)/$(MOD_NAME).ko $(MOD_NAME).ko

clean:
	$(MAKE) -C $(KERNEL_SOURCE) M=$(PWD) clean
	cargo clean
	rm src/os/kernel.rs