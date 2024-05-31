fn n_queens_solver(n: usize) {
    let mut board = vec![vec!['.'; n]; n];
    let mut solutions = vec![];
    solve(&mut board, 0, &mut solutions)
}

fn is_safe(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
    for i in 0..row {
        if board[i][col] == 'Q' {
            return false;
        }
    }

    for i in 0..row {
        let j = i as isize + col as isize - row as isize;
        if j >= 0 && board[i][j as usize] == 'Q' {
            return false;
        }
    }

    for i in 0..row {
        let j = col + row - i;
        if j < board.len() && board[i][j as usize] == 'Q' {
            return false;
        }
    }

    true
}

fn solve(board: &mut Vec<Vec<char>>, row: usize, solutions: &mut Vec<Vec<String>>) {
    let n = board.len();
    if row == n {
        let solution = board.iter().map(|row| row.iter().collect()).collect();
        solutions.push(solution);
        return;
    }

    for col in 0..n {
        if is_safe(board, row, col) {
            board[row][col] = 'Q';
            solve(board, row + 1, solutions);
            board[row][col] = '.';
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
