use borrow_me_the_reference::{delete_and_backspace, is_correct};

fn main() {
	let mut a = String::from("bpp--o+er+++sskroi-++lcw");
	let mut b: Vec<&str> = vec!["2+2=4", "3+2=5", "10-3=3", "5+5=10"];

	// - If a value does **not implement Copy**, it must be **borrowed** and so will be passed by **reference**.
	delete_and_backspace(&mut a); // the reference of  the value
	let per = is_correct(&mut b); // the reference of  the value

	println!("{:?}", (a, b, per));
	// output: ("borrow", ["✔", "✔", "✘", "✔"], 75)
}