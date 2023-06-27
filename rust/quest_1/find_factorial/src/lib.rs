#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn factorial(num: u64) -> u64 {
    let mut res = 1;
    let mut cur_nr = 1;

    loop {
        if cur_nr > num {

            break
        } else {
            res*= cur_nr;
            cur_nr+= 1;
        }
    }

    return res
}