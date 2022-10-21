use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    // value: Option<u32>,
    value_map: HashMap<u32, u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            // value: None,
            value_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        let cache_value = self.value_map.get(&arg);
        println!("cache value: {:?}", cache_value);
        match cache_value {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value_map.insert(arg, v);
                v
            }
        }
        // match self.value {
        //     Some(v) => v,
        //     None => {
        //         let v = (self.calculation)(arg);
        //         v
        //     }
        // }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    }
    else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }
        else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let add_one = |x: u32| -> u32 { x + 1 };
    // let add_one = |x| {x + 1};
    // let add_one = |x| x + 1;

    let three = add_one(2);
    println!("Hello, world! {}", three);

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        c.value(1); // v1
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}