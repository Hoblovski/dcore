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
kernel/ $ make gdb1	# term1
kernel/ $ make gdb2	# term2
```
然后可以 `layout src` 或者 `layout asm`

使用 vscode：安装 rust-analyzer，并且把设置写到 .vscode/settings.json

