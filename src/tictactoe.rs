use core::result;
use std::fmt;
use std::io;
use std::io::Write; // <--- bring flush() into scope

use crate::enums::{other_player, Player, Result};
use crate::gamestate::GameState;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Command {
    i: usize,
    j: usize,
}

const SIZE: usize = 3;
impl Command {
    fn new(i: usize, j: usize) -> Command {
        assert!(i < SIZE);
        assert!(j < SIZE);
        Command { i, j }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.i, self.j)
    }
}

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
    let mut c = vec![];

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
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.board {
            write!(f, "\n")?;
            for cell in row {
                if *cell == 0 {
                    write!(f, ".")?
                }
                if *cell == 1 {
                    write!(f, "O")?
                }
                if *cell == 2 {
                    write!(f, "X")?
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

pub fn str_to_command(s: &String) -> result::Result<Command, &'static str> {
    let vec: Vec<&str> = s.split_whitespace().collect();

    if vec.len() != 2 {
        return Err("Enter 2 numbers.");
    }

    match vec[0].parse::<usize>() {
        Ok(i) => match vec[1].parse::<usize>() {
            Ok(j) => {
                if i < 3 && j < 3 {
                    Ok(Command::new(i, j))
                } else {
                    Err("Pick numbers between 0 and 2")
                }
            }
            Err(what) => Err("Can't parse j"),
        },
        Err(what) => return Err("Can't parse i"),
    }
}

pub fn get_command(state: &State) -> Command {
    loop {
        print!("Command> ");
        io::stdout().flush().unwrap();

        let mut str_command = String::new();

        io::stdin()
            .read_line(&mut str_command)
            .expect("failed to readline");

        match str_to_command(&str_command) {
            Ok(command) => return command,
            Err(why) => println!("{}", why),
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
