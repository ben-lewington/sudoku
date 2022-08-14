use crate::utils::array::{new, new_checked};
use anyhow::Result;

use super::{Board, Segment};

#[derive(Debug)]
pub(crate) struct Metadata<const N: usize>
where
    [(); 3 * N * N]: Sized,
    [(); N * N + 1]: Sized,
{
    pub(crate) cnts: [usize; 3 * N * N],
    pub(crate) freqs: [[usize; N * N + 1]; 3 * N * N],
}

impl<const N: usize> Metadata<N>
where
    [(); N * N * N * N]: Sized,
    [(); N * N * N * N - 1]: Sized,
    [(); 3 * N * N]: Sized,
    [(); N * N + 1]: Sized,
    [(); N * N - 1]: Sized,
{
    pub fn new(b: &Board<N>) -> Result<Self> {
        let v = Self {
            cnts: new((0..N * N)
                .map(|i| b.segment(Segment::row(i)).sum::<usize>())
                .chain((0..N * N).map(|i| b.segment(Segment::col(i)).sum::<usize>()))
                .chain((0..N * N).map(|i| b.segment(Segment::minor(i)).sum::<usize>())))?,
            freqs: new::<{ 3 * N * N }, [usize; N * N + 1]>(
                (0..N * N)
                    .map(|i| {
                        new::<{ N * N + 1 }, usize>(
                            (0..=N * N)
                                .map(|v| b.segment(Segment::row(i)).filter(|&w| w == v).count()),
                        )
                        .unwrap()
                    })
                    .chain((0..N * N).map(|i| {
                        new::<{ N * N + 1 }, usize>(
                            (0..=N * N)
                                .map(|v| b.segment(Segment::col(i)).filter(|&w| w == v).count()),
                        )
                        .unwrap()
                    }))
                    .chain((0..N * N).map(|i| {
                        new::<{ N * N + 1 }, usize>(
                            (0..=N * N)
                                .map(|v| b.segment(Segment::minor(i)).filter(|&w| w == v).count()),
                        )
                        .unwrap()
                    })),
            )?,
        };
        Ok(v)
    }
}
