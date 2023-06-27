pub use chrono::{Utc, NaiveDate};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FErr {
    // expected public fields
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FErr {
    pub fn new(name: String, error: String, err: String) -> FErr {
        Self{
            form_values: (name, error),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    // expected public fields
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub fav_colour: Color,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        fav_colour: Color,
        birth_location: String,
        password: String,

    ) -> Form {
        Self {
            first_name: first_name,
            last_name: last_name, 
            birth: birth, 
            fav_colour: fav_colour, 
            birth_location: birth_location, 
            password: password
        }
    }
    
    pub fn validate(&self) -> Result<Vec<&str>, FErr> {

        if self.first_name.is_empty() {
            return Err(
                FErr::new(
                    "first_name".to_string(),
                    self.first_name.clone(),
                    "No user name".to_string(),
            ));
        }

        if self.password.len() < 8 {
            return Err(
                FErr::new(
                    "password".to_string(),
                    self.password.clone(),
                    "At least 8 characters".to_string(),
            ));
        }

        if !self.password.chars().any(|ch| ch.is_alphabetic()) || !self.password.chars().any(|ch| !ch.is_alphanumeric()) || !self.password.chars().any(|ch| ch.is_numeric()) {
            return Err(
                FErr::new(
                    "password".to_string(),
                    self.password.clone(),
                    "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string(),
            ));
        }




        Ok(vec![
            "Valid first name",
            "Valid password"
        ])
    }
}