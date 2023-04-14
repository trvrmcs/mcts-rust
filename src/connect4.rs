use crate::enums::{other_player, Player, Result};
use crate::gamestate::GameState;

use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Command {
    pub column: usize,
}
impl Command {
    pub fn new(column: usize) -> Command {
        assert!(column < 7);
        Command { column }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.column)
    }
}

type BoardType = [[u8; 7]; 6];

#[derive(Clone)]
pub struct State {
    player: Player,
    board: BoardType,
    result: Result,
    commands_: Vec<Command>,
}

impl State {
    pub fn new() -> Self {
        let board = [[0; 7]; 6];
        let result = Result::InProgress;
        State {
            player: Player::One,
            board: board,
            result: result,
            commands_: commands(&board, result),
        }
    }

    pub fn cell(&self, i: usize, j: usize) -> u8 {
        self.board[j][i]
    }
}

fn commands(board: &BoardType, result: Result) -> Vec<Command> {
    let mut c = vec![];
    if result != Result::InProgress {
        return c;
    }

    for column in 0..7 {
        if board[5][column] == 0 {
            c.push(Command { column });
        }
    }
    c
}

fn column_result(board: &BoardType, i: usize, j: usize) -> Result {
    assert!(i < 7);
    assert!(j < 3);

    let first: u8 = board[j][i];
    if first == 0 {
        return Result::InProgress;
    }

    for jj in j..j + 4 {
        if (board[jj][i]) != first {
            return Result::InProgress;
        }
    }

    match first {
        1 => Result::PlayerOne,
        2 => Result::PlayerTwo,
        _ => Result::InProgress,
    }
}

fn row_result(board: &BoardType, i: usize, j: usize) -> Result {
    assert!(i < 4);
    assert!(j < 6);

    let first = board[j][i];
    if first == 0 {
        return Result::InProgress;
    }

    for ii in i..i + 4 {
        if board[j][ii] != first {
            return Result::InProgress;
        }
    }
    match first {
        1 => Result::PlayerOne,
        2 => Result::PlayerTwo,
        _ => Result::InProgress,
    }
}

fn up_diag_result(board: &BoardType, i: usize, j: usize) -> Result {
    assert!(i < 4);
    assert!(j < 3);

    let first = board[j][i];
    if first == 0 {
        return Result::InProgress;
    }

    for delta in 1..4 {
        let ii = i + delta;
        let jj = j + delta;
        if board[jj][ii] != first {
            return Result::InProgress;
        }
    }
    match first {
        1 => Result::PlayerOne,
        2 => Result::PlayerTwo,
        _ => Result::InProgress,
    }
}

fn down_diag_result(board: &BoardType, i: usize, j: usize) -> Result {
    assert!(i < 4);
    assert!(j > 2);
    assert!(j < 6);

    let first = board[j][i];
    if first == 0 {
        return Result::InProgress;
    }

    for delta in 1..4 {
        let ii = i + delta;
        let jj = j - delta;
        if board[jj][ii] != first {
            return Result::InProgress;
        }
    }
    match first {
        1 => Result::PlayerOne,
        2 => Result::PlayerTwo,
        _ => Result::InProgress,
    }
}

fn row_results(board: &BoardType) -> Result {
    for i in 0..4 {
        for j in 0..6 {
            let r = row_result(board, i, j);
            if r != Result::InProgress {
                return r;
            }
        }
    }
    return Result::InProgress;
}

fn up_diag_results(board: &BoardType) -> Result {
    for i in 0..4 {
        for j in 0..3 {
            let r = up_diag_result(board, i, j);
            if r != Result::InProgress {
                return r;
            }
        }
    }
    return Result::InProgress;
}

fn down_diag_results(board: &BoardType) -> Result {
    for i in 0..4 {
        for j in 3..6 {
            let r = down_diag_result(board, i, j);
            if r != Result::InProgress {
                return r;
            }
        }
    }
    return Result::InProgress;
}
fn column_results(board: &BoardType) -> Result {
    for i in 0..7 {
        for j in 0..3 {
            let r = column_result(board, i, j);
            if r != Result::InProgress {
                return r;
            }
        }
    }
    return Result::InProgress;
}

fn result(board: &BoardType) -> Result {
    let r = row_results(board);
    if r != Result::InProgress {
        return r;
    }

    let r = column_results(board);
    if r != Result::InProgress {
        return r;
    }

    let r = up_diag_results(board);
    if r != Result::InProgress {
        return r;
    }

    let r = down_diag_results(board);
    if r != Result::InProgress {
        return r;
    }

    for i in 0..7 {
        if board[5][i] == 0 {
            return Result::InProgress;
        }
    }

    /*
        TODO: diagonal rows
    */
    Result::Draw
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for j in (0..6).rev() {
            write!(f, "\n")?;
            for i in 0..7 {
                let cell = self.board[j][i];

                if cell == 0 {
                    write!(f, ".")?
                }
                if cell == 1 {
                    write!(f, "R")?
                }
                if cell == 2 {
                    write!(f, "Y")?
                }
            }
        }
        write!(f, "\n{:?}", &self.result())
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
        assert!(self.result() == Result::InProgress, "game not in progress");
        assert!(self.commands().contains(&command), "bad command");

        let i = command.column;

        assert!(self.board[5][i] == 0);

        let mut j = 0;

        for jj in 0..6 {
            if self.board[jj][i] == 0 {
                j = jj;
                break;
            }
        }

        assert!(self.board[j][i] == 0);

        let mut new_board = self.board.clone();
        new_board[j][i] = match self.player() {
            Player::One => 1,
            Player::Two => 2,
        };

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
    use super::*;

    #[test]
    fn test_a() {
        let mut s = State::new();

        assert!(commands(&s.board, Result::InProgress).len() == 7);

        let command = &s.commands()[0];

        assert!(command == &Command { column: 0 });

        assert!(s.cell(0, 0) == 0);
        s = s.apply(command);

        assert!(s.cell(0, 0) == 1);

        println!("WHAT: {}", s);
        println!("board: {:?}", s.board);
        println!("board: {:?}", s.board);

        assert!(s.board[0][0] == 1);

        assert!(commands(&s.board, Result::InProgress).len() == 7);
    }
}
