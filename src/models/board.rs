use super::cells::{board_index, BoardIndex, BoardValues, Cell, CellState};

pub(crate) struct Board<const N: usize>
where
    [(); N * N * N * N]: Sized,
{
    pub(super) board: [Cell<N>; N * N * N * N],
}

pub(crate) struct BoardGenerator<const N: usize>
where
    [(); N * N * N * N]: Sized,
{
    pub board_states: [CellState<N>; N * N * N * N],
}

impl<const N: usize> BoardGenerator<N>
where
    [(); N * N * N * N]: Sized,
{
    pub(crate) fn new() -> Self {
        let board_states = [CellState::new(); N * N * N * N];
        Self { board_states }
    }

    fn from_board_index(index: BoardIndex<N>) -> usize {
        let m = N * N;
        let (r, c) = index;
        (usize::from(r) - 1) * m + (usize::from(c) - 1)
    }

    fn into_board_index(index: usize) -> BoardIndex<N> {
        let m = N * N;
        let i = index / m + 1;
        let j = index + 1 - (i - 1) * m;
        (
            BoardValues::<N>::new(i).unwrap(),
            BoardValues::<N>::new(j).unwrap(),
        )
    }

    pub(crate) fn fix(&mut self, row: usize, col: usize, value: usize) -> &mut Self {
        let m = N * N;
        let c = &mut self[(row, col)];
        *c = CellState::Fixed(Cell::new(value));
        for i in 1..=m {
            let _ = *self[(i, col)].eliminate(value);
            let _ = *self[(row, i)].eliminate(value);
        }
        let block_head = ((row - 1) / 3, (col - 1) / 3);
        for r in 1..=N {
            for c in 1..=N {
                let _ = *self[(block_head.0 * 3 + r, block_head.1 + c)].eliminate(value);
            }
        }
        self
    }

    pub(crate) fn update_board(&mut self) -> &mut Self {
        let m = N * N;
        for r in 1..=m {
            for c in 1..=m {
                self[(r, c)] = self[(r, c)].try_fix();
            }
        }
        self
    }
}

impl<const N: usize> std::ops::Index<(usize, usize)> for BoardGenerator<N>
where
    [(); N * N * N * N]: Sized,
{
    type Output = CellState<N>;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        let ind = board_index::<N>(i, j);
        &self.board_states[Self::from_board_index(ind)]
    }
}

impl<const N: usize> std::ops::IndexMut<(usize, usize)> for BoardGenerator<N>
where
    [(); N * N * N * N]: Sized,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        let ind = board_index::<N>(i, j);
        &mut self.board_states[Self::from_board_index(ind)]
    }
}

impl<const N: usize> std::fmt::Display for BoardGenerator<N>
where
    [(); N * N * N * N]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = (1..=9)
            .map(|i| {
                (1..=9)
                    .map(|j| format!("{}", self[(i, j)]))
                    .collect::<Vec<String>>()
                    .join(" | ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", &s)
    }
}
