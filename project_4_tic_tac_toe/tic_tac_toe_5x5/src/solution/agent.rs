use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        //counts amount of moves left
        let moves_left = board.moves().len();
        let depth_lim;
        //deeper search for when board is more empty
        if moves_left <= 5 {
            depth_lim = 7;
        } else if moves_left <= 9 {
            depth_lim = 6;
        } else {
            depth_lim = 4;
        }
        //starts minimax algorithm
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
            return (board.score() * 1000, 0, 0);
        }
        //base case
        if depth >= max_depth {
            return (Self::heuristic(board), 0, 0);
        }
        //gets all moves
        let moves = board.moves();
        //tracks best move and score
        let mut best_score;
        let mut best_row = 0;
        let mut best_column = 0;

        //x wants to maximize, o wants to minimize
        if player == Player::X {
            best_score = -100000;
        } else {
            best_score = 100000;
        }

        for move_pos in moves {
            board.apply_move(move_pos, player);
            //switch player
            let next_player;
            if player == Player::X {
                next_player = Player::O;
            } else {
                next_player = Player::X;
            }
            //recursively evaluate the move
            let result = Self::minimax(
                board,
                next_player,
                depth + 1,
                max_depth,
            );

            let score = result.0;

            board.undo_move(move_pos, player);
            //update best move depending on who is playing
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
        //returns best score and best moves
        (best_score, best_row, best_column)
    }
    //estimates board value when depth lim is reached
    fn heuristic(board: &Board) -> i32 {
        board.score() * 50
    }
}
