use crate::{ind, Index};
pub(crate) mod index;
pub(crate) mod metadata;

#[derive(Debug)]
pub(crate) enum Segment<const N: usize>
where
    [(); N * N]: Sized,
{
    Row(usize),
    Col(usize),
    Minor(usize, usize),
}

impl<const N: usize> Segment<N>
where
    [(); N * N]: Sized,
{
    fn row(index: usize) -> Self {
        Self::Row(index / (N * N))
    }
    fn col(index: usize) -> Self {
        Self::Col(index % (N * N))
    }
    fn minor(index: usize) -> Self {
        Self::Minor((index / (N * N)) / N, (index % (N * N)) / N)
    }
}

#[derive(Debug)]
pub struct Board<const N: usize>
where
    [(); N * N * N * N]: Sized,
{
    pub base: [usize; N * N * N * N],
}

impl<const N: usize> Board<N>
where
    [(); N * N * N * N]: Sized,
    [(); N * N * N * N - 1]: Sized,
    [(); N * N - 1]: Sized,
{
    pub fn new(base: [usize; N * N * N * N]) -> Self {
        Self { base }
    }

    pub(crate) fn get<'a>(&'a self, i: Index<N>) -> &'a usize {
        // Safety: use the ind! macro to construct indexes, this will panic
        // if the required bounds aren't met
        unsafe { self.base.get_unchecked(i.at()) }
    }

    // pub fn set(&mut self, i: Index<N>, v: usize) -> &mut Self {
    //     // Safety: use the ind! macro to construct indexes, this will panic
    //     // if the required bounds aren't met
    //     unsafe { *self.base.get_unchecked_mut(i.at()) = v }
    //     self
    // }

    // pub(crate) fn swap(&mut self, i: Index<N>, j: Index<N>) -> &mut Self {
    //     // Safety: use the ind! macro to construct indexes, this will panic
    //     // if the required bounds aren't met
    //     unsafe { self.base.swap_unchecked(i.at(), j.at()) }
    //     self
    // }

    pub(crate) fn segment(&self, seg_ind: Segment<N>) -> impl Iterator<Item = usize> + '_ {
        (0..N * N)
            .map(move |x| match seg_ind {
                Segment::Row(i) => (N * N) * i + x,
                Segment::Col(j) => (N * N) * x + j,
                Segment::Minor(i, j) => {
                    let (k, l) = (x / N, x % N);
                    (N * N) * (i + k) + (j + l)
                }
            })
            .map(|v| *self.get(ind!(N, v).expect("index is in bounds by construction.")))
    }

    pub fn is_valid_layout(&self) -> bool {
        (0..N * N).all(|i| {
            self.segment(Segment::<N>::row(i)).sum::<usize>() == (N * N) * (N * N + 1) / 2
                && self.segment(Segment::<N>::col(i)).sum::<usize>() == (N * N) * (N * N + 1) / 2
                && self.segment(Segment::<N>::minor(i)).sum::<usize>() == (N * N) * (N * N + 1) / 2
        })
    }
}
