#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fahrenheit_test() {
        assert_eq!(95.0, celsius_to_fahrenheit(35.0))
    }
    #[test]
    fn celsius_test() {
        assert_eq!(35.0, fahrenheit_to_celsius(95.0))
    }
    #[test]
    fn special_case() {
        assert_eq!(-6.666666666666666, fahrenheit_to_celsius(20.0))
    }
}




pub fn fahrenheit_to_celsius(f: f64) -> f64 {


    (f - 32.0)  / (9.0 / 5.0)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {


    (c * 9.0 / 5.0) + 32.0
}