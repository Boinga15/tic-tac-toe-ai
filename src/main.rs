use crate::{genetic_algorithm::{agent::{Agent, Sigmoid}, factory::generate_agent}, tic_tac_toe::game::{Board, get_board_state}};

pub mod genetic_algorithm;
pub mod tic_tac_toe;

fn main() {
    let test_agent: Agent = generate_agent(9, 9, 5, 10, Box::new(Sigmoid {}), 5.0);

    let result: Vec<f64> = test_agent.compute(vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0]);
    println!("{result:?}");

    let board: Board = Board {
        board: [
            [1, -1, -1],
            [-1, 1, 1],
            [1, -1, -1]
        ]
    };

    let board_state = get_board_state(board);

    println!("{board_state}")
}