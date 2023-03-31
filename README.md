
# Demonstration of Monte Carlo Tree Search

The MCTS engine is built in rust.

Running `docker compose up --build wasm-builder` should compile 
the source code and produce webassembly files in /docs/pkg

The UI is built in ractive/bulma, so javascript 
handles user interaction and display, and rust/wasm handles 
high performance computation.

Two games are provided: connect 4 and tictactoe. The MCTS algorithm
itself has no knowledge of the game mechanics, it can learn to play
ANY game that implements the `GameState` trait:


```rust
pub trait GameState {
    type CommandType: Copy + Debug;

    fn player(&self) -> Player;
    fn result(&self) -> Result;
    fn commands(&self) -> &Vec<Self::CommandType>;
    fn apply(&self, command: &Self::CommandType) -> Self;
}

```


## Build

`docker compose up --build wasm-builder`

Compiles rust source into web assembly, makes available to statit site



## Deploy locally

Just host the files, e.g.

```
python3 -m http.server --directory docs
```





# Performance Notes:

Some rough performance sampling:


## x86 VirtualBox

100000 playouts in 470.533265ms seconds. 

* 212,539/sec

## Chrome/WebAssembly

Ran 100000 playouts in 0.727 seconds

* 137,551/sec

## Firefox/WebAssembly

Ran 100000 playouts in 0.857 seconds

* 116,686/sec

## Python / Numpy 

Ran 1000 playouts in 1.58  seconds
    
* 632/sec



Note that the Rust version still isn't doing the obvious optimization 
of reusing nodes between moves, although this doesn't affect number 
of playouts, but still runs more than 100x faster than my previous Python
algorithm.