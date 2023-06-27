use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn mean(list: &Vec<i32>) -> f64 {
    let mut sum: i32 = 0;
    for nr in list {
        sum += nr;
    }

    return sum as f64 / list.len() as f64;
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut numbers:Vec<i32> = list.to_vec();
    numbers.sort();
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid - 1], numbers[mid]]) as i32
    } else {
        numbers[mid]
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    // new HashMap
    let mut times = HashMap::new();

    // count
    for x in list {
        let cnt = times.entry(*x as usize).or_insert(0);
        *cnt += 1;
    }

    let mut best: (i32, i32) = (*times.iter().nth(0).expect("Fatal.").0 as i32, *times.iter().nth(0).expect("Fatal.").1 as i32);

    for x in times.iter() {
        if *x.1 > best.1 {
            best = (*x.0 as i32, *x.1);
        }
    }
    
    return best.0
}