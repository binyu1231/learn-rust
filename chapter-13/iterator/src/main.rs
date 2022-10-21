#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }
        else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        // [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5]]
        .map(|(a, b)| a * b)
        // // [0, 2, 6, 12, 20]
        .filter(|x| x % 3 == 0)
        // // [6, 12]
        .sum();
        // // 18
    
    println!("sum: {}", sum);
}
