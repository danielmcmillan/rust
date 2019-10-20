use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 21);

    loop {
        println!("Enter a number");

        let mut guess = String::new(); // new is 'associated function' (= static method) of String
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
