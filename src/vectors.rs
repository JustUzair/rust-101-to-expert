pub fn test_int_vector() {
    let mut int_vec: Vec<u8> = Vec::new();
    int_vec.push(1);
    int_vec.push(2);
    int_vec.push(3);
    println!("Int Vector : {:?}", int_vec);
}

pub fn test_string_vec() {
    let string_vec = vec!["ABC", "DEF", "GHI"];
    println!("String Vector : {:?}", string_vec);
}

struct Car {
    model: String,
    year: u16,
}

pub fn test_struct_vec() {
    let car1 = Car {
        model: String::from("Toyota"),
        year: 2010,
    };
    let car2 = Car {
        model: String::from("Honda"),
        year: 2015,
    };
    let car3 = Car {
        model: String::from("Ford"),
        year: 2020,
    };

    let car_vec = vec![car1, car2, car3];
    for car in car_vec.iter() {
        println!("Car Model : {}, Year : {}", car.model, car.year);
    }
}
