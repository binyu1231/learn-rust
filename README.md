# Rust 程序语言设计

- Reference: <https://kaisery.github.io/trpl-zh-cn/title-page.html>
- crate: <https://crates.io/>

## Chapter 1

```bash
$ rustc --version
$ cargo --version 

# 创建项目
$ cargo new <project_name>

# 构建项目
<project_name>$ cargo build 
<project_name>$ .\target\debug\hello-cargo.exe

<project_name>$ cargo build --release

# 一步构建并运行项目
<project_name>$ cargo run

# 在不生成二进制文件的情况下构建项目来检查错误
<project_name>$ cargo check

# 更新以来
<project_name>$ cargo update

# 构建所有安装包crate的文档 以查看trait
<project_name>$ cargo doc --open
```

``` rust
println();  // 函数调用
println!(); // 宏调用

let apples = 5;
println!("Your apples {apples}"); // "Your apples 5"

let x = 5; 
let y = 10;
println!("x = {} and y = {}", x, y); // "x = 5 and y = 10"
```

## 变量

``` rust
let apples = 5;      // 不可变
let mut bananas = 5; // 可变

// 变量遮蔽
let x = 5;
let x = x + 1; // 声明一个同名变量 而不是赋值旧变量
{
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}"); // 12
}
println!("The value of x is: {x}"); // 6

const FOO_BAZ: u32 = 12; // 常量必须注明类型

```

## 循环 

``` rust
loop {
    continue;

    break;
}

```

## 数据类型 data type

### 标量 scalar

- 整型
- 浮点型
- 布尔类型
- 字符类型

#### 整型

i8, i16, i32, i64, i128, isise 以及对应的无符号整型 u8 ..

字面量表示

```rust
const DECIMAL: u32 = 89_222; // 自定义下划线位置方便读数
const HEX: u8 = 0xff;
const OCTAL: u8 = 0o77;
const BINARY: u8 = 0b1111_0000;
const BYTE = u8 = b'A'; // 仅限 u8

```

Note: 整型溢出时与 % 运算结果相同(即 256 => 0), 如果需要这种功能最好使用标准库 Wrapping 显式调用

#### 浮点型

f32, f64 现代浏览器中速度几乎一样，默认为f64

``` rust
let x = 2.0;
let y: f32: 3.0;
```

#### 布尔类型

bool

``` rust
let loading = true;
let initialize: bool = false;
```

#### 字符类型

char 四个字节，可以表示

``` rust
let foo = 'z';
let bar: char = 'ℤ';
let baz = '😻';
```

### 复合 compound

- 原生复合类型
    - 元组 tuple
    - 数组 array

#### 元组 tuple

``` rust
let bar: (i32. f64, u8) = (500, 6.4, 1);
let baz = (500, 6.4, 1);
// 解构 
let (x, y, z) = baz;

baz.0; // 500
baz.1; // 6.4
baz.2; // 1

// 单元 unit 特殊的元组
let foo: () = ();
```

#### 数组

``` rust
let weekends = ["Sat.", "Sun."];
let foo: [i32; 5] = [1, 2, 3, 4, 5];
let bar = [3; 5]; // [3, 3, 3, 3, 3];

weekends[0]; // "Sat."
weekends[1]; // "Sun."

```


## 函数 fn

``` rust
fn foo(x: 32) {}
fn bar() -> i32 {
    5
}
fn bar() -> i32 {
    return 5;
}

bar(); // 5;

// 不能通过编译，需要删除分号
fn baz() -> i32 {
    5;
}

```

## 注释

``` rust
// abc
/* def */
```

## OO

``` rust
String::new(); // 实例化
```

## Result 

```
```

## trait

## 范围表达式

``` rs
// start..=end
1..=100
```