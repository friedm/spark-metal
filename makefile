SELF_DIR := $(dir $(lastword $(MAKEFILE_LIST)))

RUSTC = rustc
RUSTFLAGS = -C opt-level=2 -Z no-landing-pads
RUSTFLAGS += --target thumbv7em-none-eabi -g --emit obj -L libcore-thumbv7m

RUSTSRC = main.rs
RUSTOBJS = $(RUSTSRC:.rs=.o)

all: spark-metal.bin

spark-metal.bin: spark-metal.elf
	arm-none-eabi-objcopy -S -O binary spark-metal.elf spark-metal.bin

spark-metal.elf: c_main.o gpio.o rcc.o util.o startup.o led.o button.o spark-metal.ld $(RUSTOBJS)
	arm-none-eabi-ld -T spark-metal.ld -I$(SELF_DIR)include startup.o c_main.o gpio.o rcc.o util.o led.o button.o $(RUSTOBJS) -o spark-metal.elf

c_main.o gpio.o util.o rcc.o led.o button.o: c_main.c gpio.c rcc.c util.c led.c button.c
	arm-none-eabi-gcc -c -mcpu=cortex-m3 -I$(SELF_DIR)include -g c_main.c gpio.c rcc.c util.c led.c button.c -mthumb

startup.o: startup.S
	arm-none-eabi-as -mcpu=cortex-m3 -g startup.S -o startup.o

%.o: %.rs
	$(RUSTC) $(RUSTFLAGS) -o ${@} ${<}

clean:
	rm -rf *.o spark-metal.elf spark-metal.bin

load: spark-metal.bin
	st-flash erase && st-flash write spark-metal.bin 0x08000000
