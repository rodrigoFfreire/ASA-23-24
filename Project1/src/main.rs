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

struct Order {
    pieces: Vec<[usize; ARGS_PIECE]>,
    amount: usize,
}

impl Order {
    fn new(amount: usize) -> Self {
        let pieces: Vec<[usize; ARGS_PIECE]> = vec![[0, 0, 0]; amount];

        Self { pieces, amount }
    }

    fn add_piece(&mut self, x: usize, y: usize, price: usize, index: usize) {
        self.pieces[index] = [x, y, price];
    }
}

fn piece_fits(piece_x: usize, piece_y: usize, sheet_x: usize, sheet_y: usize) -> PieceFit {
    if check_fit!(piece_x, piece_y, sheet_x, sheet_y) {
        if check_fit!(rotated, piece_x, piece_y, sheet_x, sheet_y) {
            return PieceFit::FitsAll;
        }
        return PieceFit::OnlyFitsOriginal;
    } else if check_fit!(rotated, piece_x, piece_y, sheet_x, sheet_y) {
        if check_fit!(piece_x, piece_y, sheet_x, sheet_y) {
            return PieceFit::FitsAll;
        }
        return PieceFit::OnlyFitsRotated;
    }
    return PieceFit::NoFit;
}

fn calculate_best_value(
    piece_x: usize,
    piece_y: usize,
    piece_price: usize,
    matrix: &mut Vec<Vec<usize>>,
    x: usize,
    y: usize,
    max_sheet_x: usize,
) {
    let mut _best_value = 0;

    if piece_x == x && piece_y == y {
        _best_value = matrix[x][y].max(piece_price);
    } else {
        let mut h_cut_value = 0;
        let mut v_cut_value = 0;

        if x > piece_x {
            v_cut_value = matrix[piece_x][y] + matrix[x - piece_x][y];
        }
        if y > piece_y {
            h_cut_value = matrix[x][piece_y] + matrix[x][y - piece_y];
        }
        _best_value = matrix[x][y].max(h_cut_value.max(v_cut_value));
    }
    if _best_value > 0 {
        matrix[x][y] = _best_value;
        if y > x && y <= max_sheet_x {
            matrix[y][x] = _best_value;
        }
    }
}

fn solve_best_value(order: &Order, sheet_x: usize, sheet_y: usize) -> usize {
    if order.amount <= 0 {
        return 0;
    }

    let mut new_sheet_x = sheet_x;
    let mut new_sheet_y = sheet_y;
    let piece_amount = order.amount;

    if sheet_x > sheet_y {
        (new_sheet_x, new_sheet_y) = (new_sheet_y, new_sheet_x);
    }
    let mut max_value = vec![vec![0 as usize; new_sheet_y + 1]; new_sheet_x + 1];

    for x in 1..=new_sheet_x {
        for y in x..=new_sheet_y {
            for i in 0..piece_amount {
                let piece_x = order.pieces[i][DIM_X];
                let piece_y = order.pieces[i][DIM_Y];
                let piece_price = order.pieces[i][PRICE];

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
    return max_value[new_sheet_x][new_sheet_y];
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
    let sheet_dimensions = parse_integer_tokens(PARSE_ARGS_SHEET_SIZE).expect(ERR_FAILED_PARSE);

    let piece_amount = parse_integer_tokens(PARSE_ARGS_PIECE_AMOUNT).expect(ERR_FAILED_PARSE)[0];

    let mut order: Order = Order::new(piece_amount);

    for i in 0..piece_amount {
        let piece = parse_integer_tokens(ARGS_PIECE).expect(ERR_FAILED_PARSE);
        order.add_piece(piece[0], piece[1], piece[2], i);
    }

    let result = solve_best_value(&order, sheet_dimensions[DIM_X], sheet_dimensions[DIM_Y]);
    println!("{}", result);
}
