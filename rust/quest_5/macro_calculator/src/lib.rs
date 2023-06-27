#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub use json::{parse, stringify};

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros (foods: Vec<Food>) -> json::JsonValue {
    let mut calories:f64 = 0.0;
    let mut fats:f64 = 0.0;
    let mut carbs:f64 = 0.0;
    let mut proteins:f64 = 0.0;

    for food in foods {
        calories += (food.calories[1].strip_suffix("kcal").unwrap().parse::<f64>().unwrap() * food.nbr_of_portions) as f64 ;
        fats += food.fats * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
    }
    
    return json::object! {
        "cals": (calories*100.0).round()/100.0,
        "carbs": (carbs*100.0).round()/100.0,
        "proteins": (proteins*100.0).round()/100.0,
        "fats": (fats*100.0).round()/100.0,
    };
}