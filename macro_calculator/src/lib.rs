use json::{ object};

pub struct Food {
    // expected public fields
    pub name: String,
    pub calories: (String, String),
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut proteins = 0.0;
    let mut fats = 0.0;
    let mut carbs = 0.0;
    let mut cals = 0.0;

    for foo in foods {
        let cal_str: String = foo.calories.1.chars().take_while(|c| c.is_digit(10) || *c == '.').collect();
        proteins += foo.nbr_of_portions * foo.proteins;
        fats += foo.nbr_of_portions * foo.fats;
        carbs += foo.nbr_of_portions * foo.carbs;
        cals += foo.nbr_of_portions * cal_str.parse::<f64>().unwrap()
    }


    object! {
        cals:  (cals * 100.0).round() / 100.0,
        carbs:   (carbs * 100.0).round() / 100.0,
        proteins:   (proteins * 100.0).round() / 100.0 ,
        fats:   (fats * 100.0).round() / 100.0 ,
    }
}
