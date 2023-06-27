use std::io;

fn main() {
    let mut end_program = false;
    let mut tries = 0;


    loop {
        if end_program == true {
            break
        } else {
            tries+=1;
            println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

            let mut user_input = String::new();
            match io::stdin().read_line(&mut user_input) {
                Ok(_) => {}
                Err(error) => println!("error: {error}"),
            }

            if user_input == "The letter e\n" {
                end_program = true;
            }
        }
    }

    if tries == 1 {
        println!("It took you {} trial to get the right answer", tries);
    } else {
        println!("It took you {} trials to get the right answer", tries);
    }
}
