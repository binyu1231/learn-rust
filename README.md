# Rust 程序语言设计

- Reference: <https://kaisery.github.io/trpl-zh-cn/title-page.html>
- crate: <https://crates.io/>
- 笔记整理到 <https://www.binyu.me/shortcut#Language> Rust 栏目下

## 待整理
## OO

``` rust
String::new(); // 实例化

let mut s = String::from("hello");
s.push_str(", world!");

println!("{}", s);

let bytes = s.as_bytes(); // 字符串转化为字节数组
for (i, &item) in bytes.iter().enumerate() {}

s.clear(); // s => ""
```

## Result 

```
```

