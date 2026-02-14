use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        if min + 1 == max {
            return min;
        }
        let middle_num = (min + max) / 2;
        let solution = player.ask_to_compare(middle_num);
        if solution == 0 {
            return middle_num;
        }
        else if solution == -1 {
            return Part2::guess_the_number(player, min, middle_num);
        }
        else {
            return Part2::guess_the_number(player, middle_num+1, max);

        }
    }
}