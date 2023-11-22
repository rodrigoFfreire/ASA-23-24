mod constants;
use crate::constants::*;
use std::{io, num::ParseIntError, usize};

macro_rules! check_fit {
    ($piece_x:expr, $piece_y:expr, $sheet_x:expr, $sheet_y:expr) => {
        $piece_x <= $sheet_x && $piece_y <= $sheet_y
    };

    (rotated, $piece_x:expr, $piece_y:expr, $sheet_x:expr, $sheet_y:expr) => {
        $piece_x <= $sheet_y && $piece_y <= $sheet_x
    };
}

macro_rules! matrix_get {
    ($x:expr, $y:expr, $m:expr) => {
        $m.table[$y * $m.matrix_x + $x]
    }
}

macro_rules! matrix_set {
    ($x:expr, $y:expr, $m:expr, $value:expr) => {
        $m.table[$y * $m.matrix_x + $x] = $value;
    };
}

struct Matrix {
    matrix_x: usize,
    table: Vec<usize>,
    // matrix_y doesnt need to be defined since its not used in any operation
}

impl Matrix {
    fn new(matrix_x: usize, matrix_y: usize) -> Self {
        Self {
            matrix_x,
            table: vec![0; matrix_x * matrix_y],
        }
    }
}

struct Piece {
    x: usize,
    y: usize,
    price: usize
}

impl Piece {
    fn new(piece_info: &Vec<usize>) -> Self {
        Self { 
            x: piece_info[DIM_X],
            y: piece_info[DIM_Y],
            price: piece_info[PRICE]
        }
    }
}

fn piece_fits(&piece_x: &usize, &piece_y: &usize, sheet_x: usize, sheet_y: usize) -> PieceFit {
    match piece_x * piece_y > sheet_x * sheet_y {
        true => PieceFit::NoFit,
        false => match (check_fit!(piece_x, piece_y, sheet_x, sheet_y),
                        check_fit!(rotated, piece_x, piece_y, sheet_x, sheet_y)) {
            (true, true) => PieceFit::FitsAll,
            (true, false) => PieceFit::OnlyFitsOriginal,
            (false, true) => PieceFit::OnlyFitsRotated,
            (false, false) => PieceFit::NoFit,
        }
    }
}
    

fn calculate_best_value(
    &piece_x: &usize,
    &piece_y: &usize,
    &piece_price: &usize,
    matrix: &mut Matrix,
    x: usize,
    y: usize,
    max_sheet_x: usize,
) {
    let mut _best_value = 0;

    if piece_x == x && piece_y == y {
        _best_value = matrix_get!(x, y, matrix).max(piece_price);
    } else {
        let mut h_cut_value = 0;
        let mut v_cut_value = 0;

        if x > piece_x {
            v_cut_value = matrix_get!(piece_x, y, matrix) + matrix_get!(x - piece_x, y, matrix);
        }
        if y > piece_y {
            h_cut_value = matrix_get!(x, piece_y, matrix) + matrix_get!(x, y - piece_y, matrix);
        }
        _best_value = matrix_get!(x, y, matrix).max(h_cut_value.max(v_cut_value));
    }
    if _best_value > 0 {
        matrix_set!(x, y, matrix, _best_value);
        if y > x && y <= max_sheet_x {
            matrix_set!(y, x, matrix, _best_value);
        }
    }
}

fn solve_best_value(order: &Vec<Piece>, amount: usize, sheet_x: usize, sheet_y: usize) -> usize {
    if amount <= 0 {
        return 0;
    }

    let mut new_sheet_x = sheet_x;
    let mut new_sheet_y = sheet_y;

    if sheet_x > sheet_y {
        (new_sheet_x, new_sheet_y) = (new_sheet_y, new_sheet_x);
    }
    let mut max_value = Matrix::new(new_sheet_x + 1, new_sheet_y + 1);

    for x in 1..=new_sheet_x {
        for y in x..=new_sheet_y {
            for piece in order.iter() {
                let Piece { x: piece_x, y: piece_y, price: piece_price} = piece;
                let fits = piece_fits(piece_x, piece_y, x, y);

                match fits {
                    PieceFit::NoFit => continue,
                    PieceFit::OnlyFitsOriginal => calculate_best_value(
                        piece_x, piece_y, piece_price, &mut max_value, x, y, new_sheet_x,
                    ),
                    PieceFit::OnlyFitsRotated => calculate_best_value(
                        piece_y, piece_x, piece_price, &mut max_value, x, y, new_sheet_x,
                    ),
                    PieceFit::FitsAll => {
                        calculate_best_value(
                            piece_x, piece_y, piece_price, &mut max_value, x, y, new_sheet_x,
                        );
                        calculate_best_value(
                            piece_y, piece_x, piece_price, &mut max_value, x, y, new_sheet_x,
                        )
                    }
                }
            }
        }
    }
    return matrix_get!(new_sheet_x, new_sheet_y, max_value);
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

    let piece_amount = parse_integer_tokens(PARSE_ARGS_PIECE_AMOUNT).expect(ERR_FAILED_PARSE)[0];

    let mut order = Vec::with_capacity(piece_amount);

    for _ in 0..piece_amount {
        let piece_info = parse_integer_tokens(ARGS_PIECE).expect(ERR_FAILED_PARSE);
        order.push(Piece::new(&piece_info));
    }

    let result = solve_best_value(&order, piece_amount, sheet[DIM_X], sheet[DIM_Y]);
    println!("{}", result);
}
