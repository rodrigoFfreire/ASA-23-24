use std::{io, num::ParseIntError, vec};

struct Sheet {
    sheet_area: u32,
    memo_table: Vec<Vec<u32>>,
}

impl Sheet {
    fn new(sheet_area: u32) -> Sheet {
        let memo_table = vec![vec![0; (sheet_area + 1) as usize]; (sheet_area + 1) as usize];
        Sheet {
            sheet_area,
            memo_table,
        }
    }
}

struct Order {
    areas: Vec<u32>,
    prices: Vec<u32>,
    amount: u32,
}

impl Order {
    fn new(amount: u32) -> Order {
        let areas: Vec<u32> = vec![0; (amount + 1) as usize];
        let prices: Vec<u32> = vec![0; (amount + 1) as usize];
        Order {
            areas,
            prices,
            amount,
        }
    }

    fn add_piece(&mut self, area: u32, price: u32) {
        self.areas.push(area);
        self.prices.push(area);
    }
}

const SHEET_DIMENSIONS: usize = 2;
const PIECE_ARGUMENTS: usize = 3;
const PIECE_AMOUNT: usize = 1;
const ERR_FAILED_PARSE: &str = "Failed Parsing!";


fn solve_max_value_order(order: &Order, sheet: &Sheet) -> u32 {
    0
}

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
    let sheet_dimensions = parse_integer_tokens(SHEET_DIMENSIONS).expect(ERR_FAILED_PARSE);

    let piece_amount = parse_integer_tokens(PIECE_AMOUNT).expect(ERR_FAILED_PARSE)[0];

    let mut sheet = Sheet::new(sheet_dimensions[0] * sheet_dimensions[1]);
    let mut order: Order = Order::new(piece_amount);

    if piece_amount <= 0 {
        eprintln!("There must be at least 1 piece!");
        return;
    }

    for _ in 0..piece_amount {
        let piece = parse_integer_tokens(PIECE_ARGUMENTS).expect(ERR_FAILED_PARSE);
        order.add_piece(piece[0] * piece[1], piece[2]);
    }

    println!("Ok!");
}
