use bounded_integer::BoundedUsize;

use crate::{SudokuErr, SudokuResult};

pub(crate) type BoardValues<const N: usize> = BoundedUsize<1, { N * N }>;
pub(crate) type BoardIndex<const N: usize> = (BoardValues<N>, BoardValues<N>);

#[derive(Debug)]
pub(crate) enum ValueErr<const N: usize> {
    OutofBounds { value: Vec<usize> },
}

#[derive(Debug)]
pub(crate) enum CellErr {
    NoValidStates,
    MultipleValidStates,
}

pub(crate) fn board_value<const N: usize>(v: usize) -> SudokuResult<BoardValues<N>, N>
where
    [(); N * N]: Sized,
{
    if let Some(v) = BoardValues::<N>::new(v) {
        Ok(v)
    } else {
        Err(SudokuErr::InvalidBoardValue(v))
    }
}

pub(crate) fn board_index<const N: usize>(i: usize, j: usize) -> SudokuResult<BoardIndex<N>, N>
where
    [(); N * N]: Sized,
{
    match (board_value::<N>(i), board_value::<N>(j)) {
        (Ok(i), Ok(j)) => Ok((i, j)),
        _ => Err(SudokuErr::InvalidBoardIndex(i, j)),
    }
}

pub(crate) fn unwrap<const N: usize>(b: BoardIndex<N>) -> (usize, usize)
where
    [(); N * N]: Sized,
{
    let (r, c) = b;
    (usize::from(r), usize::from(c))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Cell<const N: usize>
where
    [(); N * N]: Sized,
{
    pub(crate) value: BoardValues<N>,
}

impl<const N: usize> Cell<N>
where
    [(); N * N]: Sized,
{
    pub(crate) fn new(value: BoardValues<N>) -> SudokuResult<Self, N> {
        Ok(Self { value })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CellOptions<const N: usize>
where
    [(); N * N]: Sized,
{
    pub(crate) options: [bool; N * N],
}

impl<const N: usize> std::ops::Index<BoardValues<N>> for CellOptions<N>
where
    [(); N * N]: Sized,
{
    type Output = bool;

    fn index(&self, index: BoardValues<N>) -> &Self::Output {
        &self.options[usize::from(index) - 1]
    }
}

impl<const N: usize> std::ops::IndexMut<BoardValues<N>> for CellOptions<N>
where
    [(); N * N]: Sized,
{
    fn index_mut(&mut self, index: BoardValues<N>) -> &mut Self::Output {
        &mut self[index]
    }
}

impl<const N: usize> CellOptions<N>
where
    [(); N * N]: Sized,
{
    fn new(start: bool) -> Self {
        let options = [start; N * N];
        Self { options }
    }

    pub(crate) fn with_value(v: BoardValues<N>) -> SudokuResult<Self, N> {
        let mut co = Self::new(false);
        *&mut co[v] = true;
        Ok(co)
    }

    pub(crate) fn eliminate(&mut self, v: BoardValues<N>) -> SudokuResult<&mut Self, N> {
        *&mut self[v] = false;
        Ok(self)
    }

    pub(crate) fn count_options(&self) -> usize {
        self.options.iter().filter(|&o| *o).count()
    }

    pub(crate) fn option_values(&self) -> Vec<usize> {
        self.options
            .iter()
            .enumerate()
            .filter_map(|(i, o)| if *o { Some(i + 1) } else { None })
            .collect()
    }

    pub(crate) fn implies_fixed(&self) -> bool {
        self.count_options() == 1
    }

    pub(crate) fn first_option(&self) -> SudokuResult<BoardValues<N>, N> {
        if let Some(v) =
            self.options
                .iter()
                .enumerate()
                .find_map(|(i, o)| if *o { Some(i + 1) } else { None })
        {
            Ok(board_value(v)?)
        } else {
            Err(SudokuErr::NoCellOptionsForCast)
        }
    }
}

impl<const N: usize> TryInto<Cell<N>> for CellOptions<N>
where
    [(); N * N]: Sized,
{
    type Error = SudokuErr<N>;

    fn try_into(self) -> Result<Cell<N>, Self::Error> {
        if self.implies_fixed() {
            Ok(Cell::new(self.first_option()?)?)
        } else {
            let e = match self.count_options() {
                0 => SudokuErr::NoCellOptionsForCast,
                _ => SudokuErr::MultipleCellOptionsForCast(self),
            };
            Err(e)
        }
    }
}

impl<const N: usize> std::fmt::Display for CellOptions<N>
where
    [(); N * N]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .option_values()
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "[{}]", s)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CellState<const N: usize>
where
    [(); N * N]: Sized,
{
    Unfixed(CellOptions<N>),
    Fixed(Cell<N>),
}

// Arr<T, const N: usize> = [T; N];

// stack
// -> (&0x13fefa, N + 1)

impl<const N: usize> CellState<N>
where
    [(); N * N]: Sized,
{
    pub(crate) fn new() -> Self {
        Self::Unfixed(CellOptions::new(true))
    }

    pub(crate) fn try_fix(self) -> SudokuResult<Self, N> {
        match self {
            Self::Unfixed(co) => match co.try_into() {
                Ok(c) => Ok(Self::Fixed(c)),
                Err(e) => Err(e),
            },
            _ => Ok(self),
        }
    }

    pub(crate) fn eliminate(&mut self, v: BoardValues<N>) -> SudokuResult<&mut Self, N> {
        match self {
            CellState::Unfixed(c) => {
                let _ = *c.eliminate(v)?;
                Ok(self)
            }
            CellState::Fixed(_) => Ok(self),
        }
    }
}

impl<const N: usize> TryInto<Cell<N>> for CellState<N>
where
    [(); N * N]: Sized,
{
    type Error = SudokuErr<N>;
    fn try_into(self) -> Result<Cell<N>, Self::Error> {
        match self.try_fix() {
            Ok(Self::Fixed(c)) => Ok(c),
            Ok(Self::Unfixed(co)) => Err(SudokuErr::MultipleCellOptionsForCast(co)),
            Err(e) => Err(e),
        }
    }
}

impl<const N: usize> std::fmt::Display for CellState<N>
where
    [(); N * N]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = match self {
            Self::Unfixed(co) => {
                format!("<{}>", co.count_options())
            }
            Self::Fixed(c) => {
                format!("={}=", c.value)
            }
        };
        write!(f, "{}", s)
    }
}
