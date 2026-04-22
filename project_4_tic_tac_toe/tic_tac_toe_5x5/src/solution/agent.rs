use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;


pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let moves_left = board.moves().len();

        //depth dynamic to what stage of the game we are at; earlier stage 
        //limit depth to reduce run time; end of game, increase depth to 
        //search for accurate decisions
        let max_depth = if moves_left > 16 {
            3
        } else if moves_left > 9 {
            5
        } else {
            7
        };

        //added alpha beta minimax search to eliminate unnecessary branches
        //prioritizes moves that blocks opponents move by giving higher score 
        if let Some(m) = Self::find_immediate_move(board, player) {
            return (10000, m.0, m.1);
        }
    
        let (score, x, y) = Self::minimax(board, player, 0, max_depth, i32::MIN, i32::MAX);
        (score, x, y)
    }
}

//implement minimax helper functino for SolutionAgent
impl SolutionAgent {
    fn minimax( board: &mut Board, player: Player, depth: i32, max_depth: i32, mut alpha: i32, mut beta: i32) -> (i32, usize, usize) {

        //base case; if game is over, return final score; depth subtracted so faster wins
        //are preferred and slower losses penalized less
        if board.game_over() {
            return (board.score() - depth, 0, 0);
        }
        //base case; if reach maximum search depth, stop searching
        //and estimate position using heuristic rather than recursion searching
        if depth == max_depth {
            return (Self::heuristic(board), 0, 0);
        }
        let mut moves = board.moves();

        //improve pruning by prioritizing central positions
        moves.sort_by_key(|&(x, y)| {
            let center =2;
            (x as i32 - center).abs() + (y as i32 - center).abs()
        });

        //initialize best move
        let mut best_move = moves[0];
        

        //maximizing player, X, tries to get high score 
        if player == Player::X {
            let mut best_score = i32::MIN;

            //goes through moves and restores board for next iteration
            for m in moves {
                board.apply_move(m, player);

                let (score, _, _) = Self::minimax(board, Player::O, depth+1, max_depth, alpha, beta);

                board.undo_move(m, player);

                if score > best_score {
                    best_score = score; 
                    best_move = m;
                }
                //tracks best guarenteed score for maximizing player
                alpha = alpha.max(best_score);

                if beta <= alpha {
                    break;
                }
            }
            let(x, y) = best_move;
            (best_score, x, y)

        } else {
            //minimizing player, O, tries to get lowest score
            let mut best_score = i32::MAX;

            for m in moves {
                board.apply_move(m, player);

                let(score, _, _) = Self::minimax(board, Player::X, depth+1, max_depth, alpha, beta);

                board.undo_move(m, player);

                if score < best_score {
                    best_score = score;
                    best_move = m;
                }

                //track best score for minimizing player
                beta = beta.min(best_score);

                if beta <= alpha {
                    break;
                }
            }
            let(x, y) = best_move;
            (best_score, x, y)
        }
    }

    //helper function to hone in on obvious wins (avoids missing 1-step wins)
    fn find_immediate_move(board: &mut Board, player: Player) -> Option<(usize, usize)> {
        let moves = board.moves();

        //checks if player can win in 1 move
        for m in &moves {
            board.apply_move(*m, player);
            if board.game_over() {
                board.undo_move(*m, player);
                return Some(*m);
            }
            board.undo_move(*m, player);
        }
        //determine opposing player to simulate next move
        let opponent = match player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
        //check if opponent can win in 1 move; if they can return move to block them
        for m in &moves {
            board.apply_move(*m, opponent);
            if board.game_over() {
                board.undo_move(*m, opponent);
                return Some(*m);
            }
            board.undo_move(*m, opponent);
        }
        //no threat found 
        None
    }
    
    //heuristic function used when depth limit is reached
    fn heuristic(board: &Board) -> i32 {
        let base = board.score();
        //number of possible moves left on board
        let mobility = board.moves().len() as i32;

        //rewards having more available moves (more flexibility)
        let mobility_bonus = mobility * 2;

        //prioritizes winning advantages over mobility and small positional differences that don't win the game
        let weighted_score = base * 20;
        //combines board strength with flexibility into one final value
        weighted_score + mobility_bonus
    }
}