#[derive(Debug)]
pub struct Color(i32, i32, i32);

impl Color {
    pub fn red() -> Self {
        Self(255, 0, 0)
    }
}

#[derive(Debug)]
pub struct Car {
    brand: String,
    model: String,
    manufacture_year: i32,
}

impl Car {
    pub fn new(brand: String, model: String) -> Self {
        Self {
            brand,
            model,
            manufacture_year: 0,
        }
    }

    pub fn info(&self) -> String {
        if self.manufacture_year == 0 {
            format!("{} model {}", self.brand, self.model)
        } else {
            format!(
                "{} model {}, manufactured at {}",
                self.brand, self.model, self.manufacture_year
            )
        }
    }

    pub fn congratulate(&self, name: String) {
        println!("hello {}", name);
        println!("congrats with you new car {}", self.info());
        println!("whoosh whoos!");
    }

    pub fn set_manufacture_year(&mut self, year: i32) {
        self.manufacture_year = year;
    }
}
