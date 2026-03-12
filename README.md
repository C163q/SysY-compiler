# SysY compiler

[北大编译实践](https://pku-minic.github.io/online-doc)个人题解，构建一个从 SysY 到 RISC-V 汇编的编译器。

## SysY 语言规范

SysY 语言是 C 语言的子集，其具体定义见[在线文档](https://pku-minic.github.io/online-doc/#/misc-app-ref/sysy-spec)。

[RISC-V](https://en.wikipedia.org/wiki/RISC-V) 是由加州大学伯克利分校设计并推广的第五代 RISC 指令系统体系结构。

该编译器将生成 `RV32IM` 范围内的 RISC-V 汇编。

## 使用方法

```text
sysy_compiler <flags> <src> -o <dest>
    flags:
        -koopa: 生成 koopa IR
        -riscv: 生成 RISC-V 汇编
    src: 源文件
    dest: 目标文件
```

例如：`sysy_compiler -riscv hello.c -o hello.S`

## 安装

如果你不希望将该程序安装到系统当中，只需要使用：

```shell
cargo run --release -- <args>
```

即可编译并立刻运行。其中 `<args>` 表示上面除了程序名以外的其他参数。

如果想要安装到系统当中，请运行：

```shell
cargo install --path .
```

## 运行 RISC-V 程序

如果你的系统不是 RISC-V 架构的，使用如下命令编译并运行一个完整的 RISC-V 程序
（假设已经编译出 RISC-V 汇编 `hello.S`）。

```shell
clang hello.S -c -o hello.o -target riscv32-unknown-linux-elf -march=rv32im -mabi=ilp32
ld.lld hello.o -L/path/to/libsysy/dir -lsysy -o hello
qemu-riscv32-static hello
```

其中 `libsysy` 库来自于[外部仓库](https://gitlab.eduxiji.net/csc1/nscscc/compiler2021/-/tree/master/%E5%85%AC%E5%BC%80%E7%94%A8%E4%BE%8B%E4%B8%8E%E8%BF%90%E8%A1%8C%E6%97%B6%E5%BA%93)
的 `libsysy.a` 或 `libsysy.so` 文件。

`/path/to/libsysy/dir` 应当是存放 `libsysy.a` 或 `libsysy.so` 的目录。

使用 `qemu` 来模拟 `riscv32` 架构运行程序。
