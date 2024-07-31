#[derive(Debug)]
pub struct LegoSet {
    code: i32,
    name: String,
    category: String,
    age_minimum: i32,
}

impl LegoSet {
    pub fn new(code: i32, name: String, category: String, age_minimum: i32) -> Self {
        return LegoSet {
            code,
            name,
            category,
            age_minimum,
        };
    }

    pub fn what_is_lego() {
        println!("Lego is a line of plastic contruction toys")
    }
}

impl LegoSet {
    pub fn say_nice() {
        println!("Nice!");
    }
}
