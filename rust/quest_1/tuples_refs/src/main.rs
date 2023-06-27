use tuples_refs::*;

fn main() {
	let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
	println!("Student: {:?}", student);
	println!("Student first name: {}", first_name(&student));
	println!("Student last name: {}", last_name(&student));
	println!("Student Id: {}", id(&student));
}