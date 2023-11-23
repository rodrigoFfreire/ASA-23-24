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


fn parse_integer_tokens(amount: usize) -> Result<Vec<u32>, String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != amount {
        return Err(format!("Please input {} integers!", amount));
    }

    // Maps the string tokens into integers
    let parsed_integers: Result<Vec<u32>, ParseIntError> =
        tokens.into_iter().map(|token| token.parse()).collect();

    parsed_integers.map_err(|e| format!("Failed to parse {}", e))
}

fn main() {
    let sheet = parse_integer_tokens(PARSE_ARGS_SHEET_SIZE).expect(ERR_FAILED_PARSE);

    let piece_amount = parse_integer_tokens(PARSE_ARGS_PIECE_AMOUNT).expect(ERR_FAILED_PARSE)[0];

    let mut order = Vec::with_capacity(piece_amount as usize);

    for _ in 0..piece_amount {
        let piece_info = parse_integer_tokens(ARGS_PIECE).expect(ERR_FAILED_PARSE);
        order.push(Piece::new(&piece_info));
    }

    let result = solve_best_value(&order, piece_amount, sheet[DIM_X], sheet[DIM_Y]);
    println!("{}", result);
}
