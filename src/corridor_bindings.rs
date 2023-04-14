use wasm_bindgen::prelude::wasm_bindgen;

use crate::enums::{Player, Result};
use crate::gamestate::GameState;
use crate::node::Node;
use crate::corridor;
use crate::corridor::{Direction, Intersection};

extern crate console_error_panic_hook;
#[wasm_bindgen]
pub struct CorridorGame{
    state: corridor::State
} 

/*javascriptable object */
#[wasm_bindgen]
pub struct CorridorCommand{
    command:corridor::Command
}


#[wasm_bindgen]
pub struct CorridorPlayerState{
    player_state: corridor::PlayerState
}

#[wasm_bindgen]
pub struct CorridorLocation{
    location:corridor::Location
}

#[wasm_bindgen]
impl CorridorLocation{

    #[wasm_bindgen(getter)]
    pub fn i(&self)->usize{
        self.location.i
    }
    #[wasm_bindgen(getter)]
    pub fn j(&self)->usize{
        self.location.j
    }
}
impl From<corridor::Location> for CorridorLocation{
    fn from(l:corridor::Location)->Self{
        CorridorLocation { location: l} 
    }
}

#[wasm_bindgen]
impl CorridorPlayerState{
    #[wasm_bindgen(getter)]
    pub fn walls(&self)->u8{
        self.player_state.walls
    }

    #[wasm_bindgen(getter)]
    pub fn location(&self)->CorridorLocation{
        self.player_state.location.into()

    }
}

impl From<corridor::PlayerState> for CorridorPlayerState{
    fn from(s:corridor::PlayerState)->Self{
        CorridorPlayerState { player_state: s }
    }
}

impl CorridorCommand{
    pub fn as_string(&self)->String{
        "Some corridor command".to_string()
    }
}

impl From<corridor::Command> for CorridorCommand{
    fn from(cmd:corridor::Command)->Self{
        CorridorCommand { 
            command:cmd
         }
    }
}

impl From<CorridorCommand> for corridor::Command{
    fn from(cmd: CorridorCommand)->Self{
        cmd.command
    }
}

#[wasm_bindgen]
pub fn move_command(direction:String)->CorridorCommand{
    let d = match direction.as_str(){
        "up"=>Direction::Up, 
        "down"=>Direction::Down,
        "left"=>Direction::Left,
        "right"=>Direction::Right,
        _=>panic!("bad direction")
    };
    corridor::Command::Move(d).into()
}





#[wasm_bindgen]
impl CorridorGame{
    #[wasm_bindgen(constructor)]
    pub fn new() -> CorridorGame {
        console_error_panic_hook::set_once();
        CorridorGame {
            state: corridor::State::new(),
        }
    }

    pub fn reset(&mut self){
        self.state= corridor::State::new();
    }
    #[wasm_bindgen(getter)]
    pub fn player(&self) -> String {
        match self.state.player() {
            Player::One => "PlayerOne",
            Player::Two => "PlayerTwo",
        }
        .to_string()
    }
    #[wasm_bindgen(getter)]
    pub fn player1(&self)->CorridorPlayerState{
        return self.state.player1.into()
    }
    #[wasm_bindgen(getter)]
    pub fn player2(&self)->CorridorPlayerState{
        return self.state.player2.into()
    }

    #[wasm_bindgen(getter)]
    pub fn result(&self) -> String {
        match self.state.result() {
            Result::InProgress => "InProgress",
            Result::Draw => "Draw",
            Result::PlayerOne => "PlayerOne",
            Result::PlayerTwo => "PlayerTwo",
        }
        .to_string()
    }

    pub fn suggest_move(&self, n: u32) -> CorridorCommand {
        let mut node = Node::new(self.state.clone());

        for _i in 0..n {
            node.mcts();
        }

        self.state.commands()[node.best()].into()
        
    }


    pub fn can_place_hwall(&self, i:usize, j:usize)->bool{
        for command in self.state.commands(){
            match command{
                corridor::Command::HWall(i_,j_ )=>{
                    if(i==*i_ && j==*j_){return true}
                },
                _=>{}
            }
        }
        false
    }
    pub fn can_place_vwall(&self, i:usize, j:usize)->bool{
        for command in self.state.commands(){
            match command{
                corridor::Command::VWall(i_,j_ )=>{
                    if(i==*i_ && j==*j_){return true}
                },
                _=>{}
            }
        }
        false
    }




    pub fn can_move_to(&self,i:usize, j:usize)->bool{
        if self.state.result()!=Result::InProgress{
            return false;
        }
        let current_location = self.state.current_player_state().location ;

        let target = corridor::Location::new(i,j);
          
        for command in self.state.commands(){
            match command{
                corridor::Command::Move(direction)=>{

                    if current_location + *direction==target{return true;}
                },
                corridor::Command::Hop(d1,d2)=>{
                     if current_location+*d1 + *d2 == target{return true;}
                }
                _=>{}
            }
        }
        false
    }

    pub fn hwall_at(&self, i:usize, j:usize)->bool{
        self.state.walls[j][i]==Intersection::Horizontal
    }
    pub fn vwall_at(&self, i:usize, j:usize)->bool{
        self.state.walls[j][i]==Intersection::Vertical
    }

    pub fn move_to_command(&self, i:usize, j:usize)->CorridorCommand{
        let current_location = self.state.current_player_state().location ;
        let target = corridor::Location::new(i,j);
          
        for command in self.state.commands(){
            match command{
                corridor::Command::Move(direction)=>{

                    if current_location + *direction==target{return (*command).into();}
                },
                corridor::Command::Hop(d1,d2)=>{
                     if current_location+*d1 + *d2 == target{return (*command).into();}
                 }
                _=>{}
            }
        }
        panic!("no such command")
    }


    pub fn hwall_command(&self, i:usize, j:usize)->CorridorCommand{
        corridor::Command::HWall(i,j).into()
    }

    pub fn vwall_command(&self, i:usize, j:usize)->CorridorCommand{
        corridor::Command::VWall(i,j).into()
    }


    pub fn apply(&mut self, command:CorridorCommand)->String{
        
        if self.state.result() != Result::InProgress {
            return "Game not in progress".into();
        }


        let cmd : corridor::Command = command.into();

        if !(self.state.commands().contains(&cmd)) {
            return "Bad command".to_string();
        }


        self.state = self.state.apply(&cmd);

        "Ok".into()
    }


}