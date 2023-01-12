// don't really understand this syntax yet.
mod connect4;
mod enums;
mod gamestate;
mod node;
mod tictactoe;
use std::time::{Duration, SystemTime};

use crate::enums::Result;

use crate::node::Node;

// use crate
use crate::gamestate::GameState;

///use crate::tictactoe::{State, get_command,Command};
use crate::connect4::{get_command, Command, State};

fn play_game() {
    let mut s = State::new();

    println!("The Board state is: {}", s);
    println!("{:?} available commands", s.commands().len());

    while s.result() == Result::InProgress {
        let command = &s.commands()[0];
        println!("Apply command {:?}", command);
        s = s.apply(command);

        println!("The Board state is: {}", s);

        println!("{:?} available commands", s.commands().len());
    }
}

fn learn() {
    let s = State::new();

    let mut n = Node::new(s);

    println!("Node is {}", n);

    for _n in 0..1000000 {
        n.mcts();
    }

    println!("Node is {}", n);
    show_best_line(n);
}

fn pick_move(state: &State) -> Command {
    let mut node = Node::new(state.clone());

    let then = SystemTime::now() + Duration::from_millis(1500);
    while SystemTime::now() < then {
        node.mcts();
    }

    println!("Node is {}", node);


    let v: Vec<usize> = node.best_line().into_iter().map(|c| c.column).collect();

    println!("Best line is {:?}",v);
    state.commands()[node.best()]

    // state.commands()[0]
}

fn play() {
    let mut state = State::new();

    println!("{}", state);
    loop {
        let human = get_command(&state);

        println!("You chose {}", human);

        /*TODO: reuse existing child node if applicable*/

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
    // play_game();
    // learn();
    play();
}

fn show_best_line(node: Node<State>) {
    println!("Current best line is:");
    let line = node.best_line();

    let mut s = State::new();
    println!("{}", s);

    for c in line {
        println!("{}", c);

        s = s.apply(&c);
        println!("{}", s);
    }
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
