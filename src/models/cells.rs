use bounded_integer::BoundedUsize;

pub(crate) type BoardValues<const N: usize> = BoundedUsize<1, { N * N }>;
pub(crate) type BoardIndex<const N: usize> = (BoardValues<N>, BoardValues<N>);

pub(crate) fn board_index<const N: usize>(i: usize, j: usize) -> BoardIndex<N>
where
    [(); N * N]: Sized,
{
    (
        BoardValues::<N>::new(i).unwrap(),
        BoardValues::<N>::new(j).unwrap(),
    )
}

#[derive(Clone, Copy, Debug)]
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
    pub(crate) fn new(value: usize) -> Self {
        Self {
            value: BoardValues::<N>::new(value).unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CellOptions<const N: usize>
where
    [(); N * N]: Sized,
{
    pub(crate) options: [bool; N * N],
}

impl<const N: usize> CellOptions<N>
where
    [(); N * N]: Sized,
{
    pub(crate) fn new() -> Self {
        let options = [false; N * N];
        Self { options }
    }

    pub(crate) fn eliminate(&mut self, value: usize) -> &mut Self {
        let value = BoardValues::<N>::new(value).unwrap();
        self.options[usize::from(value) - 1] = true;
        self
    }

    pub(crate) fn count_options(&self) -> usize {
        let mut c = 0;
        for is_option in self.options {
            if !is_option {
                c += 1;
            }
        }
        c
    }

    pub(crate) fn option_values(&self) -> Vec<usize> {
        let mut v = Vec::new();
        for (i, o) in self.options.iter().enumerate() {
            if !*o {
                v.push(i + 1);
            }
        }
        v
    }

    pub(crate) fn implies_fixed(&self) -> bool {
        let mut c = 0;
        for o in self.options {
            if !o {
                c += 1;
            }
            if c > 1 {
                return false;
            }
        }
        if c == 1 {
            true
        } else {
            false
        }
    }

    pub(crate) fn first_option(&self) -> Option<usize> {
        for (i, o) in self.options.iter().enumerate() {
            if !o {
                return Some(i + 1);
            }
        }
        None
    }
}

impl<const N: usize> TryInto<Cell<N>> for CellOptions<N>
where
    [(); N * N]: Sized,
{
    type Error = CellOptions<N>;

    fn try_into(self) -> Result<Cell<N>, Self::Error> {
        if self.implies_fixed() {
            Ok(Cell::new(self.first_option().unwrap()))
        } else {
            Err(self)
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

#[derive(Clone, Copy, Debug)]
pub(crate) enum CellState<const N: usize>
where
    [(); N * N]: Sized,
{
    Unfixed(CellOptions<N>),
    Fixed(Cell<N>),
}

impl<const N: usize> CellState<N>
where
    [(); N * N]: Sized,
{
    pub(crate) fn new() -> Self {
        let c = CellOptions::new();
        Self::Unfixed(c)
    }

    pub(crate) fn try_fix(self) -> Self {
        match self {
            Self::Unfixed(co) => {
                let c = co.try_into();
                match c {
                    Ok(c) => {
                        return Self::Fixed(c);
                    }
                    Err(co) => return Self::Unfixed(co),
                }
            }
            _ => self,
        }
    }

    pub(crate) fn eliminate(&mut self, value: usize) -> &mut Self {
        match self {
            CellState::Unfixed(c) => {
                let _ = *c.eliminate(value);
                self
            }
            CellState::Fixed(_) => self,
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
