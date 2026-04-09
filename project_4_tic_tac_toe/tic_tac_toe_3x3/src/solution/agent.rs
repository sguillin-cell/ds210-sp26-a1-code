use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl SolutionAgent {
    fn opponent(player: Player) -> Player {
        match player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    // Minimax recursion using board.score()
    fn minimax(board: &mut Board, player: Player, maximizing: bool, _time_limit: u64) -> i32 {
        if board.game_over() {
            return board.score();
        }

        let moves = board.moves();

        if maximizing {
            let mut best = i32::MIN;

            for m in moves {
                board.apply_move(m, player);

                let val = Self::minimax(
                    board,
                    Self::opponent(player),
                    false,
                );

                board.undo_move(m, player);

                best = best.max(val);
            }

            best
        } else {
            let mut best = i32::MAX;

            for m in moves {
                board.apply_move(m, player);

                let val = Self::minimax(
                    board,
                    Self::opponent(player),
                    true,
                );

                board.undo_move(m, player);

                best = best.min(val);
            }

            best
        }
    }
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let mut best_score = if player == Player::X {
            i32::MIN
        } else {
            i32::MAX
        };

        let mut best_move = (0, 0);

        for m in board.moves() {
            board.apply_move(m, player);

            let score = SolutionAgent::minimax(
                board,
                SolutionAgent::opponent(player),
                player == Player::O,
            );

            board.undo_move(m, player);

            if player == Player::X {
                if score > best_score {
                    best_score = score;
                    best_move = m;
                }
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = m;
                }
            }
        }

        (best_score, best_move.0, best_move.1)
    }
}

