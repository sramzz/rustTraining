use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to the GUESSING GAME!!!");
    let magic_number: u8;
    magic_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter a number");
        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");
        let user_guess: u8 = match user_guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guesed number was:");

        match user_guess.cmp(&magic_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Perfect! You have win!");
                break;
            }
        }
    }
    println!("Magic number was:{magic_number}");
}
