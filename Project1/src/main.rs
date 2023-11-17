use std::{io, num::ParseIntError, vec};

const SHEET_DIMENSIONS: usize = 2;
const PIECE_ARGUMENTS: usize = 3;
const PIECE_AMOUNT: usize = 1;
const DIM_X: usize = 0;
const DIM_Y: usize = 1;
const ERR_FAILED_PARSE: &str = "Failed Parsing!";


struct Order {
    areas: Vec<u32>,
    prices: Vec<u32>,
    amount: u32,
}

impl Order {
    fn new(amount: u32) -> Self {
        let areas: Vec<u32> = vec![0; amount as usize];
        let prices: Vec<u32> = vec![0; amount as usize];

        Self {
            areas,
            prices,
            amount,
        }
    }

    fn add_piece(&mut self, x: u32, y: u32, price: u32, index: usize) {
        self.areas[index] = x * y;
        self.prices[index] = price;
    }
}


fn solve_best_value(order: &Order, max_area: usize) -> u32 {
    let piece_amount = order.amount as usize;
    let areas = &order.areas;
    let prices = &order.prices;

    let mut max_value = vec![0; (max_area + 1) as usize];

    for w in 0..=max_area {
        for i in 0..piece_amount {
            if areas[i] as usize <= w {
                max_value[w] = max_value[w].max(max_value[w - areas[i] as usize] + prices[i]);
            }
        }
    }

    return max_value[max_area];

    /*for i in 1..=piece_amount {
        for j in 1..=max_x {
            for k in 1..=max_y {
                if dimensions[i][DIM_X] > j.try_into().unwrap() {
                    matrix[i][j][k] = matrix[i - 1][j][k];
                } else {
                    if dimensions[i][DIM_Y] > k.try_into().unwrap() {
                        matrix[i][j][k] = matrix[i - 1][j][k];
                    } else {
                        let mut diff = [0, 0];
                        let piece_x = dimensions[i][DIM_X]; let piece_y = dimensions[i][DIM_Y];

                        if piece_y <= k as u32 || piece_x >= piece_y {
                            diff[1] = piece_y
                        } else {
                            diff[0] = piece_x
                        }

                        matrix[i][j][k] = max!(
                            matrix[i - 1][j][k],
                            matrix[i - 1][j - (diff[0]) as usize][k - (diff[1]) as usize] + prices[i]
                        );
                    }
                }
            }
        }
    }
    
    matrix[piece_amount][max_x][max_y]*/
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

    let mut order: Order = Order::new(piece_amount);
    let max_area = (sheet_dimensions[DIM_X] * sheet_dimensions[DIM_Y]) as usize;

    if piece_amount <= 0 {
        eprintln!("There must be at least 1 piece!");
        return;
    }

    for i in 0..piece_amount {
        let piece = parse_integer_tokens(PIECE_ARGUMENTS).expect(ERR_FAILED_PARSE);
        order.add_piece(piece[0], piece[1], piece[2], i as usize);
    }

    let result = solve_best_value(&order, max_area);
    println!("{}", result);
}
