use rand::Rng;

pub struct Board {
    pub board: [[i32; 3]; 3]
}

impl Board {
    // Returns 1 for Positive Win, -1 for Negative Win, 0 for no win state, and 2 for draw.
    pub fn get_board_state(&mut self) -> i32 {
        // Check for horizontal and vertical wins.
        for i in 0..3 {
            if self.board[i][0] == self.board[i][1] && self.board[i][1] == self.board[i][2] {
                return self.board[i][0];
            } else if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] {
                return self.board[0][i];
            }
        }

        // Next, check for diagonal wins.
        if (self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2]) || self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            return self.board[1][1];
        }

        // Finally, check to see if we have a draw.
        let mut is_draw: bool = true;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j] == 0 {
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


    pub fn probability_input(&mut self, probabilities: Vec<f64>, use_maximum: bool, value: i32) -> bool {
        // First, check to see what probabilities we could even use (and calculate total just in case).
        let mut ordered_probabilities: Vec<(usize, usize, f64)> = vec![];
        let mut total: f64 = 0.0;

        for (i, probability) in probabilities.iter().enumerate() {
            let row = i / 3;

            if self.board[row][i % 3] == 0 {
                let mut c_index = 0;
                let mut inserted: bool = false;

                while c_index < ordered_probabilities.len() {
                    if ordered_probabilities[c_index].2 >= *probability {
                        ordered_probabilities.insert(c_index, (row, i % 3, *probability));
                        inserted = true;
                        break;
                    } else {
                        c_index += 1;
                    }
                }

                if !inserted {
                    ordered_probabilities.push((row, i % 3, *probability));
                }

                total += *probability;
            }
        }

        if ordered_probabilities.len() <= 0 { // If the entire board is filled, then return that we failed to add the value.
            return false;
        }


        if use_maximum {
            let chosen_tile: (usize, usize, f64) = ordered_probabilities[ordered_probabilities.len() - 1];
            self.board[chosen_tile.0][chosen_tile.1] = value;
        } else {
            let mut rng = rand::thread_rng();
            let chosen_value = rng.gen_range(0.0..total);
            let mut c_index = 0;

            while c_index < ordered_probabilities.len() - 1 && chosen_value > ordered_probabilities[c_index].2 {
                c_index += 1;
            }

            let chosen_tile: (usize, usize, f64) = ordered_probabilities[c_index];
            self.board[chosen_tile.0][chosen_tile.1] = value;
        }
        
        return true;
    }
}


struct Game {
    
}