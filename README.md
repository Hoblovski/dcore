# Dev
依赖
```
rustup component add llvm-tools-preview
rustup target add riscv64gc-unknown-none-elf
```

运行
```
user/ $ make
kernel/ $ make run
```

调试：在两个终端里分别
```
kernel/ $ make gdb1  # term1
kernel/ $ make gdb2  # term2
```
然后可以 `layout src` 或者 `layout asm`

使用 vscode：安装 rust-analyzer，并且把设置写到 .vscode/settings.json

细节：
如果修改了用户程序，需要 `kernel/ $ make clean` 因为 makefile 不考虑依赖

# TODO
frame_allocator 和 task_manager 一个有函数包装一个没有
