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

macro_rules! _check_fit {
    ($piece_x:expr, $piece_y:expr, $sheet_x:expr, $sheet_y:expr) => {
        $piece_x <= $sheet_x && $piece_y <= $sheet_y
    };

    (rotated, $piece_x:expr, $piece_y:expr, $sheet_x:expr, $sheet_y:expr) => {
        $piece_x <= $sheet_y && $piece_y <= $sheet_x
    };
}


fn _parse_integer_tokens(amount: usize) -> Result<Vec<usize>, String> {
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
  /*   let sheet = _parse_integer_tokens(PARSE_ARGS_SHEET_SIZE).expect(ERR_FAILED_PARSE);

    let mut piece_amount = _parse_integer_tokens(PARSE_ARGS_PIECE_AMOUNT).expect(ERR_FAILED_PARSE)[0];

    let mut order = Vec::with_capacity(piece_amount);

    for _ in 0..piece_amount {
        let piece_info = _parse_integer_tokens(ARGS_PIECE).expect(ERR_FAILED_PARSE);
        if !_check_fit!(piece_info[0], piece_info[1], sheet[0], sheet[1]) &&
            !_check_fit!(rotated, piece_info[0], piece_info[1], sheet[0], sheet[1]) {
                piece_amount -= 1;
            }
        else {
            order.push(Piece::new(&piece_info));
        }
    } */
    let sheet: [usize; 2] = [500, 500];
    let piece_amount = 10;
    let mut order = Vec::with_capacity(piece_amount);
    order.push(Piece::new(&vec![7, 4, 10]));
    order.push(Piece::new(&vec![6, 3, 8]));
    order.push(Piece::new(&vec![1, 11, 645]));
    order.push(Piece::new(&vec![2, 3, 541]));
    order.push(Piece::new(&vec![3, 7, 432]));
    order.push(Piece::new(&vec![4, 3, 224]));
    order.push(Piece::new(&vec![5, 2, 400]));
    order.push(Piece::new(&vec![6, 9, 200]));
    order.push(Piece::new(&vec![7, 12, 12]));
    order.push(Piece::new(&vec![10, 1, 100]));

    /* let sheet: [usize; 2] = [11, 6];
    let piece_amount = 4;
    let mut order = Vec::with_capacity(piece_amount);
    order.push(Piece::new(&vec![5, 3, 10], sheet[0]));
    order.push(Piece::new(&vec![4, 4, 20], sheet[0]));
    order.push(Piece::new(&vec![8, 3, 50], sheet[0]));
    order.push(Piece::new(&vec![11, 4, 100], sheet[0])); */

    let result = solve_best_value(&order, piece_amount, sheet[DIM_X], sheet[DIM_Y]);
    println!("{}", result);
}
