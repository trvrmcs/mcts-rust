use std::fmt;

use rand::seq::SliceRandom;

use crate::enums::{other_player, Player, Result};
use crate::gamestate::GameState;
use rand::thread_rng;

use std::collections::BTreeMap;

pub struct Node<S: GameState + Clone > {
    state: S,
    children: BTreeMap<usize, Node<S>>,
    playouts: u32,
    wins: f32, // half a point for a draw
}

impl<S: GameState + Clone > Node<S> {
    pub fn new(state: S) -> Self {
        let children = BTreeMap::new();

        Self {
            state: state,
            playouts: 0,
            wins: 0.0,
            children: children,
        }
    }

    fn no_commands(&self) -> bool {
        self.state.commands().len() == 0
    }
    fn unexplored_commands(&self) -> bool {
        &self.children.len() < &self.state.commands().len()
    }

    fn ratio(&self) -> f32 {
        if self.wins == 0.0 {
            return 0.0;
        }
        return self.wins / (self.playouts as f32);
    }

    fn sanity(&self) {
        if self.unexplored_commands() {
            let r = self.state.result();
            if r != Result::InProgress {
                let c = self.state.commands();
                println!(
                    "Result:{:?},   commands:{} ",
                    r,
                    // self.state,
                    c.len()
                );

                println!("Commands: {:?}", c);
                panic!("no sanity");
            }
        }
    }

    fn uct_score(&self, parent_playouts: u32) -> f32 {
        /*
        UCT is 'Upper Confidence Bound Applied to Trees'.

        See https://en.wikipedia.org/wiki/Monte_Carlo_tree_search#Exploration_and_exploitation
        and
        https://medium.com/@quasimik/monte-carlo-tree-search-applied-to-letterpress-34f41c86e238
        */
        assert!(parent_playouts > 0);
        let playouts = self.playouts as f32;

        // explore-vs-exploit parameter
        let c: f32 = (2.0 as f32).sqrt();

        let explore = self.wins / playouts;
        let exploit = c * ((parent_playouts as f32).ln() / playouts).sqrt();

        explore + exploit
    }

    fn child_with_highest_uct_score(&mut self) -> usize {
        assert!(self.children.len() > 0);
        let mut best_score = 0.0;
        let mut best_index = 0;
        for (i, child) in &self.children {
            let score = child.uct_score(self.playouts);
            if score > best_score {
                best_score = score;
                best_index = *i;
            }
        }
        best_index
    }

    pub fn best(&self) -> usize {
        /*
            Wikipedia says
            "the move with the most simulations made (i.e. the highest denominator)
            is chosen as the final answer."
        */
        assert!(self.children.len() > 0);
        let mut best = 0;
        let mut most_playouts = 0;

        for (i, child) in self.children.iter() {
            let playouts = child.playouts;
            if playouts > most_playouts {
                best = *i;
                most_playouts = playouts;
            }
        }
        best
    }

 

    pub fn playout(&self) -> Result {
        if self.state.result() != Result::InProgress {
            return self.state.result();
        }
        let mut state = self.state.clone();
        let mut rng = thread_rng();

        while state.result() == Result::InProgress {
            let command = state.commands().choose(&mut rng).unwrap();

            state = state.apply(command);
        }

        state.result()
    }

    pub fn update_node(&mut self, result: Result) {
        assert!(result != Result::InProgress);
        // the player who just played the move that got us to this state
        let player = other_player(self.state.player());

        if result == Result::PlayerOne && player == Player::One {
            self.wins += 1.0;
        }

        if result == Result::PlayerTwo && player == Player::Two {
            self.wins += 1.0;
        }

        if result == Result::Draw {
            self.wins += 0.5;
        }

        self.playouts += 1;
    }


     
    fn select_command_index(&self)->usize {
        assert!(self.state.result()==Result::InProgress);

        for i in 0..self.state.commands().len(){
            if !(&self.children.contains_key(&i)){
                return i
                }
            }
        panic!("no available unexplored commands");
    }

   

    fn expand(&mut self) -> Result {
        assert!(self.state.result() == Result::InProgress);
        let index = self.select_command_index();
        let command = &self.state.commands()[index];
        let new_state = self.state.apply(command);
        let mut child = Self::new(new_state);
        let child_result = child.playout();
        child.update_node(child_result);
        self.children.insert(index, child);
        child_result
    }

    fn explore_exploit(&mut self) -> Result {
        let index: usize = self.child_with_highest_uct_score();
        let child = self.children.get_mut(&index).unwrap();
        child.mcts()
    }

    pub fn size(&self) -> usize {
        /*
        1 + sum of childrens size()
        */
        let mut s = 1;
        for (_, c) in &self.children {
            s += c.size();
        }
        s
    }
    pub fn mcts(&mut self) -> Result {
        self.sanity();
        let result = if self.unexplored_commands() {
            self.expand()
        } else if self.no_commands() {
            self.playout()
        } else {
            self.explore_exploit()
        };

        self.update_node(result);
        result
    }
}

// impl<S: GameState + Clone + Display> fmt::Display for Node<S> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "Node(playouts={}, ratio={}, nodes={})",
//             self.playouts,
//             self.ratio(),
//             self.size()
//         )
//     }
// }
