use crate::enums::{other_player, Player, Result};
use crate::gamestate::GameState;

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct Command{

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    Move(Direction),
    Hop(Direction, Direction),
    HWall(usize, usize),
    VWall(usize, usize),
}

/*
Any intersection on the wall grid is either empty or occupied
by a horizontal or vertical wall.
All intersections start off empty
 */
#[derive(Clone, Copy, PartialEq)]
pub enum Intersection {
    Horizontal,
    Vertical,
    Empty,
}

const WALLS_PER_PLAYER: u8 = 10;
const BOARD_SIZE: usize = 9;
const M: usize = BOARD_SIZE - 1;

type WallGridType = [[Intersection; M]; M];

fn all_move_commands() -> Vec<Command> {
    vec![
        Command::Move(Direction::Up),
        Command::Move(Direction::Down),
        Command::Move(Direction::Left),
        Command::Move(Direction::Right),
    ]
}

fn all_hop_commands()->Vec<Command>{
    vec![
        Command::Hop(Direction::Up, Direction::Up),
        Command::Hop(Direction::Up, Direction::Left),
        Command::Hop(Direction::Up, Direction::Right),
        Command::Hop(Direction::Down, Direction::Down),
        Command::Hop(Direction::Down, Direction::Left),
        Command::Hop(Direction::Down, Direction::Right),
        Command::Hop(Direction::Left, Direction::Up),
        Command::Hop(Direction::Left, Direction::Down),
        Command::Hop(Direction::Left, Direction::Left),
        Command::Hop(Direction::Right, Direction::Up),
        Command::Hop(Direction::Right, Direction::Down),
        Command::Hop(Direction::Right, Direction::Right),
    ]
}

fn all_wall_commands()->Vec<Command>{
    let mut commands:Vec<Command>=Vec::new();

    for i in 0..M{
        for j in 0..M{
            commands.push(Command::HWall(i, j));
            commands.push(Command::VWall(i, j));
        }
    }
    commands
}

fn all_commands()->Vec<Command>{
   
    let mut commands:Vec<Command>=Vec::new();
    commands.extend(all_move_commands());
    commands.extend(all_hop_commands());
    commands.extend(all_wall_commands());
    commands
}

 

#[derive(Clone, Copy, PartialEq)]
pub struct Location {
    pub i: usize,
    pub j: usize,
}

impl Location {
    pub fn new(i: usize, j: usize) -> Location {
        assert!(i <= M, "i = {}, j = {}", i, j);
        assert!(j <= M, "i = {}, j = {}", i, j);
        assert!(j <= M);
        Location { i: i, j: j }
    }
}
use std::ops::Add;
impl Add<Direction> for Location {
    type Output = Location;
    fn add(self, other: Direction) -> Location {
        match other {
            Direction::Up => Location::new(self.i, self.j + 1),
            Direction::Down => Location::new(self.i, self.j - 1),
            Direction::Left => Location::new(self.i - 1, self.j),
            Direction::Right => Location::new(self.i + 1, self.j),
        }
    }
}

#[derive(Clone, Copy)]
pub struct PlayerState {
    pub location: Location,
    pub walls: u8,
}

#[derive(Clone)]
pub struct State {
    player: Player,

    commands: Vec<Command>,
    pub walls: WallGridType,
    pub player1: PlayerState,
    pub player2: PlayerState,
}

fn move_player(player: &PlayerState, direction: Direction) -> PlayerState {
    PlayerState {
        location: player.location + direction,
        walls: player.walls,
    }
}

fn hwall(walls: &WallGridType, i: usize, j: usize) -> WallGridType {
    assert!(walls[j][i] == Intersection::Empty);
    let mut w = walls.clone();
    w[j][i] = Intersection::Horizontal;
    w
}

fn vwall(walls: &WallGridType, i: usize, j: usize) -> WallGridType {
    assert!(walls[j][i] == Intersection::Empty);
    let mut w = walls.clone();
    w[j][i] = Intersection::Vertical;
    w
}

impl State {
    pub fn new() -> Self {
        let mut s=  State {
            player: Player::One,

            walls: [[Intersection::Empty; M]; M],
            player1: PlayerState {
                location: Location::new(BOARD_SIZE / 2, 0),
                walls: WALLS_PER_PLAYER,
            },
            player2: PlayerState {
                location: Location::new(BOARD_SIZE / 2, BOARD_SIZE - 1),
                walls: WALLS_PER_PLAYER,
            },
            commands:Vec::new()
        };

        s.commands=s.legal_commands();
        s
    }
    pub fn current_player_state(&self) -> &PlayerState {
        match self.player {
            Player::One => &self.player1,
            Player::Two => &self.player2,
        }
    }
    fn hop(&self, d1: &Direction, d2: &Direction) -> Self {
        unimplemented!();
    }
    fn _move(&self, direction: &Direction) -> Self {
        let player1 = match self.player {
            Player::One => move_player(&self.player1, *direction),
            Player::Two => self.player1,
        };
        let player2 = match self.player {
            Player::One => self.player2,
            Player::Two => move_player(&self.player2, *direction),
        };

        State {
            player: other_player(self.player),
            walls: self.walls,
            player1: player1,
            player2: player2,
            commands:Vec::new()  // set commands in `apply`
        }
    }
    fn hwall(&self, i: usize, j: usize) -> Self {
        State {
            player: other_player(self.player),
            player1: self.player1,
            player2: self.player2,
            walls: hwall(&self.walls, i, j),
            commands:Vec::new()  // set commands in `apply`
        }
    }
    fn vwall(&self, i: usize, j: usize) -> Self {
        State {
            player: other_player(self.player),
            player1: self.player1,
            player2: self.player2,
            walls: vwall(&self.walls, i, j),
            commands:Vec::new()  // set commands in `apply`
        }
    }

    fn outside_bounds(&self, direction: Direction) -> bool {
        let location = self.current_player_state().location;
        let i = location.i;
        let j = location.j;

        (j == 0 && direction == Direction::Down)
            || (j >= M && direction == Direction::Up)
            || (i == 0 && direction == Direction::Left)
            || (i == M && direction == Direction::Right)
    }

    fn piece_blocking(&self, direction: Direction) -> bool {
        let target_location = self.current_player_state().location + direction;
        // is there a piece there?
        (self.player1.location == target_location) || (self.player2.location == target_location)
    }

    /* 2 ways of doing this. We'll start with this way and eventually move to the bitfield optimization */
    fn wall_blocking(&self, direction: Direction) -> bool {
        let location = self.current_player_state().location;
        let i = location.i;
        let j = location.j;
        const V: Intersection = Intersection::Vertical;
        const H: Intersection = Intersection::Horizontal;

        match direction {
            Direction::Right => {
                i == M || (j < M && self.walls[j][i] == V) || (j > 0 && self.walls[j - 1][i] == V)
            }
            Direction::Left => {
                i == 0
                    || (j < M && self.walls[j][i - 1] == V)
                    || (j > 0 && self.walls[j - 1][i - 1] == V)
            }
            Direction::Up => {
                j == M || (i < M && self.walls[j][i] == H) || (i > 0 && self.walls[j][i - 1] == H)
            }
            Direction::Down => {
                j == 0
                    || (i < M && self.walls[j - 1][i] == H)
                    || (i > 0 && self.walls[j - 1][i - 1] == H)
            }
        }
    }

    /*TODO: implement optimized bitwise boundary-checking */
    fn is_legal_move(&self, direction: Direction, check_pieces: bool) -> bool {
        if self.outside_bounds(direction) {
            return false;
        };

        if check_pieces && self.piece_blocking(direction) {
            return false;
        }

        if self.wall_blocking(direction) {
            return false;
        }
        true
    }

    fn is_legal_hwall(&self, i: usize, j: usize) -> bool {
        assert!(j<=7);
        assert!(i<=7);
        self.walls[j][i] == Intersection::Empty
            && (i == 0 || self.walls[j][i - 1] != Intersection::Horizontal)
            && (i == (M-1) || self.walls[j][i + 1] != Intersection::Horizontal)
    }
    fn is_legal_vwall(&self, i: usize, j: usize) -> bool {
        assert!(j<=7);
        assert!(i<=7);

        self.walls[j][i] == Intersection::Empty
            && (j == 0 || self.walls[j - 1][i] != Intersection::Horizontal)
            && (j == (M-1) || self.walls[j + 1][i] != Intersection::Horizontal)
    }
    fn is_legal_hop(&self, d1: Direction, d2: Direction) -> bool {
        false //TODO
    }
    fn is_legal(&self, command: &Command) -> bool {
        match command {
            Command::Move(direction) => self.is_legal_move(*direction, true),
            Command::HWall(i, j) => self.is_legal_hwall(*i, *j),
            Command::VWall(i, j) => self.is_legal_vwall(*i, *j),
            Command::Hop(d1, d2) => self.is_legal_hop(*d1, *d2),
        }
    }


    fn legal_commands(&self)->Vec<Command>{
        all_commands().into_iter().filter(|command| self.is_legal(command)).collect()
    }
}

impl GameState for State {
    type CommandType = Command;
    fn player(&self) -> Player {
        self.player
    }

    fn result(&self) -> Result {
        if self.player1.location.j == BOARD_SIZE - 1 {
            Result::PlayerOne
        } else if self.player2.location.j == 0 {
            Result::PlayerTwo
        } else {
            Result::InProgress
        }
    }

    fn commands(&self) -> &Vec<Self::CommandType> {
           &self.commands
     }

    fn apply(&self, command: &Self::CommandType) -> Self {
        assert!(self.result() == Result::InProgress, "game not in progress");
        assert!(self.commands().contains(&command), "bad command");

        let mut new_state:State=  match command {
            Command::Hop(d1, d2) => self.hop(d1, d2),
            Command::Move(d1) => self._move(d1),
            Command::VWall(i, j) => self.vwall(*i, *j),
            Command::HWall(i, j) => self.hwall(*i, *j),
        };

        new_state.commands=new_state.legal_commands();
        new_state

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let state = State::new();
        assert_eq!(state.result(), Result::InProgress);

        let location = state.current_player_state().location;
        for command in state.commands(){

            match command{
                Command::Move(direction)=>{
                    assert!(*direction!=Direction::Down);
                    let new_location = location + *direction; //shouldn't panic
                 },
                Command::Hop(d1,d2)=>{
                    let new_location = location + *d1 + *d2; //shouldn't panic
                },
                _=>{}
            }
        }
    }

    #[test]
    fn test_b(){
        let state = State::new();
        let command = Command::Move(Direction::Up);

        let state = state.apply(&command);




        for command in state.legal_commands(){
            assert!( command!=Command::Move(Direction::Up));
        }

        for command in state.commands(){
            assert!( *command!=Command::Move(Direction::Up));
        }



    }
}
