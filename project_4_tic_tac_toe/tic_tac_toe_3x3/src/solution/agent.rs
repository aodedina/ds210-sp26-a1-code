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
    // explores possible game states and chooses the optimal move; X maximizes score and O minimizes score 
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`

        //base case: game is over, return final score
        if board.game_over() {
            return (board.score(), 0,0);
        }

        //get all possible moves on the board
        let moves = board.moves();

        //start with move at index 1 being best, then iterates through all moves 
        let mut best_move = moves[0];
        //initialize best score. X wants to maximize so start with worst possible scenerio
        // and move up; O wants to minimize so start with worst possible scenrio and
        // move down
        let mut best_score;
        match player {
            Player::X => best_score = i32::MIN,
            Player::O => best_score = i32::MAX,
        }

        //loop through possible moves
        for m in moves {
            board.apply_move(m, player);

            //evaluate the resulting board
            let (score, _, _) = SolutionAgent::solve(
                board, 
                match player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                },
                _time_limit,
            );
            //unmove board before trying other possible moves
            board.undo_move(m, player);

            //choose best move for player
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
        let (x, y) = best_move;
        return (best_score, x, y);
    }
}
