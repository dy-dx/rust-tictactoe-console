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

fn player_to_string(player: u16) -> &'static str {
    match player {
        0 => "X",
        _ => "O",
    }
}

fn is_direction_filled(player: &'static str, list: Vec<&str>) -> bool {
    let mut is_filled = true;
    for i in 0..list.len() {
        if &list[i] != &player {
            is_filled = false;
        }
    }
    return is_filled;
}

fn is_stalemate(board: &[[&str; 3]]) -> bool {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == "" {
                return false;
            }
        }
    }
    return true;
}

fn did_player_win(player: &'static str, board: &mut [[&str; 3]]) -> bool {
    for i in 0..board.len() {
        // check row i
        let row = board[i].to_vec();
        if is_direction_filled(player, row) {
            return true;
        }

        // check col i
        let col = board.iter().map(|&r| r[i]).collect::<Vec<&str>>();
        if is_direction_filled(player, col) {
            return true;
        }
    }

    // check ↘ diagonals
    let mut i = 0;
    let diagonals = board
        .iter()
        .map(|&r| {
            let cell = r[i];
            i += 1;
            cell
        })
        .collect::<Vec<&str>>();

    if is_direction_filled(player, diagonals) {
        return true;
    }

    // check ↙ diagonals
    i = board.len();
    let reverse_diagonals = board
        .iter()
        .map(|&r| {
            i -= 1;
            r[i]
        })
        .collect::<Vec<&str>>();

    if is_direction_filled(player, reverse_diagonals) {
        return true;
    }

    return false;
}

fn player_turn(player: &'static str, board: &mut [[&str; 3]]) -> bool {
    let mut row_index;
    let mut col_index;

    loop {
        draw(&board);
        println!("Please enter a move for player {}.", player);

        row_index = match get_input("Row (0, 1, 2): ") {
            Ok(n) => n,
            _ => {
                println!("Try again.");
                continue;
            }
        };

        col_index = match get_input("Col (0, 1, 2): ") {
            Ok(n) => n,
            _ => continue,
        };

        if row_index > 2 || col_index > 2 || board[row_index][col_index] != "" {
            println!("Try again.");
            continue;
        }

        break;
    }

    board[row_index][col_index] = player;
    return did_player_win(player, board);
}

fn main() {
    loop {
        let mut board = [[""; 3]; 3];
        let mut current_player = 0;

        loop {
            if player_turn(player_to_string(current_player), &mut board) {
                draw(&board);
                println!("You win!");
                break;
            } else {
                if is_stalemate(&board) {
                    println!("Stalemate!");
                    break;
                };
                current_player = 1 - current_player
            }
        }
    }
}
