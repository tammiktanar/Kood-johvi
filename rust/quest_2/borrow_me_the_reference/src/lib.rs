#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn delete_and_backspace(s: &mut String) {
    println!("{}", s);
    let res = remove_strings_proper(remove_strings_proper(s, '+'), '-').to_string();
    

    println!("{}", res);
    *s = res
}

pub fn remove_strings_proper(s: &mut String, which: char) -> &mut String{
    let mut res = String::new();

    let mut nr = 0;
    let submitted_text = s;

    if which == '-' {
        *submitted_text = submitted_text.chars().rev().collect::<String>();
    }

    for cha in submitted_text.chars() {
        if cha != which {
            if nr <= 0 {
                res.push(cha);
            } else {
                nr-=1;
            }
        } else {
            nr+= 1;
        }
    }

    if which == '-' {
        res = res.chars().rev().collect::<String>();
    }

    *submitted_text = res;

    return submitted_text
}




pub fn is_correct(v: &mut Vec<&str>) -> usize {
    for string in v.iter_mut() {
        let mut split_string = string.split('=');

        let temp_val_1 = split_string.next().unwrap();
        let temp_val_2 = split_string.next().unwrap();

        let val_1 = meval::eval_str(temp_val_1).unwrap();
        let val_2 = meval::eval_str(temp_val_2).unwrap();


        let mut res = "";

        if val_1 == val_2 {
            res = "✔";
        } else {
            res = "✘";
        }

        *string = res
    };



    return (v.iter().filter(|&n| *n == "✔").count() * 100) / v.len()
}
