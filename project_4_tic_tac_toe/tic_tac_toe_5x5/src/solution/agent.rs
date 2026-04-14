use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let max_depth = if board.moves().len() <= 9 {
    7  
    } else {
    3
    };
        SolutionAgent::minimax(board, player, 0, max_depth)
    }
}
//implement minimax helper functino for SolutionAgent
impl SolutionAgent {
    fn minimax(
        board: &mut Board,
        player: Player,
        depth: i32,
        max_depth: i32,
    ) -> (i32, usize, usize) {

        //base case; if case i finished, return final score
        if board.game_over() {
            return (board.score(), 0, 0);
        }
        //base case; if reach maximum search depth, stop searching and estimate position using heuristic
        if depth == max_depth {
            return (Self::heuristic(board), 0, 0);
        }
        //get all possible best move
        let moves = board.moves();
        //initialize best move
        let mut best_move = moves[0];
        //initalize best score depending on player (X tries to maximize and O tries to minimize)
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };

        //loop through possible moves
        for m in moves {
            board.apply_move(m, player);

            let (score, _, _) = Self::minimax(
                board,
                match player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                },
                depth + 1,
                max_depth,
            );
            //undo board to prepare for next iteration
            board.undo_move(m, player);
            //choose best move depending on player 
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
        //get x and y coordinates from best move
        let (x, y) = best_move;
        (best_score, x, y)
    }
    //heuristic function used when depth limit is reached
    fn heuristic(board: &Board) -> i32 {
        let base = board.score();
        //number of possible moves left on board
        let mobility = board.moves().len() as i32;

        //rewards having more available moves (more flexibility)
        let mobility_bonus = mobility * 3;

        //prioritizes winning advantages over mobility and small positional differences that don't win the game
        let weighted_score = base * 15;
        //combines board strength with flexibility into one final value
        weighted_score + mobility_bonus
    }
}