#![allow(unused)]

mod fight;
mod player;
use player::Class;
use player::Player;
use player::Stats;

fn main() {
    let max = Player {
        name: "Max".to_string(),
        class: Class::Thief,
        stats: Stats::assign_thief(),
    };

    let steven = Player {
        name: "Steven".to_string(),
        class: Class::Warrior,
        stats: Stats::assign_warrior(),
    };

    fight::fight(max, steven);
}
