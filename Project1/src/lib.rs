use std::usize;

pub const DIM_X: usize = 0;
pub const DIM_Y: usize = 1;
pub const PRICE: usize = 2;
pub const ERR_FAILED_PARSE: &str = "Failed Parsing!";
pub const PARSE_ARGS_SHEET_SIZE: usize = 2;
pub const ARGS_PIECE: usize = 3;
pub const PARSE_ARGS_PIECE_AMOUNT: usize = 1;

pub enum PieceFit {
    NoFit,
    OnlyFitsOriginal,
    OnlyFitsRotated,
    FitsAll,
}

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
        Self { 
            x: piece_info[DIM_X],
            y: piece_info[DIM_Y],
            price: piece_info[PRICE]
        }
    }
}


fn piece_fits(piece_x: usize, piece_y: usize, sheet_x: usize, sheet_y: usize) -> PieceFit {
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
    piece_x: usize,
    piece_y: usize,
    matrix: &mut Matrix,
    x: usize,
    y: usize,
    max_sheet_x: usize,
) {
    let mut _best_value = 0;
    let mut h_cut_value = 0;
    let mut v_cut_value = 0;

    if x > piece_x {
        v_cut_value = matrix.get(piece_x, y) + matrix.get(x - piece_x, y);
    }
    if y > piece_y {
        h_cut_value = matrix.get(x, piece_y) + matrix.get(x, y - piece_y);
    }

    _best_value = matrix.get(x, y).max(h_cut_value.max(v_cut_value));
    matrix.set(x, y, _best_value);
    if y > x && y <= max_sheet_x {
        matrix.set(y, x, _best_value);
    }
}

pub fn solve_best_value(order: &Vec<Piece>, amount: usize, sheet_x: usize, sheet_y: usize) -> usize {
    if amount <= 0 {
        return 0;
    }

    let mut new_sheet_x = sheet_x;
    let mut new_sheet_y = sheet_y;

    if sheet_x > sheet_y {
        (new_sheet_x, new_sheet_y) = (new_sheet_y, new_sheet_x);
    }
    let mut max_value = Matrix::new(new_sheet_x + 1, new_sheet_y + 1);

    let mut _piece_x = 0;
    let mut _piece_y = 0;
    let mut _piece_price = 0;
    let mut _best_value = 0;
    let mut _fits = PieceFit::NoFit;

    for x in 1..=new_sheet_x {
        for y in x..=new_sheet_y {
            for piece in order.iter() {
                _piece_x = piece.x;
                _piece_y = piece.y;
                _piece_price = piece.price;

                if (_piece_x == x && _piece_y == y) || (_piece_x == y && _piece_y == x) {
                    _best_value = max_value.get(x, y).max(_piece_price);
                    max_value.set(x, y, _best_value);
                    if y > x && y <= new_sheet_x {
                        max_value.set(y, x, _best_value);
                    }
                    continue;
                }     

                _fits = piece_fits(_piece_x, _piece_y, x, y);

                match _fits {
                    PieceFit::NoFit => continue,
                    PieceFit::OnlyFitsOriginal => calculate_best_value(
                        _piece_x, _piece_y, &mut max_value, x, y, new_sheet_x,
                    ),
                    PieceFit::OnlyFitsRotated => calculate_best_value(
                        _piece_y, _piece_x, &mut max_value, x, y, new_sheet_x,
                    ),
                    PieceFit::FitsAll => {
                        calculate_best_value(
                            _piece_x, _piece_y, &mut max_value, x, y, new_sheet_x
                        );
                        calculate_best_value(
                            _piece_y, _piece_x, &mut max_value, x, y, new_sheet_x
                        );
                    }
                }
            }
        }
    }
    return max_value.get(new_sheet_x, new_sheet_y);
}