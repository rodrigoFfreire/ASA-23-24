use std::{io, num::ParseIntError};

macro_rules! max {
    ($x: expr, $y: expr) => {
        if $x > $y {
            $x
        } else {
            $y
        }
    };
}

struct Sheet {
    sheet_area: u32,
    memo_table: Vec<Vec<u32>>
}

impl Sheet {
    fn new(sheet_area: u32, piece_amount: u32) -> Sheet {
        let memo_table = vec![vec![0; (sheet_area + 1) as usize]; (piece_amount + 1) as usize];

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

    fn add_piece(&mut self, area: u32, price: u32, index: usize) {
        self.areas[index] = area;
        self.prices[index] = price;
    }
}


const SHEET_DIMENSIONS: usize = 2;
const PIECE_ARGUMENTS: usize = 3;
const PIECE_AMOUNT: usize = 1;
const ERR_FAILED_PARSE: &str = "Failed Parsing!";


fn solve_best_value(order: &Order, sheet: &mut Sheet) -> u32 {
    let matrix = &mut sheet.memo_table;
    let max_area = sheet.sheet_area as usize;
    let piece_amount = order.amount as usize;

    let areas = &order.areas;
    let prices = &order.prices;

    for i in 1..=piece_amount {
        for j in 1..=max_area {
            if areas[i] > j.try_into().unwrap() {
                matrix[i][j] = matrix[i - 1][j];
            } else {
                matrix[i][j] = max!(
                    matrix[i - 1][j],
                    matrix[i - 1][j - (areas[i] as usize)] + prices[i]
                );
            }
        }
    }

    matrix[piece_amount][max_area]
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

    let mut sheet = Sheet::new(sheet_dimensions[0] * sheet_dimensions[1], piece_amount);
    let mut order: Order = Order::new(piece_amount);

    if piece_amount <= 0 {
        eprintln!("There must be at least 1 piece!");
        return;
    }

    for i in 1..(piece_amount + 1) {
        let piece = parse_integer_tokens(PIECE_ARGUMENTS).expect(ERR_FAILED_PARSE);
        order.add_piece(piece[0] * piece[1], piece[2], i as usize);
    }

    let result = solve_best_value(&order, &mut sheet);
    println!("{}", result);
}
