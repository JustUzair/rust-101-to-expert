pub fn test_match_int() {
    let myage:u16 = 23;
    match myage {
        0 => println!("I am a new born"),
        1..=12 => println!("I am a child"),
        13..=19 => println!("I am a teenager"),
        20..=35 => println!("I am a young adult"),
        36..=60 => println!("I am a middle aged adult"),
        61..=100 => println!("I am a senior citizen"),
        _ => println!("I am not sure about my age")
    }
}