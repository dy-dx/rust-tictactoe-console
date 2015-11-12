use std::io;

fn main() {

    let board: [[char; 3]; 3] = [[' '; 3]; 3];

    println!("Please enter a move:");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("You entered: {}", input.trim()),
        Err(error) => println!("error: {}", error)
    }
}
