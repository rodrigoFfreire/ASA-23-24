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

macro_rules! matrix_get {
    ($x:expr, $y:expr, $m:expr) => {
        $m.table[$y as usize * $m.matrix_x as usize + $x as usize]
    }
}

macro_rules! matrix_set {
    ($x:expr, $y:expr, $m:expr, $value:expr) => {
        $m.table[$y as usize * $m.matrix_x as usize + $x as usize] = $value;
    };
}

struct Matrix {
    matrix_x: u32,
    table: Vec<u32>,
    // matrix_y doesnt need to be defined since its not used in any operation
}

impl Matrix {
    fn new(matrix_x: u32, matrix_y: u32) -> Self {
        Self {
            matrix_x,
            table: vec![0; matrix_x as usize * matrix_y as usize],
        }
    }
}

pub struct Piece {
    x: u32,
    y: u32,
    price: u32
}

impl Piece {
    pub fn new(piece_info: &Vec<u32>) -> Self {
        Self { 
            x: piece_info[DIM_X],
            y: piece_info[DIM_Y],
            price: piece_info[PRICE]
        }
    }
}

fn piece_fits(&piece_x: &u32, &piece_y: &u32, sheet_x: u32, sheet_y: u32) -> PieceFit {
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
    &piece_x: &u32,
    &piece_y: &u32,
    &piece_price: &u32,
    matrix: &mut Matrix,
    x: u32,
    y: u32,
    max_sheet_x: u32,
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

pub fn solve_best_value(order: &Vec<Piece>, amount: u32, sheet_x: u32, sheet_y: u32) -> u32 {
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