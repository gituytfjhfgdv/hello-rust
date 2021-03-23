// std : 標準ライブラリの意味
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess) // `&` はその引数が参照であることを意味する &mut guess は可変の参照の意味
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
        };
        println!("You fuessed: {}", guess);

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

// let foo=5; // immutable
// let mut bar=5; //mutable
