fn main() {

    let mut a = 1;
    println!("mut a: {a}");
    a = 2;
    println!("muted a: {a}");
    
    const B: u32 = 6;
    println!("const B: {B}");




    let x = 1;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces length: {spaces}");


    const DECIMAL: u32 = 89_222;
    const HEX: u8 = 0xff;
    const OCTAL: u8 = 0o77;
    const BINARY: u8 = 0b1111_0000;
    const BYTE: u8 = b'A'; // 仅限 u8

    println!("字面量表示法: 十进制: {DECIMAL}; 十六进制: {HEX}; 八进制:{OCTAL}; 二进制:{BINARY}; 单字节字符:{BYTE}");

    /**
     * 
     */
    fn five() -> i32 {
        5
    }
    
    let x = five();

    println!("函数表达式 five, {x}");

    fn plus_one(x: i32) -> i32 {
        return x + 1;
    }

    let x = plus_one(x);
    println!("函数返回值 plus_one {x}");
}
