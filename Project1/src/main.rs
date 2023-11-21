mod constants;
use std::{io, num::ParseIntError, usize};
use crate::constants::*;

struct Order {
    pieces: Vec<[usize; PARSE_ARGS_PIECE]>,
    amount: usize,
}

impl Order {
    fn new(amount: usize) -> Self {
        let pieces: Vec<[usize; PARSE_ARGS_PIECE]> = vec![[0, 0, 0]; amount];

        Self {
            pieces,
            amount,
        }
    }

    fn add_piece(&mut self, x: usize, y: usize, price: usize, index: usize) {
        self.pieces[index] = [x, y, price];
    }
}

fn piece_fits(piece_x: usize, piece_y: usize, current_sheet_x: usize, current_sheet_y: usize) -> bool {
    return piece_x <= current_sheet_x && piece_y <= current_sheet_y;
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
                let mut piece_x = order.pieces[i][DIM_X];
                let mut piece_y = order.pieces[i][DIM_Y];
                let piece_price = order.pieces[i][PRICE];

                if piece_x > piece_y {
                    (piece_x, piece_y) = (piece_y, piece_x);
                }
                if piece_fits(piece_x, piece_y, x, y) {
                    if piece_x == x && piece_y == y {
                        max_value[x][y] = max_value[x][y].max(piece_price);
                    } else {
                        let mut h_cut_value = 0;
                        let mut v_cut_value = 0;

                        if x > piece_x {
                            v_cut_value = max_value[piece_x][y] + max_value[x - piece_x][y];
                        }
                        if y > piece_y {
                            h_cut_value = max_value[x][piece_y] + max_value[x][y - piece_y];
                        }

                        max_value[x][y] = max_value[x][y].max(
                            h_cut_value.max(v_cut_value)
                        );
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
        let piece = parse_integer_tokens(PARSE_ARGS_PIECE).expect(ERR_FAILED_PARSE);
        order.add_piece(piece[0], piece[1], piece[2], i);
    }

    let result = solve_best_value(&order, sheet_dimensions[DIM_X], sheet_dimensions[DIM_Y]);
    println!("{}", result);
}
