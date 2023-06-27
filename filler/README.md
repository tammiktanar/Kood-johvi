# filler

### Running
1. `cargo install cargo-make`
2. `cargo make init`
3. `cargo make` for debug build or `cargo make -p release` for optimized build
4. `cargo make run`
5. `./solution/run.sh solution/cheekyboi robots/terminator`

You don't need to restart docker when you make changes to the code, you can just run `cargo make` in another terminal, and it'll update the robots.

### Troubleshooting
Q: `cargo make` gives an error about unstable features.  
A: Your version of Rust is outdated. Try running `rustup update` and try again.

Q: `cargo make run` gives an error.  
A: This is probably because you don't have docker running in the background.  

Q: `./solution/run.sh` gives a weird error.  
A: This is likely because git added CRLF line endings to `run.sh`. Open `game_engine/solution/run.sh` and change the line endings to `LF`. There's a button in the lower-right for this in most editors.