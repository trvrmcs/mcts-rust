use std::fmt;

use crate::enums::{other_player, Player, Result};
use crate::gamestate::GameState;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Command {
    pub i: usize,
    pub j: usize,
}

pub const SIZE: usize = 3;
impl Command {
    pub fn new(i: usize, j: usize) -> Command {
        assert!(i < SIZE);
        assert!(j < SIZE);
        Command { i, j }
    }
}

// impl fmt::Display for Command {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{},{}]", self.i, self.j)
//     }
// }

type BoardType = [[u8; SIZE]; SIZE];

#[derive(Clone)]
pub struct State {
    player: Player,
    // rows, columns
    board: BoardType,
    result: Result,

    commands_: Vec<Command>,
}

fn commands(board: &BoardType, result: Result) -> Vec<Command> {
    let mut c: Vec<Command> = vec![];

    if result != Result::InProgress {
        return c;
    }
    for j in 0..SIZE {
        for i in 0..SIZE {
            if board[j][i] == 0 {
                c.push(Command::new(i, j))
            }
        }
    }

    c
}

fn column_result(board: &BoardType, i: usize) -> Result {
    let first = board[0][i];

    for j in 1..SIZE {
        if board[j][i] != first {
            return Result::InProgress;
        }
    }

    match first {
        1 => Result::PlayerOne,
        2 => Result::PlayerTwo,
        _ => Result::InProgress,
    }
}
fn row_result(board: &BoardType, j: usize) -> Result {
    let row = board[j];
    let first = row[0];

    for i in 1..SIZE {
        if row[i] != first {
            return Result::InProgress;
        }
    }

    match first {
        1 => Result::PlayerOne,
        2 => Result::PlayerTwo,
        _ => Result::InProgress,
    }
}

fn diag_result_1(board: &BoardType) -> Result {
    let first = board[0][0];
    for x in 1..SIZE {
        if board[x][x] != first {
            return Result::InProgress;
        }
    }
    match first {
        1 => Result::PlayerOne,
        2 => Result::PlayerTwo,
        _ => Result::InProgress,
    }
}

fn diag_result_2(board: &BoardType) -> Result {
    /*
        This is failing, we need unit tests
    */
    let n = SIZE - 1;
    let first = board[0][n];
    for x in 1..SIZE {
        if board[x][n - x] != first {
            return Result::InProgress;
        }
    }
    match first {
        1 => Result::PlayerOne,
        2 => Result::PlayerTwo,
        _ => Result::InProgress,
    }
}

fn result(board: &BoardType) -> Result {
    
    for i in 0..SIZE {
        let r = column_result(board, i);
        if r != Result::InProgress {
            return r;
        }
    }
    for j in 0..SIZE {
        let r = row_result(board, j);
        if r != Result::InProgress {
            return r;
        }
    }

    let r = diag_result_1(board);
    if r != Result::InProgress {
        return r;
    }

    let r = diag_result_2(board);
    if r != Result::InProgress {
        return r;
    }

    for i in 0..SIZE {
        for j in 0..SIZE {
            if board[j][i] == 0 {
                return Result::InProgress;
            }
        }
    }

    Result::Draw
}

impl State {
    pub fn new() -> Self {
        let board = [[0; SIZE]; SIZE];
        let result = Result::InProgress;
        State {
            player: Player::One,
            board: board,
            result: result,
            commands_: commands(&board, result),
        }
    }

    pub fn cell(&self, i:usize, j:usize)->u8{
        self.board[j][i]
    }
}

 

impl GameState for State {
    type CommandType = Command;

    fn player(&self) -> Player {
        self.player
    }

    fn result(&self) -> Result {
        self.result
    }

    fn commands(&self) -> &Vec<Self::CommandType> {
        &self.commands_
    }

    fn apply(&self, command: &Self::CommandType) -> Self {
        // These tests can and should be applied one step
        // further up
        assert!(self.result() == Result::InProgress, "game not in progress");
        assert!(self.commands().contains(&command), "bad command");

        // This can't be abstracted up

        let cell: u8 = self.board[command.j][command.i];

        assert!(cell == 0);

        let mut new_board = self.board.clone();

        let p = match self.player() {
            Player::One => 1,
            Player::Two => 2,
        };

        // could use 1 and 4: rowsum of 3 or 12 equals a win?

        new_board[command.j][command.i] = p;

        let result = result(&new_board);
        State {
            player: other_player(self.player()),
            board: new_board,
            result: result,
            commands_: commands(&new_board, result),
        }
    }
}
 

 

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_a() {
        let board = [[0; SIZE]; SIZE];
        let r = result(&board);
        assert_eq!(r, Result::InProgress);
    }
    #[test]
    fn test_b() {
        let board = [[0, 0, 0], [0, 0, 0], [1, 1, 1]];
        let r = result(&board);
        assert_eq!(r, Result::PlayerOne);
    }
    #[test]
    fn test_c() {
        let board = [[1, 2, 1], [2, 1, 2], [1, 0, 0]];
        let r = result(&board);
        assert_eq!(r, Result::PlayerOne);
    }
}
