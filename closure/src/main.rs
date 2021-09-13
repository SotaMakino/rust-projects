use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Casher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Casher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Casher<T> {
        Casher {
            calculation: calculation,
            value: None,
        }
    }

    fn calc(&mut self, arg: u32) -> u32 {
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

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Casher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Do {} pushups", expensive_closure.calc(intensity));

        println!("Next do {} situps", expensive_closure.calc(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.calc(intensity)
            );
        }
    }
}

fn simulated_expensive_calc(intensity: u32) -> u32 {
    println!("slow exec...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
