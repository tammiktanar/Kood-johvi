# smart-road

Welcome to our smart road project! 

Audit questions can be found [here](https://github.com/01-edu/public/tree/master/subjects/smart-road/audit).

### Running
1. Make sure you have [rust installed](https://www.rust-lang.org/tools/install)
2. Open the terminal in our project's root folder
3. Run `cargo run`

On linux you may have to install `libfontconfig1-dev`.
For example like so: `sudo apt install libfontconfig1-dev`

### How it works
We are precalculating every car's entire path as soon as the car is spawned, so there should be 0 chance of collisions.
This is made possible by having cars avoid every car that came before it, but not even knowing about the cars that come after.

Overall, this method did work great, but it was difficult to implement. 
We had to use a recursive function within another recursive function to make it work.