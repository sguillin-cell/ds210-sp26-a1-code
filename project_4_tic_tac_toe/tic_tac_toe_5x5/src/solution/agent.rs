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

    fn evaluate(board: &Board, _player: Player) -> i32 {
        let b = board.get_cells();
        let n = b.len() as isize;

        let mut score = 0;

        for i in 0..n {
            for j in 0..n {
                // horizontal
                if j + 2 < n {
                    let mut x = 0;
                    let mut o = 0;

                    for k in 0..3 {
                        match b[i as usize][(j + k) as usize] {
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

                // vertical
                if i + 2 < n {
                    let mut x = 0;
                    let mut o = 0;

                    for k in 0..3 {
                        match b[(i + k) as usize][j as usize] {
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

                // diag ↘
                if i + 2 < n && j + 2 < n {
                    let mut x = 0;
                    let mut o = 0;

                    for k in 0..3 {
                        match b[(i + k) as usize][(j + k) as usize] {
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

                // diag ↙
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

    fn minimax(
        board: &mut Board,
        player: Player,
        depth: i32,
        mut alpha: i32,
        mut beta: i32,
        maximizing: bool,
    ) -> i32 {
        if depth == 0 {
            return Self::evaluate(board, player);
        }

        let moves = board.moves();
        if moves.is_empty() {
            return Self::evaluate(board, player);
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
    fn move_heuristic(board: &mut Board, player: Player, m: (usize, usize)) -> i32 {
        board.apply_move(m, player);

        let score = Self::evaluate(board, player);

        board.undo_move(m, player);

        score
    }

    fn solve_internal(board: &mut Board, player: Player) -> (i32, usize, usize) {
        let moves = board.moves();

        let mut best_move = moves[0];

        let mut best_score = if player == Player::X {
            i32::MIN
        } else {
            i32::MAX
        };

        let depth = 4;

        for m in moves {
            // 🔥 immediate impact (fixes 3x3 weakness)
            let move_score = Self::move_heuristic(board, player, m);

            board.apply_move(m, player);

            let search_score = Self::minimax(
                board,
                Self::opponent(player),
                depth,
                i32::MIN,
                i32::MAX,
                player == Player::X,
            );

            board.undo_move(m, player);

            let total = move_score * 3 + search_score;

            if player == Player::X {
                if total > best_score {
                    best_score = total;
                    best_move = m;
                }
            } else {
                if total < best_score {
                    best_score = total;
                    best_move = m;
                }
            }
        }

        (best_score, best_move.0, best_move.1)
    }
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        Self::solve_internal(board, player)
    }
}