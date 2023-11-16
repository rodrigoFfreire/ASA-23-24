use std::{io, num::ParseIntError, vec};

macro_rules! max {
    ($x: expr, $y: expr) => {
        if $x > $y {
            $x
        } else {
            $y
        }
    };
}

const SHEET_DIMENSIONS: usize = 2;
const PIECE_ARGUMENTS: usize = 3;
const PIECE_AMOUNT: usize = 1;
const DIM_X: usize = 0;
const DIM_Y: usize = 1;
const ERR_FAILED_PARSE: &str = "Failed Parsing!";

struct Sheet {
    sheet_x: u32,
    sheet_y: u32,
    memo_table: Vec<Vec<Vec<u32>>>
}

impl Sheet {
    fn new(sheet_x: u32, sheet_y: u32, piece_amount: u32) -> Sheet {
        let memo_table =
            vec![vec![vec![0; (sheet_y + 1) as usize]; (sheet_x + 1) as usize]; (piece_amount + 1) as usize];

        Sheet {
            sheet_x,
            sheet_y,
            memo_table
        }
    }
}


struct Order {
    dimensions: Vec<Vec<u32>>,
    prices: Vec<u32>,
    amount: u32,
}

impl Order {
    fn new(amount: u32) -> Order {
        let dimensions = vec![vec![0; (SHEET_DIMENSIONS + 1) as usize]; (amount + 1) as usize];
        let prices: Vec<u32> = vec![0; (amount + 1) as usize];

        Order {
            dimensions,
            prices,
            amount,
        }
    }

    fn add_piece(&mut self, x: u32, y: u32, price: u32, index: usize) {
        self.dimensions[index][DIM_X] = x;
        self.dimensions[index][DIM_Y] = y;
        self.prices[index] = price;
    }
}


fn solve_best_value(order: &Order, sheet: &mut Sheet) -> u32 {
    let matrix = &mut sheet.memo_table;
    let piece_amount = order.amount as usize;
    let max_x = sheet.sheet_x as usize;
    let max_y = sheet.sheet_y as usize;

    let dimensions = &order.dimensions;
    let prices = &order.prices;

    for i in 1..=piece_amount {
        for j in 1..=max_x {
            for k in 1..=max_y {
                if dimensions[i][DIM_X] > j.try_into().unwrap() {
                    matrix[i][j][k] = matrix[i - 1][j][k];
                } else {
                    if dimensions[i][DIM_Y] > k.try_into().unwrap() {
                        matrix[i][j][k] = matrix[i - 1][j][k];
                    } else {
                        matrix[i][j][k] = max!(
                            matrix[i - 1][j][k],
                            matrix[i - 1][j - (dimensions[i][DIM_X]) as usize][k - (dimensions[i][DIM_Y]) as usize] + prices[i]
                        );
                    }
                }
            }
        }
    }
    
    matrix[piece_amount][max_x][max_y]
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

    let mut sheet = Sheet::new(sheet_dimensions[0], sheet_dimensions[1], piece_amount);
    let mut order: Order = Order::new(piece_amount);

    if piece_amount <= 0 {
        eprintln!("There must be at least 1 piece!");
        return;
    }

    for i in 1..(piece_amount + 1) {
        let piece = parse_integer_tokens(PIECE_ARGUMENTS).expect(ERR_FAILED_PARSE);
        order.add_piece(piece[0], piece[1], piece[2], i as usize);
    }

    let result = solve_best_value(&order, &mut sheet);
    println!("{}", result);
}
