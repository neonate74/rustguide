extern crate greprs;
mod counter;
use counter::Counter;

use std::{time::Duration, thread};

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>
}
impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // let args: Vec<String> = args().collect();
    // let config = Config::new(&args);
    // greprs::run(config);

    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| (x % 3) == 0)
        .sum();

        // for i in Counter::new().zip(Counter::new().skip(1))
        // .map(|(a, b)| a * b)
        // .filter(|x| (x % 3) == 0) {
        //     println!("{:?}", i);
        // }

        println!("{}", sum);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushup!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situp!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            )
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
