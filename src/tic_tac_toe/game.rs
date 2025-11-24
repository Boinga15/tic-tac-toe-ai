pub struct Board {
    pub board: [[i32; 3]; 3]
}


// Returns 1 for Positive Win, -1 for Negative Win, 0 for no win state, and 2 for draw.
pub fn get_board_state(board: Board) -> i32 {
    // Check for horizontal and vertical wins.
    for i in 0..3 {
        if board.board[i][0] == board.board[i][1] && board.board[i][1] == board.board[i][2] {
            return board.board[i][0];
        } else if board.board[0][i] == board.board[1][i] && board.board[1][i] == board.board[2][i] {
            return board.board[0][i];
        }
    }

    // Next, check for diagonal wins.
    if (board.board[0][0] == board.board[1][1] && board.board[1][1] == board.board[2][2]) || board.board[0][2] == board.board[1][1] && board.board[1][1] == board.board[2][0] {
        return board.board[1][1];
    }

    // Finally, check to see if we have a draw.
    let mut is_draw: bool = true;
    for i in 0..3 {
        for j in 0..3 {
            if board.board[i][j] == 0 {
                is_draw = false;
            }
        }
    }

    if is_draw {
        return 2;
    } else {
        return 0;
    }
}