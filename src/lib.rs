use crate::random;
use bounded_integer::BoundedUsize;

pub(crate) type BoxValue<const N: usize> = BoundedUsize<1, N>;
pub(crate) type Value<const N: usize> = BoundedUsize<1, { N * N }>;
pub(crate) type BoxIndex<const N: usize> = (BoxValue<N>, BoxValue<N>);
pub(crate) type Index<const N: usize> = (Value<N>, Value<N>);
pub(crate) type Cell<const N: usize> = Option<Value<N>>;

pub(crate) fn new_value<const N: usize>(v: usize) -> Value<N>
where
    [(); N * N]: Sized,
{
    Value::<N>::new(v).unwrap()
}

pub(crate) fn new_index<const N: usize>(r: usize, c: usize) -> Index<N>
where
    [(); N * N]: Sized,
{
    (Value::<N>::new(r).unwrap(), Value::<N>::new(c).unwrap())
}

pub(crate) fn destructure<const N: usize>(index: Index<N>) -> (usize, usize)
where
    [(); N * N]: Sized,
{
    let (r, c) = index;
    (usize::from(r) - 1, usize::from(c) - 1)
}

pub(crate) fn into_index<const N: usize>(board_index: usize) -> Index<N>
where
    [(); N * N]: Sized,
{
    new_index::<N>(board_index / (N * N) + 1, board_index % (N * N) + 1)
}

pub(crate) struct Board<const N: usize>
where
    [(); N * N * N * N]: Sized,
{
    values: [Cell<N>; N * N * N * N],
}

impl<const N: usize> std::ops::Index<Index<N>> for Board<N>
where
    [(); N * N * N * N]: Sized,
{
    type Output = Option<Value<N>>;

    fn index(&self, index: Index<N>) -> &Self::Output {
        let (r, c) = destructure::<N>(index);
        &self.values[(N * N) * r + c]
    }
}

impl<const N: usize> std::ops::IndexMut<Index<N>> for Board<N>
where
    [(); N * N * N * N]: Sized,
{
    fn index_mut(&mut self, index: Index<N>) -> &mut Self::Output {
        let (r, c) = destructure::<N>(index);
        &mut self.values[(N * N) * r + c]
    }
}

impl<const N: usize> std::fmt::Display for Board<N>
where
    [(); N * N * N * N]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl<const N: usize> Board<N>
where
    [(); N * N * N * N]: Sized,
{
    pub(crate) fn new() -> Self {
        Self {
            values: [None; N * N * N * N],
        }
    }

    pub(crate) fn clear(&mut self) -> &mut Self {
        (1..=N * N).for_each(|r| {
            (1..=N * N).for_each(|c| {
                let ind = new_index::<N>(r, c);
                self[ind] = None;
            })
        });
        self
    }

    pub(crate) fn display(&self) -> String {
        self.values
            .iter()
            .map(|&v| match v {
                Some(v) => v.to_string(),
                None => ".".to_string(),
            })
            .collect::<Vec<String>>()
            .join("")
    }

    pub(crate) fn next_unassigned(&self) -> Option<Index<N>> {
        self.values.iter().enumerate().find_map(|(i, &v)| {
            if v.is_none() {
                Some(into_index::<N>(i))
            } else {
                None
            }
        })
    }

    pub(crate) fn value_in_row(&self, i: Index<N>, num: Value<N>) -> bool {
        let (r, _) = destructure::<N>(i);
        (1..=N * N).any(|c| {
            let ind = new_index::<N>(r + 1, c);
            self[ind] == Some(num)
        })
    }

    pub(crate) fn value_in_col(&self, i: Index<N>, num: Value<N>) -> bool {
        let (_, c) = destructure::<N>(i);
        (1..=N * N).any(|r| {
            let ind = new_index::<N>(r, c + 1);
            self[ind] == Some(num)
        })
    }

    pub(crate) fn value_in_block(&self, i: Index<N>, num: Value<N>) -> bool {
        let (r, c) = destructure::<N>(i);
        (1..=N).any(|i| {
            (1..=N).any(|j| {
                let ind = new_index::<N>((r + 1) / N + i, (c + 1) / N + j);
                self[ind] == Some(num)
            })
        })
    }

    pub(crate) fn valid_value_placement(&self, i: Index<N>, num: Value<N>) -> bool {
        !(self.value_in_row(i, num) && self.value_in_col(i, num) && self.value_in_block(i, num))
    }

    pub(crate) fn seed_box(&mut self, b_i: BoxIndex<N>) -> &mut Self {
        let (b_i, b_j) = b_i;
        let (b_i, b_j) = (usize::from(b_i) - 1, usize::from(b_j) - 1);
        random::shuffle_range::<N>()
            .iter()
            .enumerate()
            .for_each(|(n, &r)| {
                let (i, j) = ((n / N), (n % N));
                let ind = new_index::<N>((b_i * N) + i + 1, (b_j * N) + j + 1);
                (*self)[ind] = Some(new_value::<N>(r));
            });
        self
    }

    pub(crate) fn generate_grid(&mut self) -> &mut Self {
        (1..=N).for_each(|i| {
            let i = BoxValue::<N>::new(i).unwrap();
            let i = (i, i);
            let _ = self.seed_box(i);
        });
        self.solve_grid()
    }

    pub(crate) fn recursive_backtrace_solver(&mut self) -> bool {
        if let Some(index) = self.next_unassigned() {
            for v in 1..=N * N {
                let v = Value::<N>::new(v).unwrap();
                if self.valid_value_placement(index, v) {
                    (*self)[index] = Some(v);
                    if self.recursive_backtrace_solver() {
                        return true;
                    }
                    (*self)[index] = None;
                    continue;
                }
            }
            return false;
        } else {
            return true;
        }
    }

    pub(crate) fn solve_grid(&mut self) -> &mut Self {
        let _ = self.recursive_backtrace_solver();
        self
    }

    pub(crate) fn is_valid_grid(&self) -> Option<bool> {
        if let Some(_) = self.next_unassigned() {
            None
        } else {
            Some(self.values.iter().enumerate().all(|(i, &v)| {
                let i = into_index::<N>(i);
                self.valid_value_placement(i, v.unwrap())
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub(crate) fn test_index_conversion() {
        assert_eq!(destructure::<3>(into_index::<3>(0)), (0, 0));
        assert_eq!(destructure::<3>(into_index::<3>(9)), (1, 0));
        assert_eq!(destructure::<3>(into_index::<3>(80)), (8, 8));
        assert_eq!(destructure::<2>(into_index::<2>(15)), (3, 3));
    }

    #[test]
    pub(crate) fn test_board_display() {
        assert_eq!(Board::<1>::new().display(), ".");
        assert_eq!(Board::<2>::new().display(), "................");
        let mut b = Board::<3>::new();
        (1..=9).for_each(|i| {
            let ind = new_index::<3>(i, 1);
            b[ind] = Some(new_value::<3>(i));
        });
        println!("{}", b);
        assert_eq!(
            b.display(),
            concat!(
                "1........",
                "2........",
                "3........",
                "4........",
                "5........",
                "6........",
                "7........",
                "8........",
                "9........",
            )
        );
        assert_eq!(b.next_unassigned(), Some(new_index::<3>(1, 2)));
        assert_eq!(b.is_valid_grid(), None);
    }

    #[test]
    pub(crate) fn test_board_generator() {
        let mut b = Board::<3>::new();
        let _ = b.generate_grid();
        println!("{}", b);
    }
}
