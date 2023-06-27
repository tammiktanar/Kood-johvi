#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}



pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {

    return km_h / 3.6    
}