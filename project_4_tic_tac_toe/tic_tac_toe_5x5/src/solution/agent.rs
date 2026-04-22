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

    fn evaluate(board: &Board) -> i32 {
        let b = board.get_cells();
        let n = b.len() as isize;

        // Real game objective: maximize board.score() for X, minimize for O.
        // board.score() already counts overlapping triples, so XXXX contributes 2.
        let mut score = board.score() * 1000;

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

    fn best_immediate_swing(board: &mut Board, player: Player) -> i32 {
        let current = board.score();
        let mut best = if player == Player::X { i32::MIN } else { i32::MAX };
        let moves = board.moves();

        if moves.is_empty() {
            return 0;
        }

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

    fn ordered_moves(board: &mut Board, player: Player) -> Vec<(usize, usize)> {
        let base_score = board.score();
        let mut scored_moves: Vec<(i32, (usize, usize))> = Vec::new();

        for m in board.moves() {
            board.apply_move(m, player);
            let delta = board.score() - base_score;
            board.undo_move(m, player);
            scored_moves.push((delta, m));
        }

        if player == Player::X {
            scored_moves.sort_by(|a, b| b.0.cmp(&a.0));
        } else {
            scored_moves.sort_by(|a, b| a.0.cmp(&b.0));
        }

        scored_moves.into_iter().map(|(_, m)| m).collect()
    }

    fn minimax(board: &mut Board, player: Player, depth: i32, mut alpha: i32, mut beta: i32) -> i32 {
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
                if beta <= alpha {
                    break;
                }
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
                if beta <= alpha {
                    break;
                }
            }
            best
        }
    }
    fn move_heuristic(
        board: &mut Board,
        player: Player,
        base_score: i32,
        opp_best_reply_swing: i32,
    ) -> i32 {
        let mut score = Self::evaluate(board);
        let delta = board.score() - base_score;
        let own_best_swing = Self::best_immediate_swing(board, player);

        // Keep heuristic centered on real score swing first.
        score += delta * 300;

        // Tactical one-ply pressure.
        score += own_best_swing * 260;
        score += opp_best_reply_swing * 220;

        // As O, strongly avoid giving X immediate positive swings.
        if player == Player::O {
            if opp_best_reply_swing > 0 {
                score += opp_best_reply_swing * 220;
            }
        }

        score
    }

    fn solve_internal(board: &mut Board, player: Player) -> (i32, usize, usize) {
        let moves = Self::ordered_moves(board, player);
        let base_score = board.score();

        let mut best_move = moves[0];

        let mut best_score = if player == Player::X {
            i32::MIN
        } else {
            i32::MAX
        };

        // Dynamic depth to stay within time limits as branching grows.
        let depth = if player == Player::O {
            if moves.len() > 14 { 3 } else { 4 }
        } else if moves.len() > 16 {
            2
        } else {
            3
        };
        let mut best_opp_reply_swing = i32::MAX;
        let mut best_search_score = i32::MAX;

        for m in moves {
            board.apply_move(m, player);

            let opp_reply_swing = Self::best_immediate_swing(board, Self::opponent(player));
            let move_score = Self::move_heuristic(board, player, base_score, opp_reply_swing);
            let search_score = Self::minimax(board, Self::opponent(player), depth, i32::MIN, i32::MAX);

            board.undo_move(m, player);

            let total = if player == Player::O {
                move_score * 4 + search_score
            } else {
                move_score + search_score
            };

            if player == Player::X {
                if total > best_score {
                    best_score = total;
                    best_move = m;
                }
            } else {
                // Safety-first for black:
                // 1) minimize X's best immediate scoring swing
                // 2) then minimize deeper minimax score
                // 3) then use total heuristic as tie-breaker
                let better = opp_reply_swing < best_opp_reply_swing
                    || (opp_reply_swing == best_opp_reply_swing && search_score < best_search_score)
                    || (opp_reply_swing == best_opp_reply_swing
                        && search_score == best_search_score
                        && total < best_score);

                if better {
                    best_opp_reply_swing = opp_reply_swing;
                    best_search_score = search_score;
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