fn main() {
    let x = if true { 5 } else { 6 };
    println!("ctrlFlowDemo {x}");

    for baz in [10, 20, 30] {
        println!("for print {baz}");
    }

    for bar in (1..=4).rev() {
        println!("for range print {bar}");
    }
}
