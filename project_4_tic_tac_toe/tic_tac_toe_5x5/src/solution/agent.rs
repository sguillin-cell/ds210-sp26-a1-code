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

    // -----------------------------
    // FAST WINDOW SCORING (STATIC)
    // -----------------------------
    fn window_score(x: i32, o: i32) -> i32 {
        match (x, o) {
            (3, 0) => 120,
            (0, 3) => -120,

            (2, 0) => 35,
            (0, 2) => -35,

            (1, 0) => 10,
            (0, 1) => -10,

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
                    let mut x = 0;
                    let mut o = 0;

                    for k in 0..3 {
                        match b[(i + k) as usize][(j - k) as usize] {
                            Cell::X => x += 1,
                            Cell::O => o += 1,
                            Cell::Empty => {}
                            Cell::Wall => {
                                x = -1;
                                break;
                            }
                        }
                    }

                    if x >= 0 {
                        score += Self::window_score(x, o);
                    }
                }
            }
        }

        score
    }

    // -----------------------------
    // MINIMAX (ALPHA-BETA ONLY)
    // -----------------------------
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

        best
    }

    // -----------------------------
    // MOVE HEURISTIC (KEY FIX FOR 3x3)
    // -----------------------------
    fn move_heuristic(board: &mut Board, player: Player, m: (usize, usize)) -> i32 {
        board.apply_move(m, player);

        let score = Self::evaluate(board, player);

        board.undo_move(m, player);

        score
    }

    // -----------------------------
    // ROOT SOLVER
    // -----------------------------
    fn solve_internal(board: &mut Board, player: Player) -> (i32, usize, usize) {
        let moves = board.moves();

        let mut best_move = moves[0];
        let mut best_score = if player == Player::X {
            i32::MIN
        } else {
            i32::MAX
        };

        let max_depth = 4;

        for m in moves {
            // 🔥 immediate impact (fixes 3x3 weakness)
            let move_score = Self::move_heuristic(board, player, m);

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