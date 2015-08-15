SELF_DIR := $(dir $(lastword $(MAKEFILE_LIST)))

RUSTC = rustc
RUSTFLAGS = -C opt-level=2 -Z no-landing-pads
RUSTFLAGS += --target thumbv7em-none-eabi -g --emit obj -L libcore-thumbv7m

RUSTSRC = main.rs
RUSTOBJS = $(RUSTSRC:.rs=.o)

all: spark-metal.bin

spark-metal.bin: spark-metal.elf
	arm-none-eabi-objcopy -S -O binary spark-metal.elf spark-metal.bin

spark-metal.elf: cortex-m3.ld startup.o $(RUSTOBJS)
	arm-none-eabi-ld -T cortex-m3.ld -I$(SELF_DIR)include startup.o $(RUSTOBJS) -o spark-metal.elf

startup.o: startup.S
	arm-none-eabi-as -mcpu=cortex-m3 -g startup.S -o startup.o

%.o: %.rs $(wildcard *.rs)
	$(RUSTC) $(RUSTFLAGS) -o ${@} ${<}

libcore: clean
	mkdir libcore-thumbv7m
	$(RUSTC) -C opt-level=2 -Z no-landing-pads --target thumbv7em-none-eabi -g ../rust/src/libcore/lib.rs --out-dir=libcore-thumbv7m

clean:
	rm -rf *.o spark-metal.elf spark-metal.bin libcore-thumbv7m

load: spark-metal.bin
	st-flash erase && st-flash write spark-metal.bin 0x08000000
