struct KnightTour {
    directions: Vec<(i32, i32)>,
}

impl KnightTour {
    pub fn new() -> Self {
        KnightTour {
            directions: vec![
                (-1, -2),
                (-2, -1),
                (-2, 1),
                (-1, 2),
                (1, -2),
                (2, -1),
                (2, 1),
                (1, 2),
            ],
        }
    }

    // m * n的棋盘, m为行数、n为列数
    pub fn solve(
        &self,
        m: usize,
        n: usize,
        start_x: usize,
        start_y: usize,
    ) -> Option<Vec<Vec<usize>>> {
        let mut solutions = None;
        let mut board = vec![vec![0; n]; m];
        board[start_y][start_x] = 1;
        self.solve_tour(start_x, start_y, 2, &mut board, &mut solutions);
        solutions
    }

    fn solve_tour(
        &self,
        x: usize,
        y: usize,
        number: usize,
        board: &mut Vec<Vec<usize>>,
        solutions: &mut Option<Vec<Vec<usize>>>,
    ) {
        if let Some(_) = solutions {
            return;
        }

        if number > board.len() * board[0].len() {
            *solutions = Some(board.clone());
            return;
        }

        self.directions.iter().for_each(|dir| {
            let dy = y as i32 + dir.0;
            let dx = x as i32 + dir.1;
            if self.check_ok(board, dx, dy) {
                let dy = dy as usize;
                let dx = dx as usize;
                board[dy][dx] = number;
                self.solve_tour(dx, dy, number + 1, board, solutions);
                board[dy][dx] = 0;
            }
        })
    }

    fn check_ok(&self, board: &mut Vec<Vec<usize>>, x: i32, y: i32) -> bool {
        x >= 0
            && (x as usize) < board[0].len()
            && y >= 0
            && (y as usize) < board.len()
            && board[y as usize][x as usize] == 0
    }
}
