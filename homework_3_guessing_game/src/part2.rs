use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, mut min: u32, mut max: u32) -> u32 {
        while min <= max {
            let guess = (min + max) / 2;
            let result = player.ask_to_compare(guess);
            if result == -1 {
                max = guess - 1;
            } else if result == 0 {
                return guess;
            } else if result == 1 {
                min = guess + 1;
            }
        }
    0 
    }
}
