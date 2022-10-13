file target/riscv64gc-unknown-none-elf/debug/dcore-kernel
set arch riscv:rv64
target remote localhost:1234
tbreak rust_main

# layout src and layout asm needs auto refresh

define n
  if $argc == 0
    next
  else if $argc == 1
    next $arg0
  end
  refresh
end

define si
  if $argc == 0
    stepi
  else if $argc == 1
    stepi $arg0
  end
  refresh
end
