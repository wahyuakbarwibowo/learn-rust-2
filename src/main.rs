mod lego;
mod model;

fn main() {
    // let rough_terrain_crane = lego::LegoSet {
    //     code: 42082,
    //     name: String::from("Rough Terrain Crane"),
    //     age_minimum: 11,
    //     category: String::from("Technic"),
    // };

    // println!("{:#?}", rough_terrain_crane);
    // println!("{}", rough_terrain_crane.name);
    // println!("{}", rough_terrain_crane.age_minimum);
    // println!("{}", rough_terrain_crane.category);
    // println!("{}", rough_terrain_crane.code);

    // define using assocatin function
    let rough_terrain_crane = lego::LegoSet::new(
        42082,
        String::from("Rough Terrain Crane"),
        String::from("Technic"),
        11,
    );

    println!("{:#?}", rough_terrain_crane);

    // with associated function
    let harley = lego::LegoSet::new(
        22415,
        String::from("Harley Prada"),
        String::from("Hyper"),
        17,
    );

    println!("{:#?}", harley);

    lego::LegoSet::what_is_lego();
    lego::LegoSet::say_nice();

    let red_color = model::Color::red();
    println!("{:#?}", red_color);

    let mut car: model::Car = model::Car::new(String::from("Toyoya"), String::from("Avanza"));
    println!("car: {:#?}", car);

    let info = car.info();
    println!("info: {:?}", info);

    car.congratulate(String::from("Josh"));

    car.set_manufacture_year(2045);
    let detailed_info = car.info();
    println!("detailed info: {:?}", detailed_info);

    let msg1 = String::from("neh");
    let msg2 = msg1;
    println!("{}, {}", msg1, msg2);
}
