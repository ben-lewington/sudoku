#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub(crate) mod board_generator_slim;
mod models;
use models::BoardGenerator;

fn main() {
    let mut b = BoardGenerator::<3>::new();
    let _ = b
        .fix(1, 1, 3)
        .fix(1, 2, 1)
        .fix(1, 3, 2)
        .fix(2, 1, 9)
        .fix(2, 2, 6)
        .fix(2, 3, 7)
        .fix(3, 1, 4)
        .fix(3, 2, 8)
        .update_board();

    println!("{}", &b);
    // b.seed_board();
    // println!("{}", b.board);
}
