spark-metal
===========

A dabbling in driver development that will display a randomly colored light with each button press on the core.
No libraries are referenced, but a some pieces of spark's firmware are included.
Implemented by scouring
[spark/bootloader](https://github.com/spark/bootloader) 
and [spark/firmware](https://github.com/spark/firmware) for memory ranges associated with the core's gpio and their usages. 

build
-----

`make` -- depends on gcc-arm-none

The bootloader must be unlocked (instructions at [spark/bootloader](https://github.com/spark/bootloader)) and [stlink](https://github.com/texane/stlink) before running the following:
`st-flash erase && st-flash write spark-metal.bin 0x08000000` 

[stlink](https://github.com/texane/stlink)

plans
-----

* uart driver
* driver for the core's CC3000 wifi module

