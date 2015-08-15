spark-metal
===========

An embedded ARM experiment with GPIO in Rust/C. Displays a randomly colored light with each button press on the core.
Some pieces of spark's firmware have been included.
Implemented by referencing spark GPIO in
[spark/bootloader](https://github.com/spark/bootloader)
and [spark/firmware](https://github.com/spark/firmware).

Thanks to [antoinealb/rust-demo-cortex-m4](https://github.com/antoinealb/rust-demo-cortex-m4) for the helpful rust reference.

dependencies
----

* hardware: spark core, jtag shield, st-link 2
* gcc-arm-none binutils
* nightly rust
* [stlink](https://github.com/texane/stlink)
* an unlocked spark bootloader (instructions at [spark/bootloader](https://github.com/spark/bootloader))

build
-----

* `git clone https://github.com/rust-lang/rust`
* `git clone https://github.com/friedm/spark-metal`
* `cd rust && git checkout $(rustc --version | sed 's/.*(//g' | head -c9)` -- get the source for your version of rustc
* `cd ../spark-metal && make libcore` -- cross-compile libcore for your version of rustc  
* `make` -- build flashable spark-metal.bin
* `make load` -- build and automatically flash over stlink-2

plans
-----

* uart driver
* malloc
* driver for the core's CC3000 wifi module
