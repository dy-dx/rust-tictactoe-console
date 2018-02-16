use std::io;
use std::io::Write;

fn draw(board: &[[&str; 3]]) {
    for i in 0..board.len() {
        println!("-------------");
        println!("{}", display_row(&board[i]));
    }
    println!("-------------");
}

fn display_cell(val: &str) -> &str {
    match val {
        "" => " ",
        _ => val,
    }
}

fn display_row(row: &[&str]) -> String {
    format!(
        "| {} |",
        row.iter()
            .map(|&x| display_cell(x))
            .collect::<Vec<&str>>()
            .join(" | ")
    )
}

fn get_input(prompt: &str) -> Result<usize, String> {
    print!("{}", prompt);
    io::stdout().flush().expect("Couldn't flush");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read input");

    match input.trim().parse::<usize>() {
        Ok(n) => Ok(n),
        Err(err) => Err(err.to_string()),
    }
}

fn player_turn(player: i32, board: &mut [[&str; 3]]) -> bool {
    let player_string = match player {
        0 => "X",
        _ => "O",
    };

    println!("Please enter a move for player {}.", player_string);

    let row_index = match get_input("Row (0, 1, 2): ") {
        Ok(n) => n,
        _ => return false,
    };

    let col_index = match get_input("Col (0, 1, 2): ") {
        Ok(n) => n,
        _ => return false,
    };

    if row_index > 2 || col_index > 2 || board[row_index][col_index] != "" {
        return false;
    }

    board[row_index][col_index] = player_string;
    return true;
}

fn main() {
    let mut board: [[&str; 3]; 3] = [[""; 3]; 3];
    let mut current_player = 0;

    loop {
        draw(&board);
        match player_turn(current_player, &mut board) {
            true => current_player = 1 - current_player,
            false => println!("Try again."),
        };
    }
}
