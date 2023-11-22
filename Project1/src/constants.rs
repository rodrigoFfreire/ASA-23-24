pub const DIM_X: usize = 0;
pub const DIM_Y: usize = 1;
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
