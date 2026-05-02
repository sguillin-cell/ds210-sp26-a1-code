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

    /// Score a window of 3 cells from X's perspective (positive = good for X).
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

    /// Full positional evaluation from X's perspective.
    /// Positive = good for X, negative = good for O.
    fn evaluate(board: &Board) -> i32 {
        let b = board.get_cells();
        let n = b.len() as isize;

        // Real game score dominates.
        let mut score = board.score() * 1000;

        for i in 0..n {
            for j in 0..n {
                // horizontal
                if j + 2 < n {
                    let (mut x, mut o, mut wall) = (0, 0, false);
                    for k in 0..3 {
                        match b[i as usize][(j + k) as usize] {
                            Cell::X => x += 1,
                            Cell::O => o += 1,
                            Cell::Empty => {}
                            Cell::Wall => { wall = true; break; }
                        }
                    }
                    if !wall { score += Self::window_score(x, o); }
                }

                // vertical
                if i + 2 < n {
                    let (mut x, mut o, mut wall) = (0, 0, false);
                    for k in 0..3 {
                        match b[(i + k) as usize][j as usize] {
                            Cell::X => x += 1,
                            Cell::O => o += 1,
                            Cell::Empty => {}
                            Cell::Wall => { wall = true; break; }
                        }
                    }
                    if !wall { score += Self::window_score(x, o); }
                }

                // diag ↘
                if i + 2 < n && j + 2 < n {
                    let (mut x, mut o, mut wall) = (0, 0, false);
                    for k in 0..3 {
                        match b[(i + k) as usize][(j + k) as usize] {
                            Cell::X => x += 1,
                            Cell::O => o += 1,
                            Cell::Empty => {}
                            Cell::Wall => { wall = true; break; }
                        }
                    }
                    if !wall { score += Self::window_score(x, o); }
                }

                // diag ↙
                if i + 2 < n && j >= 2 {
                    let (mut x, mut o, mut wall) = (0, 0, false);
                    for k in 0..3 {
                        match b[(i + k) as usize][(j - k) as usize] {
                            Cell::X => x += 1,
                            Cell::O => o += 1,
                            Cell::Empty => {}
                            Cell::Wall => { wall = true; break; }
                        }
                    }
                    if !wall { score += Self::window_score(x, o); }
                }
            }
        }

        score
    }

    /// Returns the best score swing achievable in one move for `player`.
    /// Used to gauge immediate threat/opportunity at a position.
    fn best_immediate_swing(board: &mut Board, player: Player) -> i32 {
        let current = board.score();
        let moves = board.moves();
        if moves.is_empty() { return 0; }

        let mut best = if player == Player::X { i32::MIN } else { i32::MAX };
        for m in moves {
            board.apply_move(m, player);
            let swing = board.score() - current;
            board.undo_move(m, player);
            if player == Player::X {
                best = best.max(swing);
            } else {
                best = best.min(swing);
            }
        }
        best
    }

    /// Sort moves by immediate score delta, best-first for the current player.
    fn ordered_moves(board: &mut Board, player: Player) -> Vec<(usize, usize)> {
        let base_score = board.score();
        let mut scored: Vec<(i32, (usize, usize))> = board
            .moves()
            .into_iter()
            .map(|m| {
                board.apply_move(m, player);
                let delta = board.score() - base_score;
                board.undo_move(m, player);
                (delta, m)
            })
            .collect();

        // Both players: best delta first from their own perspective.
        if player == Player::X {
            scored.sort_by(|a, b| b.0.cmp(&a.0));
        } else {
            scored.sort_by(|a, b| a.0.cmp(&b.0)); // most negative first = best for O
        }

        scored.into_iter().map(|(_, m)| m).collect()
    }

    /// Alpha-beta minimax. Returns a score from X's perspective.
    fn minimax(
        board: &mut Board,
        player: Player,
        depth: i32,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        if depth == 0 {
            return Self::evaluate(board);
        }

        let moves = Self::ordered_moves(board, player);
        if moves.is_empty() {
            return Self::evaluate(board);
        }

        if player == Player::X {
            let mut best = i32::MIN;
            for m in moves {
                board.apply_move(m, player);
                let val = Self::minimax(board, Self::opponent(player), depth - 1, alpha, beta);
                board.undo_move(m, player);
                best = best.max(val);
                alpha = alpha.max(best);
                if beta <= alpha { break; }
            }
            best
        } else {
            let mut best = i32::MAX;
            for m in moves {
                board.apply_move(m, player);
                let val = Self::minimax(board, Self::opponent(player), depth - 1, alpha, beta);
                board.undo_move(m, player);
                best = best.min(val);
                beta = beta.min(best);
                if beta <= alpha { break; }
            }
            best
        }
    }

    /// Unified move evaluation score, symmetric for both players.
    ///
    /// For X: higher is better.
    /// For O: lower is better.
    ///
    /// We compute everything from X's perspective (the raw numbers),
    /// and the caller uses the appropriate comparison (> for X, < for O).
    fn move_score(
        board: &mut Board,
        player: Player,
        base_score: i32,
        depth: i32,
        alpha: i32,
        beta: i32,
    ) -> i32 {
        // 1. Deep search score (primary signal).
        let search = Self::minimax(board, Self::opponent(player), depth, alpha, beta);

        // 2. Immediate score delta for this move.
        let delta = board.score() - base_score;

        // 3. Best reply opponent can make from here (X's perspective).
        let opp_reply = Self::best_immediate_swing(board, Self::opponent(player));

        // 4. Positional evaluation of the resulting board.
        let positional = Self::evaluate(board);

        // Weighted combination — all from X's perspective.
        // search carries the most weight; positional and delta add short-range guidance.
        search * 5
            + positional * 2
            + delta * 300
            - opp_reply * 180  // penalise moves that give opponent easy replies
    }

    fn solve_internal(board: &mut Board, player: Player) -> (i32, usize, usize) {
        let moves = Self::ordered_moves(board, player);
        let base_score = board.score();

        // Dynamic depth: fewer moves remaining = can afford deeper search.
        let n_moves = moves.len();
        let depth = match n_moves {
            0..=8  => 6,
            9..=14 => 5,
            _      => 4,
        };

        let mut best_move = moves[0];
        let mut best_val = if player == Player::X { i32::MIN } else { i32::MAX };

        // Alpha-beta window for the top-level loop.
        let mut alpha = i32::MIN;
        let mut beta  = i32::MAX;

        for m in moves {
            board.apply_move(m, player);

            let val = Self::move_score(board, player, base_score, depth, alpha, beta);

            board.undo_move(m, player);

            // Symmetric: X maximises, O minimises — same logic, same scoring axis.
            let better = if player == Player::X {
                val > best_val
            } else {
                val < best_val
            };

            if better {
                best_val = val;
                best_move = m;

                // Tighten the window for subsequent move_score calls.
                if player == Player::X {
                    alpha = alpha.max(best_val);
                } else {
                    beta = beta.min(best_val);
                }
            }
        }

        (best_val, best_move.0, best_move.1)
    }
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        Self::solve_internal(board, player)
    }
}