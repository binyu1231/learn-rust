pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        }
        else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn another() {
        // panic!("Make this test fail.");
        assert!(2 == 2);
        assert_eq!(2, 2);
        assert_ne!('2', '5');
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn less_than_1() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100, got 200.")]
    fn greater_than_100() {
        let guess = Guess::new(200);
        println!("Guess Value: {}", guess.value);
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        }
        else {
            Err(String::from("two plus two does not equal four."))
        }
    }
}
