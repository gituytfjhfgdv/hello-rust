// std : 標準ライブラリの意味
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess) // `&` はその引数が参照であることを意味する &mut guess は可変の参照の意味
        .expect("Failed to read line");

    println!("You fuessed: {}", guess);
}

// let foo=5; // immutable
// let mut bar=5; //mutable
