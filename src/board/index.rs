use anyhow::{Context, Result};
use bounded_integer::BoundedUsize;

#[macro_export]
macro_rules! ind {
    ($N:expr, $ind:expr) => {
        Index::<{ $N }>::std($ind)
    };
    ($N:expr, $r_ind:expr, $c_ind: expr) => {
        Index::<{ $N }>::rc($r_ind, $c_ind)
    };
}

pub(crate) fn index_bound<const N: usize>(v: usize) -> Result<usize> {
    let v = BoundedUsize::<0, N>::new(v)
        .with_context(|| format!("number {} is not between 0 and {}", v, N))?;
    Ok(usize::from(v))
}

#[derive(Debug)]
pub(crate) enum Index<const N: usize> {
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
