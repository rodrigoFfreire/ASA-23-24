mod constants;
use std::{io, num::ParseIntError};
use crate::constants::*;

struct Order {
    areas: Vec<usize>,
    prices: Vec<usize>,
    amount: usize,
}

impl Order {
    fn new(amount: usize) -> Self {
        let areas: Vec<usize> = vec![0; amount];
        let prices: Vec<usize> = vec![0; amount];

        Self {
            areas,
            prices,
            amount,
        }
    }

    fn add_piece(&mut self, x: usize, y: usize, price: usize, index: usize) {
        self.areas[index] = x * y;
        self.prices[index] = price;
    }
}

fn solve_best_value(order: &Order, max_area: usize) -> usize {
    let piece_amount = order.amount;
    let areas = &order.areas;
    let prices = &order.prices;

    let mut max_value = vec![0; max_area + 1];

    for w in 0..=max_area {
        for i in 0..piece_amount {
            if areas[i] <= w {
                max_value[w] = max_value[w].max(max_value[w - areas[i]] + prices[i]);
            }
        }
    }

    return max_value[max_area];
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
    let max_area = sheet_dimensions[DIM_X] * sheet_dimensions[DIM_Y];

    if piece_amount <= 0 {
        eprintln!("There must be at least 1 piece!");
        return;
    }

    for i in 0..piece_amount {
        let piece = parse_integer_tokens(PARSE_ARGS_PIECE).expect(ERR_FAILED_PARSE);
        order.add_piece(piece[0], piece[1], piece[2], i);
    }

    let result = solve_best_value(&order, max_area);
    println!("{}", result);
}
