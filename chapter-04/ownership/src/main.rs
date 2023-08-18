fn main() {
    let s1 = String::from("bar"); // s进入作用域

    // 可以通过引用reference 将变量借出但不转移所有权
    fn cal_len(s: &String) -> usize {
        s.len()
    }

    let len = cal_len(&s1);
    // s2 在作用域结束前仍然有效
    println!("s1: {}, s1 length: {}", s1, len);
}
