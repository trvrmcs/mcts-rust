// don't really understand this syntax yet.
mod connect4;
mod enums;
mod gamestate;
mod node;
mod tictactoe;
use std::time::{/*Duration, */SystemTime};

use std::io;
use std::io::Write; //bring flush() into scope

use crate::enums::Result;

use crate::node::Node;

// use crate
use crate::gamestate::GameState;

///use crate::tictactoe::{State, get_command,Command};
use crate::connect4::{str_to_command, Command, State};


fn pick_move(state: &State)->Command{
    let mut node = Node::new(state.clone());

    let start = SystemTime::now();
 //  let then = SystemTime::now() + Duration::from_millis(1500); 

    // while SystemTime::now() < then {
    let n = 100000;
    for _i in 0..n{
        node.mcts();
    }

    let duration = start.elapsed().unwrap();

    let v: Vec<usize> = node.best_line().into_iter().map(|c| c.column).collect();

    
    println!("Ran {} playouts in {:?}.",n, duration);
    println!("Best line is {:?}",v);
    state.commands()[node.best()]

    
}


pub fn get_human_command() -> Command {
    /*
    could be in main.rs
    */
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


fn play() {
    let mut state = State::new();

    println!("{}", state);
    loop {
        let human = get_human_command();

        println!("You chose {}", human);

        /*TODO reuse existing child node if applicable*/

        state = state.apply(&human);

        println!("{}", state);
        if state.result() != Result::InProgress {
            break;
        }

        let computer = pick_move(&state);

        println!("Computer plays {}", computer);

        state = state.apply(&computer);
        println!("{}", state);
        if state.result() != Result::InProgress {
            break;
        }
    }
}

fn main() {
    play();
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let s = State::new();

        let n = Node::new(s);

        let result = n.playout();

        assert!(result != Result::InProgress);
    }
}
