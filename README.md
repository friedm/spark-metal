spark-metal
===========

A dabbling in driver development that will display a randomly colored light with each button press on the core.
No libraries are referenced, but a some pieces of spark's firmware are included.
Implemented by scouring
[spark/bootloader](https://github.com/spark/bootloader)
and [spark/firmware](https://github.com/spark/firmware) for memory ranges associated with the core's gpio and their usages.

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

* uart driver
* driver for the core's CC3000 wifi module
