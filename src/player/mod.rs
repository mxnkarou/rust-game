#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub class: Class,
    pub stats: Stats,
}

impl Player {
    pub fn new(self) -> Player {
        Self {
            name: self.name,
            class: self.class,
            stats: self.stats,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Class {
    Warrior,
    Wizard,
    Thief,
}

#[derive(Debug)]
pub struct Stats {
    pub health: i32,
    pub defense: i32,
    pub attack: i32,
}

impl Stats {
    pub fn assign_warrior() -> Stats {
        Self {
            health: 20,
            defense: 10,
            attack: 15,
        }
    }
    pub fn assign_wizard() -> Stats {
        Self {
            health: 10,
            defense: 10,
            attack: 25,
        }
    }
    pub fn assign_thief() -> Stats {
        Self {
            health: 15,
            defense: 20,
            attack: 15,
        }
    }
}
