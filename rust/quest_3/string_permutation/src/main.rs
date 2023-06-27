use string_permutation::*;

fn main() {

	println!("{}", is_permutation("thought", "thougth")); // TRUE
    println!("{}", is_permutation("♥", "♥♥")); // FALSE
    println!("{}", is_permutation("cde", "edbca")); // FALSE
}