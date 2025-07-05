struct Solution;
impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let rows = board.len();
        let cols = board[0].len();
        let word_chars: Vec<char> = word.chars().collect();

        for row in 0..rows {
            for col in 0..cols {
                if Self::backtrack(&mut board, &word_chars, 0, row, col) {
                    return true;
                };
            }
        }

        false
    }
    fn backtrack(
        board: &mut Vec<Vec<char>>,
        word: &[char],
        word_index: usize,
        board_row: usize,
        board_col: usize,
    ) -> bool {

        if board_row >= board.len() || board_col >= board[0].len()
            || word[word_index] != board[board_row][board_col] {
            return false;
        }

        if word_index == word.len() - 1{
            return true;
        }

        let temp = board[board_row][board_col];
        board[board_row][board_col] = '#';

        let top = (-1, 0);
        let right = (0, 1);
        let bottom = (1, 0);
        let left = (0, -1);

        let directions = [top, right, bottom, left];

        for (row_direction, col_direction) in directions {
            let new_board_row = board_row as isize + row_direction;
            let new_board_col = board_col as isize + col_direction;

            if new_board_row >= 0
                && new_board_col >= 0
                && (new_board_row as usize) < board.len()
                && (new_board_col as usize) < board[0].len()
                && Self::backtrack(
                    board,
                    word,
                    word_index + 1,
                    new_board_row as usize,
                    new_board_col as usize,
                )
            {
                board[board_row][board_col] = temp;
                return true;
            }
        }

        board[board_row][board_col] = temp;
        false
    }
}

/*


*/
