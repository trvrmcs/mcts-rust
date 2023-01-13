# CLI 

cargo run 

cargo run --release 



# WASM 

build with 
```
wasm-pack build --target web
```

Run with 

```
python3 -m http.server --directory static
```


# Performance Notes:



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
of playouts.