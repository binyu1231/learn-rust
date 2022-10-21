use std::collections::HashMap;

fn _str() {
    let s = String::new();
    println!("empty string {}", s);

    let mut s = String::from("initial contents");
    let s2 = " foo";
    s.push_str(s2);
    println!("s: {}, s2: {}", s, s2);
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; 
    println!("s1: , s2: {}, s3: {}", s2, s3);
    // s1被移动了无法使用，s2可以

    let s = format!("{}-{}-{}", "1949", "10", "01");

    println!("{}", s);

    let s1 = String::from("नमस्ते");

    println!("{:?}",  &s1.chars());
    println!("{:?}",  &s1.chars().nth(0)); // O(n)
    println!("{:?}",  &s1.chars().count()); // O(n)
    println!("{:?}",  &s1.bytes().count()); // O(n)

    for c in s1.clone().chars() {
        println!("{}", c);
    }
}

fn _vec() {
    let v: Vec<i32> = Vec::new();

    println!("empty vec {:?}", &v);

    let v = Vec::from([1, 2, 3, 4]);

    println!("create vec with from {:?}", v);

    let v = vec![1, 2, 3];

    let third: &i32 = &v[2];

    println!("{} {}", v[0], third);

    match v.get(2) {
        Some(second) => {
            println!("{}", &second)
        },
        None => {
            println!("There is no third element.");
        },
    }

    if let Some(fourth) = v.get(4) {
        println!("{}", &fourth);
    }
    else {
        println!("There is no fourth.");
    }

    
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    println!("mut v {:?}", &v);
    
    v.push(66);

    println!("mut v {:?}", &v);
    
    
    println!("mut v {:?}", &v);

    if let Some(pop) = v.pop() {
        println!("v pop last {}", pop);
    }

    let pop = v.pop();
    println!("pop last {:?}", pop);

    assert_eq!(v.len(), 2);
}

fn hashmap() {
    let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // /// Read
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name); // 注意引用

    // println!("Blue team score: {:?}", score); // Some(10)

    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    let score = scores.get("Blue");

    println!("Blue team score: {:?}", score); // Some(10)


    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

}


fn main() {
    println!("Hello, world!");
    hashmap();
    // _str();
    // _vec();
}