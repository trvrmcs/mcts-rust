use wasm_bindgen::prelude::*;
mod connect4;
mod enums;
mod gamestate;
mod node;
// use std::time::{Duration, SystemTime};
// use crate
use crate::gamestate::GameState;

use crate::enums::{Player, Result};

use crate::node::Node;
///use crate::tictactoe::{State, get_command,Command};
use crate::connect4::{get_command, Command, State};

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct Connect4Game{
    state:State
    /*
        Would be better if this owned a Node, I guess.
    */

}


#[wasm_bindgen]
impl Connect4Game{

    #[wasm_bindgen(constructor)]
    pub fn new()->Connect4Game{
        Connect4Game{
            state:State::new()
        }
    } 

    pub fn reset(&mut self){
    
        self.state=State::new();
    }
  
    
    pub fn cell(&self, i:usize, j:usize)->String{
        match self.state.cell(i,j){
            0=>"Empty",
            1=>"PlayerOne",
            2=>"PlayerTwo",
            _=>panic!("Bad cell")
        }.to_string()
    }

    pub fn player(&self)->String{
        match self.state.player() {
            Player::One => "PlayerOne",
            Player::Two => "PlayerTwo",
        }.to_string()
    }

    pub fn result(&self)->String{
        match self.state.result(){
            Result::InProgress=>"InProgress",
            Result::Draw=>"Draw",
            Result::PlayerOne=>"PlayerOne",
            Result::PlayerTwo=>"PlayerTwo"
        }.to_string()
    }
 


    pub fn suggest_move(&self, n:u32)->usize{
        let mut node = Node::new(self.state.clone());


        for i in 0..n
        {
            node.mcts();
        }

        self.state.commands()[node.best()].column

    }

    pub fn apply(&mut self, column:usize)->String{

        if column>=7{
            return "Bad column".to_string();
        }
        if self.state.result()!=Result::InProgress{
            return "Game not in progress".to_string();
        }
        let command = Command::new(column);

        self.state=self.state.apply(&command);

        return "Ok".to_string()


    }

    


}


/*

What does the interface look like:

* create an engine
* request a new game (perhaps set difficulty)
* ask for a list of moves 
* send a move 
* get current state, in a form that's meaningful
  to javascript, or better yet, Ractive
* ask for a computer move 


*/