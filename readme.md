# My Rust Interpreter

参考《用Go语言自制解释器》，使用 rust 编写的解释器

# 一些感悟

1. Lexer 主要就是关注如何切分代码的字符串。里面就会涉及到判断什么是表达式(expression)，什么是语句(statement);
   也还有如何把一个字符串切成token。比如现在本语言只支持int这种数值类型，其表达式就是类似于 `5`, `10`，但是别的
   变成语言，比如 rust 中，有效的数值字面量可以是 `1u8`, `1_u8`, `1_000_u64`, `1_000u64`, `5f32`, `5.0f32`
   等等整数或者浮点数类型。新增一种类型就需要再tokenize，parse，eval的地方都各自增加这种类型。
2. Parser 则是对于如何把切分完成的token做组装的主要代码。语法树也是在这一步构建。
3. Evaluator 是最终把语法树遍历执行的。
4. Compiler 是把Ast转换成栈指令+变量的
5. VM 则是把栈指令一个一个取出，执行，跳转的
6. 善用 `Rc<String>` 可以减少后期 `String.clone()`上需要耗费的时间和内存（不只是`String`，别的数据类型
   也可以这样考虑

这本书的第二部分叫做《用Go语言自制编译器》，~~设计到了把源码编译成为机器二进制的部分。我推测这里可能会使用一个业界已经有的
语言后端。主要的工作应该是生成中间产物交给后端（比如llvm）~~ 是要实现一个虚拟机。

# Status
[![CI](https://github.com/qinyuhang/my-rust-interpreter/actions/workflows/ci.yaml/badge.svg?branch=master)](https://github.com/qinyuhang/my-rust-interpreter/actions/workflows/ci.yaml)

# 补充
用到的工具

用来查看`macro`扩展之后的情况`cargo expand`

# 如何使用
启动repl，默认interpreter `cargo run -- repl`

使用vm来启动 `cargo run -- repl --vm`

编译 `.mok` -> `.mokb` : `cargo run -- compile -i <in>.mok -o <out>.mokb`

# TODO
[] 调用优化

[] 编译到 LLVM-IR


