spark-metal
===========

A dabbling in driver development that will display a randomly colored light with each button press on the core.
Some pieces of spark's firmware have been included.
Implemented by scouring
[spark/bootloader](https://github.com/spark/bootloader)
and [spark/firmware](https://github.com/spark/firmware) for memory ranges associated with the core's gpio and their usages.
[antoinealb/rust-demo-cortex-m4](https://github.com/antoinealb/rust-demo-cortex-m4) has also been a helpful rust reference.

build
-----

`make` -- depends on gcc-arm-none

The bootloader must be unlocked (instructions at [spark/bootloader](https://github.com/spark/bootloader)) and [stlink](https://github.com/texane/stlink) installed before running the following:  
`st-flash erase && st-flash write spark-metal.bin 0x08000000`

plans
-----

* port to rust  
* uart driver
* driver for the core's CC3000 wifi module

uart memory location
----
0x40005C00
0x40006000
