use std::io;

#[allow(unused)]
fn main() {
    println!("Guess a number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("you guessed : {guess}");
}
