type Board = Vec<Vec<bool>>;
type Solutions = Vec<Vec<String>>;

pub fn solve_n_queens(n: usize) -> Solutions {
    let mut board = vec![vec![false; n]; n];
    let mut solutions = Vec::new();
    _backtrack_n_queens(n, 0, &mut board, &mut solutions);
    solutions
}

pub fn _backtrack_n_queens(n: usize, cur_row: usize, board: &mut Board, solutions: &mut Solutions) {
    if cur_row == n {
        solutions.push(board_to_string(board));
        return;
    }
    for col in 0..n {
        if is_valid(board, cur_row, col) {
            board[cur_row][col] = true;
            _backtrack_n_queens(n, cur_row + 1, board, solutions);
            board[cur_row][col] = false;
        }
    }
}

pub fn is_valid(board: &Board, row: usize, col: usize) -> bool {
    // we don't need to check the current row, because we always place one queen per row
    // we check the col
    // we only need to check the upper left and upper right diagonal since we are always at the bottom row
    for i in 0..row {
        if board[i][col]
            || row - i <= col && board[i][col - (row - i)]
            || col + (row - i) < board.len() && board[i][col + (row - i)]
        {
            return false;
        }
    }
    true
}

#[inline]
pub fn board_to_string(board: &Board) -> Vec<String> {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|&cell| if cell { 'Q' } else { '.' })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_n_queens_solver {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (n, expected_solutions) = $tc;
                    let solutions = solve_n_queens(n);
                    assert_eq!(solutions, expected_solutions);
                }
            )*
        };
    }

    test_n_queens_solver! {
        test_0_queens: (0, vec![Vec::<String>::new()]),
        test_1_queen: (1, vec![vec!["Q"]]),
        test_2_queens:(2, Vec::<Vec<String>>::new()),
        test_3_queens:(3, Vec::<Vec<String>>::new()),
        test_4_queens: (4, vec![
            vec![".Q..",
                 "...Q",
                 "Q...",
                 "..Q."],
            vec!["..Q.",
                 "Q...",
                 "...Q",
                 ".Q.."],
        ]),
        test_5_queens:(5, vec![
            vec!["Q....",
                 "..Q..",
                 "....Q",
                 ".Q...",
                 "...Q."],
            vec!["Q....",
                 "...Q.",
                 ".Q...",
                 "....Q",
                 "..Q.."],
            vec![".Q...",
                 "...Q.",
                 "Q....",
                 "..Q..",
                 "....Q"],
            vec![".Q...",
                 "....Q",
                 "..Q..",
                 "Q....",
                 "...Q."],
            vec!["..Q..",
                 "Q....",
                 "...Q.",
                 ".Q...",
                 "....Q"],
            vec!["..Q..",
                 "....Q",
                 ".Q...",
                 "...Q.",
                 "Q...."],
            vec!["...Q.",
                 "Q....",
                 "..Q..",
                 "....Q",
                 ".Q..."],
            vec!["...Q.",
                 ".Q...",
                 "....Q",
                 "..Q..",
                 "Q...."],
            vec!["....Q",
                 ".Q...",
                 "...Q.",
                 "Q....",
                 "..Q.."],
            vec!["....Q",
                 "..Q..",
                 "Q....",
                 "...Q.",
                 ".Q..."],
        ]),
        test_6_queens: (6, vec![
            vec![".Q....",
                 "...Q..",
                 ".....Q",
                 "Q.....",
                 "..Q...",
                 "....Q."],
            vec!["..Q...",
                 ".....Q",
                 ".Q....",
                 "....Q.",
                 "Q.....",
                 "...Q.."],
            vec!["...Q..",
                 "Q.....",
                 "....Q.",
                 ".Q....",
                 ".....Q",
                 "..Q..."],
            vec!["....Q.",
                 "..Q...",
                 "Q.....",
                 ".....Q",
                 "...Q..",
                 ".Q...."],
        ]),
    }
}
