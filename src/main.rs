use std::io;

fn draw(board: &[[&str; 3]]) {
    for i in 0..board.len() {
        println!("-------------");
        println!("{}", display_row(&board[i]));
    }
    println!("-------------");
}

fn display_cell(val: &str) -> &str {
    if val == "" { " " } else { val }
}

fn display_row(row: &[&str]) -> String {
    format!("| {} |", row.iter().map(|&x| display_cell(x)).collect::<Vec<&str>>().join(" | "))
}

fn main() {

    let board: [[&str; 3]; 3] = [[""; 3]; 3];
    draw(&board);

    println!("Please enter a move:");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("You entered: {}", input.trim()),
        Err(error) => println!("error: {}", error)
    }
}
