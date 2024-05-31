pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut solutions = vec![];
        Self::solve(&mut board, 0, &mut solutions);
        solutions
    }

    fn solve(board: &mut Vec<Vec<char>>, row: usize, solutions: &mut Vec<Vec<String>>) {
        if row == board.len() {
            let solution = board.iter().map(|row| row.iter().collect()).collect();
            solutions.push(solution);
            return;
        }

        for col in 0..board.len() {
            if Self::is_safe(board, row, col) {
                board[row][col] = 'Q';
                Self::solve(board, row + 1, solutions);
                board[row][col] = '.';
            }
        }
    }

    fn is_safe(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
        for i in 0..row {
            if board[i][col] == 'Q' {
                return false;
            }
        }

        for i in 0..row {
            let j = i as isize - (row as isize - col as isize);
            if j >= 0 && board[i][j as usize] == 'Q' {
                return false;
            }
        }

        for i in 0..row {
            let j = row + col - i;
            if j < board.len() && board[i][j] == 'Q' {
                return false;
            }
        }

        true
    }
}
