fn main() {
    println!("Hello, world!");

    let number_list = vec![30, 50, 25];
    let char_list = vec!['y', 'm', 'c', 'a'];

    println!("max number: {}", max(&number_list));
    println!("max char: {}", max(&char_list));

    println!("number list {:?}", number_list);
    println!("char list {:?}", char_list);
    
    println!("min number: {}", min(&number_list));
    println!("min char: {}", min(&char_list));
    
    println!("number list {:?}", number_list);
    println!("char list {:?}", char_list);
}

fn max<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    max
}

fn min<T: PartialOrd>(list: &[T]) -> &T {
    let mut min = &list[0];
    for item in list {
        if item < min {
            min = &item
        }
    }

    return min
}
