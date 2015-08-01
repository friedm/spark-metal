spark-metal
===========

A dabbling in driver development that will display a randomly colored light with each button press on the core.
Some pieces of spark's firmware have been included.
Implemented by scouring
[spark/bootloader](https://github.com/spark/bootloader)
and [spark/firmware](https://github.com/spark/firmware) for memory ranges associated with the core's gpio and their usages.
[antoinealb/rust-demo-cortex-m4](https://github.com/antoinealb/rust-demo-cortex-m4) has also been a helpful rust reference.

dependencies
----

* hardware: spark core, jtag shield, st-link 2
* gcc-arm-none binutils
* [stlink](https://github.com/texane/stlink)
* an unlocked spark bootloader (instructions at [spark/bootloader](https://github.com/spark/bootloader))

build
-----

* `make` -- build flashable spark-metal.bin
* `make load` -- build and automatically flash over stlink-2

plans
-----

* port to rust  
* uart driver
* driver for the core's CC3000 wifi module
