use wasm_bindgen::prelude::wasm_bindgen;

use crate::enums::{Player, Result};
use crate::gamestate::GameState;
use crate::node::Node;
use crate::connect4;


#[wasm_bindgen]
pub struct Connect4Game {
    state: connect4::State,
}

#[wasm_bindgen]
impl Connect4Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Connect4Game {
        Connect4Game {
            state: connect4::State::new(),
        }
    }

    pub fn reset(&mut self) {
        self.state = connect4::State::new();
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

    pub fn suggest_move(&self, n: u32) -> usize {
        let mut node = Node::new(self.state.clone());

        for _i in 0..n {
            node.mcts();
        }

        self.state.commands()[node.best()].column
    }

    pub fn apply(&mut self, column: usize) -> String {
        if column >= 7 {
            return "Bad column".to_string();
        }
        if self.state.result() != Result::InProgress {
            return "Game not in progress".to_string();
        }
        let command = connect4::Command::new(column);

        if !(self.state.commands().contains(&command)) {
            return "Bad command".to_string();
        }
        self.state = self.state.apply(&command);

        return "Ok".to_string();
    }
}