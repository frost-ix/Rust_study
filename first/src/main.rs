use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Calculate number");
    let _secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input number >> ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Call bigger"),
            Ordering::Greater => println!("Call lower"),
            Ordering::Equal => {
                println!("You win!");
                println!("Secret number is {_secret_number}");
                break;
            }
        }
    }
}
