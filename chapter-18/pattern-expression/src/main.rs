fn main() {
    let (_x, _y, _z) = (1, 2, 3);
    let (_x, _, _) = (1, 2, 3);
    let (_x, ..) = (1, 2, 3);
    
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // 声明了新变量，遮蔽了外部变量
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x)

        // Matched, y = 5
    }


    println!("at the end: x = {:?}, y = {:?}", x, y);
    // at the end: x = Some(5), y = 10
    
}
