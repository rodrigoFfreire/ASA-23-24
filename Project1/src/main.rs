extern crate solver;
use solver::{Piece, solve_best_value};
use solver::{
    DIM_X,
    DIM_Y,
    PARSE_ARGS_PIECE_AMOUNT,
    PARSE_ARGS_SHEET_SIZE,
    ERR_FAILED_PARSE,
    ARGS_PIECE
};
use std::{io, num::ParseIntError};

macro_rules! check_fit {
    ($piece_x:expr, $piece_y:expr, $sheet_x:expr, $sheet_y:expr) => {
        $piece_x <= $sheet_x && $piece_y <= $sheet_y
    };

    (rotated, $piece_x:expr, $piece_y:expr, $sheet_x:expr, $sheet_y:expr) => {
        $piece_x <= $sheet_y && $piece_y <= $sheet_x
    };
}


fn parse_integer_tokens(amount: usize) -> Result<Vec<usize>, String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != amount {
        return Err(format!("Please input {} integers!", amount));
    }

    // Maps the string tokens into integers
    let parsed_integers: Result<Vec<usize>, ParseIntError> =
        tokens.into_iter().map(|token| token.parse()).collect();

    parsed_integers.map_err(|e| format!("Failed to parse {}", e))
}

fn main() {
    let sheet = parse_integer_tokens(PARSE_ARGS_SHEET_SIZE).expect(ERR_FAILED_PARSE);

    let mut piece_amount = parse_integer_tokens(PARSE_ARGS_PIECE_AMOUNT).expect(ERR_FAILED_PARSE)[0];

    let mut order = Vec::with_capacity(piece_amount);

    for _ in 0..piece_amount {
        let piece_info = parse_integer_tokens(ARGS_PIECE).expect(ERR_FAILED_PARSE);
        if !check_fit!(piece_info[0], piece_info[1], sheet[0], sheet[1]) &&
            !check_fit!(rotated, piece_info[0], piece_info[1], sheet[0], sheet[1]) {
                piece_amount -= 1;
            }
        else {
            order.push(Piece::new(&piece_info));
        }
    }

    let result = solve_best_value(&order, piece_amount, sheet[DIM_X], sheet[DIM_Y]);
    println!("{}", result);
}
