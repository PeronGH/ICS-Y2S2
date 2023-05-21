fn is_safe(board: &[Vec<bool>], row: usize, col: usize) -> bool {
    // Check this row on left side
    (0..col).all(|i| !board[row][i]) 
    && // Check upper diagonal on left side
    (0..=row.min(col)).rev().all(|i| !board[row - i][col - i]) 
    && // Check lower diagonal on left side
    (0..=board.len().saturating_sub(row+1).min(col)).all(|i| !board[row + i][col - i])
}

fn solve_n_queens_util(board: &mut [Vec<bool>], col: usize) -> Option<()> {
    if col >= board.len() {
        return Some(());
    }

    (0..board.len())
        .find(|&row| {
            if is_safe(board, row, col) {
                board[row][col] = true;
                if solve_n_queens_util(board, col + 1).is_some() {
                    return true;
                }
                board[row][col] = false;
            }
            false
        })
        .map(|_| ())
}

pub fn solve_n_queens(n: usize) -> Option<Vec<Vec<bool>>> {
    let mut board = vec![vec![false; n]; n];
    solve_n_queens_util(&mut board, 0)?;
    Some(board)
}
