#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        assert_eq!(5 + 14, sum(5, 14))
    }

    #[test]
    fn diff_test() {
        assert_eq!(5 - 14, diff(5, 14))
    }

    #[test]
    fn pro_test() {
        assert_eq!(5 * 14, pro(5, 14))
    }

}



pub fn sum(a: u8 , b: u8) -> u8 {



    return a + b
}

pub fn diff(a: i16, b: i16) -> i16 {



    return a - b
}

pub fn pro(a: i8, b: i8) -> i8 {



    return a * b
}

pub fn quo(a: f32, b: f32) -> f32 {




    return a / b
}

pub fn rem(a: f32, b: f32) -> f32 {



    return a % b
}



