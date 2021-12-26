use crate::player::Player;
use rand::{self, random};

pub fn fight(a: Player, b: Player) {
    // Coin Flip - which player starts.
    let coin = rand::random::<u8>();
    println!("Coin Flip: {}", coin);

    if coin > 128 {
        println!("Player A: {} has first draw.", a.name);

        let remaining_health = a.stats.attack - b.stats.health;
        if remaining_health <= 0 {
            println! {"Player A wins! Player B has {} Health.", remaining_health}
        } else {
            println! {"Player A attacked! Player B has {} Health left.", remaining_health}
        }
    } else {
        println!("Player B: {} has first draw.", b.name);

        let remaining_health = b.stats.attack - a.stats.health;
        if remaining_health <= 0 {
            println! {"Player B wins! Player A has {} Health.", remaining_health}
        } else {
            println! {"Player B attacked! Player A has {} Health left.", remaining_health}
        }
    }
}
