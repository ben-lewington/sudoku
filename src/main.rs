#![allow(incomplete_features)]
#![allow(unused_imports)]
#![feature(generic_const_exprs)]
#![feature(slice_swap_unchecked)]

use std::str::FromStr;

use anyhow::{Context, Result};
use bounded_integer::BoundedUsize;

pub(crate) mod board;
pub(crate) mod utils;

use crate::utils::array::new;
use board::{
    index::{index_bound, Index},
    metadata::Metadata,
    Board,
};

pub(crate) fn parse_str_into_grid<const N: usize, T: FromStr<Err = impl std::fmt::Debug>>(
    input: &'static str,
) -> impl Iterator<Item = T> {
    input
        .split(';')
        .filter(|r| r.trim().len() != 0)
        .flat_map(|r| {
            r.split(',')
                .filter(|v| v.trim().len() != 0)
                .map(|v| v.trim().parse().unwrap())
        })
}

pub fn main() -> Result<()> {
    const N: usize = 2;
    let _u = ind!(N * N, 4)?;

    let csv = "1,2,3,4;3,4,1,2;2,1,4,3;4,3,2,1;";
    let v = new::<{ N * N * N * N }, usize>(
        parse_str_into_grid::<{ N * N * N * N }, usize>(csv)
            .map(|v| index_bound::<{ N * N }>(v).unwrap()),
    );

    let b = Board::<N>::new(v?);
    println!("{:?} {}", b, b.is_valid_layout());

    // let d =
    let d = Metadata::new(&b)?;
    println!("{:?}", &d);
    println!("{:?}", &d.cnts.iter().map(|&v| v).collect::<Vec<usize>>());
    // println!(
    //     "{:?}",
    //     &d.freqs
    //         .iter()
    //         .enumerate()
    //         .flat_map(|&(i, v)| {
    //             v.iter().enumerate().map(move |&(j, v)| (i))
    //         }
    //         .collect::<Vec<(usize, usize, usize)>>()
    // );
    Ok(())
}
