use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number babyyy!");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");
    
        let mut guess: String = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line :(");
    
        let guess: u8 = match guess.trim().parse() {
            Ok(num_received_i_guess) => num_received_i_guess,
            Err(_) => continue
        };
        
        println!("you guessed {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("go bigger pal"),
            Ordering::Equal => {println!("nice"); break;},
            Ordering::Greater => println!("go smaller bud"),
        }
    }
}
