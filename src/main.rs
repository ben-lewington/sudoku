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

pub fn main() -> Result<()> {
    const N: usize = 2;
    let _u = ind!(N * N, 4)?;

    let csv: &'static str = "1,2,3,4;3,4,1,2;2,1,4,3;4,3,2,1;";
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
    println!(
        "{:?}\n{}",
        &d.freqs
            .iter()
            .enumerate()
            .map(|(i, _)| (i, i / 3, i % 3))
            .collect::<Vec<(usize, usize, usize)>>(),
        &d.freqs
            .iter()
            .enumerate()
            .flat_map(|(i, v)| {
                dbg!(i, i / (N * N));
                let f = match i / (N * N) {
                    0 => "row",
                    1 => "column",
                    2 => "minor",
                    _ => "_",
                };
                v.iter().enumerate().map(move |(j, v)| {
                    format!("{}-th {} -> {} occurences of {}", i % (N * N), f, *v, j)
                })
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
    (0..N * N * N * N).for_each(|i| {
        let x = Index::<N>::std(i).unwrap();
        dbg!(&x, &x.at(), &x.grid_ref());
    });
    Ok(())
}

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
