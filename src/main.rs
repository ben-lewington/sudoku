#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(slice_swap_unchecked)]

use std::str::FromStr;

use anyhow::{Context, Result};
use bounded_integer::BoundedUsize;

pub(crate) mod board;
pub(crate) mod utils;
use board::{diagnostics::Diagnostics, Board};
use utils::new_arr;

#[macro_export]
macro_rules! ind {
    ($N:expr, $ind:expr) => {
        Index::<{ $N }>::std($ind).unwrap()
    };
    ($N:expr, $r_ind:expr, $c_ind: expr) => {
        Index::<{ $N }>::rc($r_ind, $c_ind).unwrap()
    };
}

#[derive(Debug)]
pub enum Index<const N: usize> {
    Std(usize),
    RC(usize, usize),
}

impl<const N: usize> Index<N>
where
    [(); N * N * N * N - 1]: Sized,
    [(); N * N - 1]: Sized,
{
    pub fn std(i: usize) -> Result<Self> {
        Ok(Self::Std(index_bound::<{ N * N * N * N - 1 }>(i)?))
    }

    pub fn rc(r: usize, c: usize) -> Result<Self> {
        Ok(Self::RC(
            index_bound::<{ N * N - 1 }>(r)?,
            index_bound::<{ N * N - 1 }>(c)?,
        ))
    }

    pub fn at(&self) -> usize {
        match self {
            Self::Std(i) => *i,
            Self::RC(r, c) => (N * N) * r + c,
        }
    }
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

pub fn index_bound<const N: usize>(v: usize) -> Result<usize> {
    let v = BoundedUsize::<0, N>::new(v)
        .with_context(|| format!("number {} is not between 0 and {}", v, N))?;
    Ok(usize::from(v))
}

pub fn main() -> Result<()> {
    const N: usize = 2;
    let _u = ind!(N * N, 4);

    let csv = "1,2,3,4;3,4,1,2;2,1,4,3;4,3,2,1;";
    let v = new_arr::<{ N * N * N * N }, usize>(
        parse_str_into_grid::<{ N * N * N * N }, usize>(csv)
            .map(|v| index_bound::<{ N * N }>(v).unwrap()),
    );

    let b = Board::<N>::new(v?);
    println!("{:?} {}", b, b.is_valid_layout());

    // let d =
    // let d = Diagnostics::new(&b);

    Ok(())
}
