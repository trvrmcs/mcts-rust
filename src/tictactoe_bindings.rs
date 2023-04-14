use wasm_bindgen::prelude::wasm_bindgen;

use crate::enums::{Player, Result};
use crate::gamestate::GameState;
use crate::node::Node;
use crate::tictactoe;




#[wasm_bindgen]
pub struct TicTacToeGame {
    state: tictactoe::State,
}

#[wasm_bindgen]
pub struct TicTacToeCommand {
    pub i: usize,
    pub j: usize,
}

// idiomatically I'd implement the From trait

#[wasm_bindgen]
impl TicTacToeGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TicTacToeGame {
        TicTacToeGame {
            state: tictactoe::State::new(),
        }
    }

    pub fn reset(&mut self) {
        self.state = tictactoe::State::new();
    }

    pub fn size(&self) -> usize {
        tictactoe::SIZE
    }
    pub fn cell(&self, i: usize, j: usize) -> String {
        match self.state.cell(i, j) {
            0 => "Empty",
            1 => "PlayerOne",
            2 => "PlayerTwo",
            _ => panic!("Bad cell"),
        }
        .to_string()
    }

    pub fn player(&self) -> String {
        match self.state.player() {
            Player::One => "PlayerOne",
            Player::Two => "PlayerTwo",
        }
        .to_string()
    }

    pub fn result(&self) -> String {
        match self.state.result() {
            Result::InProgress => "InProgress",
            Result::Draw => "Draw",
            Result::PlayerOne => "PlayerOne",
            Result::PlayerTwo => "PlayerTwo",
        }
        .to_string()
    }

    pub fn suggest_move(&self, n: u32) -> TicTacToeCommand {
        let mut node = Node::new(self.state.clone());
        for _i in 0..n {
            node.mcts();
        }

        let command = self.state.commands()[node.best()];

        TicTacToeCommand {
            i: command.i,
            j: command.j,
        }
    }

    pub fn apply(&mut self, column: usize, row: usize) -> String {
        if column >= tictactoe::SIZE {
            return "Bad column".to_string();
        }

        if row >= tictactoe::SIZE {
            return "Bad row".to_string();
        }

        if self.state.result() != Result::InProgress {
            return "Game not in progress".to_string();
        }
        let command = tictactoe::Command::new(column, row);

        // not

        if !(self.state.commands().contains(&command)) {
            return "Bad command".to_string();
        }

        self.state = self.state.apply(&command);

        return "Ok".to_string();
    }
}
