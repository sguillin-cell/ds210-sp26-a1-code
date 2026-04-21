use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl SolutionAgent {
    fn opponent(p: Player) -> Player {
        match p {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn evaluate_line(a: &Cell, b: &Cell, c: &Cell) -> i32 {
        let mut x_count = 0;
        let mut o_count = 0;

        for cell in [a, b, c] {
            match cell {
                Cell::X => x_count += 1,
                Cell::O => o_count += 1,
                Cell::Wall => return 0,
                _ => {}
            }
        }

        if x_count > 0 && o_count > 0 {
            return 0;
        }

        match (x_count, o_count) {
            (3, 0) => 100,
            (2, 0) => 10,
            (1, 0) => 1,
            (0, 3) => -100,
            (0, 2) => -10,
            (0, 1) => -1,
            _ => 0,
        }
    }

    fn heuristic(board: &Board) -> i32 {
        let cells = board.get_cells();
        let n = cells.len();
        let mut score = 0;

        for i in 0..n {
            for j in 0..n {
                if j + 2 < n {
                    score += Self::evaluate_line(&cells[i][j], &cells[i][j+1], &cells[i][j+2]);
                }
                if i + 2 < n {
                    score += Self::evaluate_line(&cells[i][j], &cells[i+1][j], &cells[i+2][j]);
                }
                if i + 2 < n && j + 2 < n {
                    score += Self::evaluate_line(&cells[i][j], &cells[i+1][j+1], &cells[i+2][j+2]);
                }
                if i + 2 < n && j >= 2 {
                    score += Self::evaluate_line(&cells[i][j], &cells[i+1][j-1], &cells[i+2][j-2]);
                }
            }
        }

        return score;
    }

    fn minimax(
        board: &mut Board,
        player: Player,
        depth: i32,
        mut alpha: i32,
        mut beta: i32,
        maximizing: bool,
    ) -> i32 {
        if board.game_over() {
            return board.score() * 10000;
        }

        if depth == 0 {
            return Self::heuristic(board);
        }

        let moves = board.moves();
        if moves.is_empty() {
            return Self::heuristic(board);
        }

        let mut best = if maximizing { i32::MIN } else { i32::MAX };

        for m in moves {
            board.apply_move(m, player);

            let val = Self::minimax(
                board,
                Self::opponent(player),
                depth - 1,
                alpha,
                beta,
                !maximizing,
            );

            board.undo_move(m, player);

            if maximizing {
                best = best.max(val);
                alpha = alpha.max(best);
            } else {
                best = best.min(val);
                beta = beta.min(best);
            }

            if beta <= alpha {
                break;
            }
        }

        return best;
    }
} 

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let moves = board.moves();

        let mut best_move = moves[0];
        let mut best_score = if player == Player::X {
            i32::MIN
        } else {
            i32::MAX
        };

        let max_depth = 4;

        for m in moves {
            board.apply_move(m, player);

            let next = SolutionAgent::opponent(player);

            let score = SolutionAgent::minimax(
                board,
                next,
                max_depth - 1,
                i32::MIN,
                i32::MAX,
                next == Player::X,
            );

            board.undo_move(m, player);

            match player {
                Player::X => {
                    if score > best_score {
                        best_score = score;
                        best_move = m;
                    }
                }
                Player::O => {
                    if score < best_score {
                        best_score = score;
                        best_move = m;
                    }
                }
            }
        }

        return (best_score, best_move.0, best_move.1);
    }
}