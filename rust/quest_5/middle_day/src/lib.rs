#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub use chrono::*;
pub type wd = Weekday;



pub fn middle_day(given_year: i32) -> Option<wd> {
    let start = NaiveDate::from_ymd(given_year, 1, 1);
    let end = NaiveDate::from_ymd(given_year + 1, 1, 1);

    let days = (end - start).num_days();

    if days % 2 == 0 {
        return None
    } 

    Some(NaiveDate::from_yo(given_year, days as u32 / 2 + 1).weekday())
}