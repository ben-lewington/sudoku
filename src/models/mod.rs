mod board;
pub(crate) use board::Board;
mod cells;
pub(crate) use cells::{board_index, board_value, Cell, CellOptions};
mod generate;

#[macro_export]
macro_rules! board {
    (unfixed) => {{
        Board::<CellOptions<3>, 3>::new()
    }};
    (unfixed: $n:literal) => {
        Board::<CellOptions<$n>, $n>::new()
    };
}

#[macro_export]
macro_rules! val {
    ($v:expr) => {
        board_value::<3>($v).unwrap()
    };
    ($v:expr, $n:literal) => {
        board_value::<$n>($v).unwrap()
    };
}

#[macro_export]
macro_rules! ind {
    ($i:expr, $j:expr) => {
        board_index::<3>($i, $j).unwrap()
    };
    ($i:expr, $j:expr, $n:literal) => {
        board_index::<$n>($i, $j).unwrap()
    };
}
