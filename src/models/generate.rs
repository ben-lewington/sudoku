use super::{board::Board, cells::BoardIndex, BoardOptions, Cell};

impl<const N: usize> Board<N>
where
    [(); N * N * N * N]: Sized,
{
    fn generate() -> Self {
        BoardOptions::new().into()
    }

    fn swap(&mut self, to: BoardIndex<N>, from: BoardIndex<N>) -> &mut Self {
        self
    }
}
