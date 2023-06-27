#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn nbr_function(nr: i32) -> (i32, f64, f64) {
    let first_val = nr;
    let f_nr = nr as f64;
    let second_val = f_nr.exp();
    let third_val = f_nr.abs().ln();

    return (first_val, second_val, third_val)
}

pub fn str_function(a: String) -> (String, String) {
    let nrs = a.split(" ");
    let mut res = String::new();
    let mut first = true;
    for nr in nrs {
        if !first {
            res.push_str(" ");
        } else {
            first = false
        }
        let cur_nr: f64 = nr.parse().unwrap();
        res.push_str(&cur_nr.exp().to_string());
    }

    return (a, res)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res = Vec::new();

    for nr in &b {
        res.push((*nr as f64).abs().ln());
    }


    return (b, res)
}
