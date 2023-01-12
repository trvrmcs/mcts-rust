#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Result {
    PlayerOne,
    PlayerTwo,
    Draw,
    InProgress,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    One,
    Two,
}

pub fn other_player(player: Player) -> Player {
    match player {
        Player::One => Player::Two,
        Player::Two => Player::One,
    }
}
