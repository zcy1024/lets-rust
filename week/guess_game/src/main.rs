use std::cmp::Ordering;
use std::io::stdin;
use rand::random_range;

fn main() {
    // random target number
    let target_number = random_range(1..=100);
    println!("Guess the number!");
    loop {
        println!("Please input your guess: ");
        // input guess number
        let mut guess_number = String::new();
        stdin().read_line(&mut guess_number).expect("Failed to read line");
        // try to parse it to u32
        let guess_number = match guess_number.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Error input...");
                continue;
            }
        };
        println!("You guessed: {}", guess_number);
        // compare
        match guess_number.cmp(&target_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
