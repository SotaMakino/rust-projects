use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..10);

    loop {
        println!("Please input your number");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("error");

        let guess: u32 = Guess::new(guess.trim().parse().expect("patronum!")).value;

        println!("your guess is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("greater"),
            Ordering::Less => println!("less"),
            Ordering::Equal => {
                println!("you win!!!");
                break;
            }
        }
    }
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("value must be between 1 and 100, got {}", value)
        }

        Guess { value }
    }
}
