target extended-remote :3333
file spark-metal.elf
break rust_panic
load
