use crate::{random::shuffle_range, SudokuResult};

use super::cells::{
    board_index, board_value, unwrap, BoardIndex, BoardValues, Cell, CellOptions, CellState,
};

pub(crate) struct Board<C, const N: usize>
where
    C: CellState<N>,
    [(); N * N * N * N]: Sized,
{
    cells: [C; N * N * N * N],
}

impl<C: CellState<N>, const N: usize> Board<C, N> where [(); N * N * N * N]: Sized {}

// pub(crate) struct BoardOptions<const N: usize>
// where
//     [(); N * N * N * N]: Sized,
// {
//     pub board_states: [CellOptions<N>; N * N * N * N],
// }

// impl<const N: usize> BoardOptions<N>
// where
//     [(); N * N * N * N]: Sized,
// {
//     pub(crate) fn new() -> Self {
//         let board_states = [CellOptions::new(true); N * N * N * N];
//         Self { board_states }
//     }

//     fn from_board_index(index: BoardIndex<N>) -> usize {
//         let m = N * N;
//         let (r, c) = index;
//         (usize::from(r) - 1) * m + (usize::from(c) - 1)
//     }

//     fn into_board_index(index: usize) -> SudokuResult<BoardIndex<N>, N> {
//         let m = N * N;
//         board_index::<N>((index / m) + 1, (index % m) as usize + 1)
//     }

//     fn eliminate_at(&mut self, index: BoardIndex<N>, value: BoardValues<N>) -> &mut Self {
//         let m = N * N;
//         let (r, c) = unwrap::<N>(index);

//         for i in 1..=m {
//             let _ = *self[board_index::<N>(i, c).unwrap()].eliminate(value);
//             let _ = *self[board_index::<N>(r, i).unwrap()].eliminate(value);
//         }
//         let block_head = ((r - 1) / N, (c - 1) / N);
//         for r in 1..=N {
//             for c in 1..=N {
//                 let _ = *self[board_index::<N>(block_head.0 * 3 + r, block_head.1 + c).unwrap()]
//                     .eliminate(value);
//             }
//         }
//         self
//     }

//     pub(crate) fn fix(&mut self, index: BoardIndex<N>, v: BoardValues<N>) -> &mut Self {
//         let _ = self.eliminate_at(index, v);
//         *&mut self[index] = CellOptions::with_value(v);
//         self
//     }

//     pub(crate) fn update_board(&mut self) -> &mut Self {
//         let m = N * N;
//         for r in 1..=m {
//             for c in 1..=m {
//                 let i = board_index::<N>(r, c).unwrap();
//                 if self[i].count_options() == 1 {
//                     let _ = self.fix(i, self[i].first_option().unwrap());
//                 }
//             }
//         }
//         self
//     }
// }

// impl<const N: usize> std::ops::Index<BoardIndex<N>> for BoardOptions<N>
// where
//     [(); N * N * N * N]: Sized,
// {
//     type Output = CellOptions<N>;
//     fn index(&self, index: BoardIndex<N>) -> &Self::Output {
//         &self.board_states[Self::from_board_index(index)]
//     }
// }

// impl<const N: usize> std::ops::IndexMut<BoardIndex<N>> for BoardOptions<N>
// where
//     [(); N * N * N * N]: Sized,
// {
//     fn index_mut(&mut self, index: BoardIndex<N>) -> &mut Self::Output {
//         &mut self.board_states[Self::from_board_index(index)]
//     }
// }

// impl<const N: usize> std::fmt::Display for BoardOptions<N>
// where
//     [(); N * N * N * N]: Sized,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let s = (1..=N * N)
//             .map(|i| {
//                 (1..=N * N)
//                     .map(|j| {
//                         let i = board_index::<N>(i, j).unwrap();
//                         format!("{}", self[i])
//                     })
//                     .collect::<Vec<String>>()
//                     .join(" | ")
//             })
//             .collect::<Vec<String>>()
//             .join("\n");
//         write!(f, "{}", &s)
//     }
// }

// // pub(crate) struct Board<const N: usize>
// // where
// //     [(); N * N * N * N]: Sized,
// // {
// //     pub(super) board: [Cell<N>; N * N * N * N],
// // }

// // impl<const N: usize> From<BoardOptions<N>> for Board<N>
// // where
// //     [(); N * N * N * N]: Sized,
// // {
// //     fn from(mut b: BoardOptions<N>) -> Self {
// //         let _: () = (1..=N * N)
// //             .map(|i| {
// //                 let r = shuffle_range::<N>();
// //                 let _ = r.iter().enumerate().map(|(j, &v)| {
// //                     let ind = board_index::<N>(i, j).unwrap();
// //                     let v = board_value::<N>(v).unwrap();
// //                     let _ = b.fix(ind, v);
// //                 });
// //             })
// //             .collect();
// //         Board {
// //             board: b.board_states.map(|c| c.try_into().unwrap()),
// //         }
// //     }
// // }

// impl<const N: usize> From<Board<N>> for BoardOptions<N>
// where
//     [(); N * N * N * N]: Sized,
// {
//     fn from(b: Board<N>) -> Self {
//         BoardOptions {
//             board_states: b.board.map(|c| c.into()),
//         }
//     }
// }
