use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let depth_lim = 4;
        SolutionAgent::minimax(board, player, 0, depth_lim)
    }
}

impl SolutionAgent {
    fn minimax(
        board: &mut Board,
        player: Player,
        depth: i32,
        max_depth: i32,
    ) -> (i32, usize, usize) {
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        if depth == max_depth {
            return (Self::heuristic(board), 0, 0);
        }

        let moves = board.moves();

        let mut best_score;
        let mut best_row = 0;
        let mut best_column = 0;

        if player == Player::X {
            best_score = -100000;
        } else {
            best_score = 100000;
        }

        for move_pos in moves {
            board.apply_move(move_pos, player);

            let next_player;
            if player == Player::X {
                next_player = Player::O;
            } else {
                next_player = Player::X;
            }

            let result = Self::minimax(
                board,
                next_player,
                depth + 1,
                max_depth,
            );

            let score = result.0;

            board.undo_move(move_pos, player);

            if player == Player::X {
                if score > best_score {
                    best_score = score;
                    best_row = move_pos.0;
                    best_column = move_pos.1;
                }
            } else {
                if score < best_score {
                    best_score = score;
                    best_row = move_pos.0;
                    best_column = move_pos.1;
                }
            }
        }

        (best_score, best_row, best_column)
    }

    fn heuristic(board: &Board) -> i32 {
        board.score() * 10
    }
}
