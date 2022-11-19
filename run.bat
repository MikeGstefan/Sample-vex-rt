cargo objcopy -- -O binary -R .hot_init program.bin
pros upload --target v5 --slot 1 program.bin