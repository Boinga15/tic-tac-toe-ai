use crate::{genetic_algorithm::{agent::{Agent, Sigmoid}, factory::generate_agent}, tic_tac_toe::game::Board};

pub mod genetic_algorithm;
pub mod tic_tac_toe;

fn main() {
    let mut board: Board = Board {
        board: [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0]
        ]
    };

    let mut test_agent: Agent = generate_agent(9, 9, 5, 10, Box::new(Sigmoid {}), 5.0);

    let mut result: Vec<f64> = test_agent.compute(vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0]);
    println!("{result:?}");

    board.probability_input(result, false, 1);

    test_agent.mutate(2.0);
    result = test_agent.compute(vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0]);
    println!("{result:?}");

    board.probability_input(result, false, -1);

    let board_state = board.get_board_state();

    println!("Board:");
    for row in board.board {
        println!("{row:?}");
    }

    
    println!("\n{board_state}");
}