use tic_tac_toe_5x5::layout::Layout5x5;
use tic_tac_toe_5x5::solution::agent::SolutionAgent;
use tic_tac_toe_stencil::{game_loop, Outcome};

#[test]
fn self_play_solution_vs_solution() {
    let mut x_wins = 0;
    let mut o_wins = 0;
    let mut draws = 0;

    for _ in 0..46 {
        let outcome = game_loop::<_, SolutionAgent, SolutionAgent>(
            Layout5x5::Random(5),
            3000,
            true,
        );

        match outcome {
            Outcome::X => x_wins += 1,
            Outcome::O => o_wins += 1,
            Outcome::Draw => draws += 1,
        }
    }

    println!("Self-play results:");
    println!("X wins: {}", x_wins);
    println!("O wins: {}", o_wins);
    println!("Draws : {}", draws);

    assert!(true);
}