SELF_DIR := $(dir $(lastword $(MAKEFILE_LIST)))

all: spark-metal.bin

spark-metal.bin: spark-metal.elf
	arm-none-eabi-objcopy -S -O binary spark-metal.elf spark-metal.bin

spark-metal.elf: main.o gpio.o rcc.o util.o startup.o led.o button.o spark-metal.ld
	arm-none-eabi-ld -T spark-metal.ld -I$(SELF_DIR)include main.o startup.o gpio.o rcc.o util.o led.o button.o -o spark-metal.elf

main.o gpio.o util.o startup.o rcc.o led.o button.o: main.c gpio.c rcc.c util.c led.c button.c
	arm-none-eabi-gcc -c -mcpu=cortex-m3 -I$(SELF_DIR)include -g main.c gpio.c rcc.c util.c led.c button.c -mthumb

startup.o: startup.S
	arm-none-eabi-as -mcpu=cortex-m3 -g startup.S -o startup.o

clean:
	rm -rf *.o spark-metal.elf 
