use simple_hash::*;
use std::collections::HashMap;

fn main() {
    let mut hash: HashMap<&str, i32> = HashMap::new();
    hash.insert("Daniel", 122);
    hash.insert("Ashley", 333);
    hash.insert("Katie", 334);
    hash.insert("Robert", 14);

    println!(
        "Does the HashMap contains the name Roman? => {}",
        contain(&hash.clone(), "Roman")
        //----------^^^^^^^^
        // this is not correct, fix it to match the solution the expected function
    );
    println!(
        "Does the HashMap contains the name Katie? => {}",
        contain(&hash.clone(), "Katie")
        //----------^^^^^^^^
        // this is not correct, fix it to match the solution the expected function
    );
    println!("Removing Robert {:?}", remove(hash.clone(), "Robert"));
    println!(
        "Does the HashMap contains the name Robert? => {}",
        contain(&hash.clone(), "Robert")
        //----------^^^^^^^^
        // this is not correct, fix it to match the solution the expected function
    );
    println!("Hash {:?}", &hash);
}
