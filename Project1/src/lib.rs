use std::usize;

pub const DIM_X: usize = 0;
pub const DIM_Y: usize = 1;
pub const PRICE: usize = 2;
pub const ERR_FAILED_PARSE: &str = "Failed Parsing!";
pub const PARSE_ARGS_SHEET_SIZE: usize = 2;
pub const ARGS_PIECE: usize = 3;
pub const PARSE_ARGS_PIECE_AMOUNT: usize = 1;


macro_rules! check_fit {
    ($piece_x:expr, $piece_y:expr, $sheet_x:expr, $sheet_y:expr) => {
        $piece_x <= $sheet_x && $piece_y <= $sheet_y
    };

    (rotated, $piece_x:expr, $piece_y:expr, $sheet_x:expr, $sheet_y:expr) => {
        $piece_x <= $sheet_y && $piece_y <= $sheet_x
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
    fn get(&mut self, x: usize, y: usize) -> usize {
        self.table[y * self.matrix_x + x]
    }
    fn set(&mut self, x: usize, y: usize, item: usize) {
        self.table[y * self.matrix_x + x] = item;
    }
}


pub struct Piece {
    x: usize,
    y: usize,
    price: usize
}

impl Piece {
    pub fn new(piece_info: &Vec<usize>) -> Self {
        let mut piece_x = piece_info[DIM_X];
        let mut piece_y = piece_info[DIM_Y];

        if piece_y > piece_x {
            (piece_x, piece_y) = (piece_y, piece_x);
        }

        Self { 
            x: piece_x,
            y: piece_y,
            price: piece_info[PRICE]
        }
    }
}

fn calculate_best_value(
    piece_x: usize,
    piece_y: usize,
    matrix: &mut Matrix,
    x: usize,
    y: usize,
) -> usize {
    let v_cut_value = matrix.get(piece_x, y) + matrix.get(x - piece_x, y);
    let h_cut_value = matrix.get(x, piece_y) + matrix.get(x, y - piece_y);

    return h_cut_value.max(v_cut_value);
}

fn get_minimum_piece(order: &[Piece], sheet_x: usize) -> (usize, usize, usize) {
    let mut min_x = order[0].x;
    let mut min_y = order[0].y;
    let mut min_index = order[0].y * sheet_x + order[0].x;
    let mut _i = 0;

    for piece in order.iter() {
        _i = piece.y * sheet_x + piece.x;
        if _i < min_index {
            min_index = _i;
            min_x = piece.x;
            min_y = piece.y;
        }
    }
    return (min_x, min_y, min_index);
}

pub fn solve_best_value(order: &[Piece], amount: usize, sheet_x: usize, sheet_y: usize) -> usize {
    if amount <= 0 {
        return 0;
    }
    let mut new_sheet_x = sheet_x;
    let mut new_sheet_y = sheet_y;

    if sheet_y > sheet_x {
        (new_sheet_x, new_sheet_y) = (new_sheet_y, new_sheet_x);
    }
    let mut max_value = Matrix::new(new_sheet_x + 1, new_sheet_y + 1);
    
    let min_piece = get_minimum_piece(order, new_sheet_x + 1);
    let mut y: usize = min_piece.1;
    let mut x: usize = min_piece.0;
    let mut _t_i = min_piece.2 - 1;

    while y <= new_sheet_y {
        while x <= new_sheet_x {
            let mut best_value = max_value.table[_t_i];
            for piece in order.iter() {
                let (piece_x, piece_y, piece_price) = (piece.x, piece.y, piece.price);
                if !check_fit!(piece_x, piece_y, x, y) {
                    continue;
                }
                if piece_x == x && piece_y == y {
                    best_value = best_value.max(piece_price);
                } else {
                    best_value = best_value.max(calculate_best_value(
                        piece_x, piece_y, &mut max_value, x, y
                    ));
                    if check_fit!(rotated, piece_x, piece_y, x, y) {
                        best_value = best_value.max(calculate_best_value(
                            piece_y, piece_x, &mut max_value, x, y
                        ));
                    }
                }
            }
            _t_i += 1;
            max_value.table[_t_i] = best_value;
            if x > y && x <= new_sheet_y {
                max_value.set(y, x, best_value);
            }
            x += 1;
        }
        y += 1;
        x = y;
        _t_i += y;
    }
    return max_value.get(new_sheet_x, new_sheet_y);
}