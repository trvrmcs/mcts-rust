use crate::enums::{Player, Result};
use std::fmt::Debug;

pub trait GameState {
    type CommandType: Copy + Debug;

    fn player(&self) -> Player;
    fn result(&self) -> Result;
    fn commands(&self) -> &Vec<Self::CommandType>;
    fn apply(&self, command: &Self::CommandType) -> Self;
}
