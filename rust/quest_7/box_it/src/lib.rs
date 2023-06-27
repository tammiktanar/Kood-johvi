#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut res = Vec::new();

    for nr in s.split(" ") {
        print!("{}", nr);
        if nr.contains('k') {
            let mut temp = *&nr.replace('k', "").parse::<f32>().unwrap();
            temp *= 1000.0;
            res.push(temp as u32);
        } else {
            res.push(nr.parse::<u32>().unwrap());
        }        
    }
    return Box::new(res);
}



pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}