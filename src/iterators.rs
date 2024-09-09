pub fn test_rust_iterators() {
    let fruits_list = vec![
        "Strawberry",
        "Banana",
        "Mango",
        "Apple",
        "Orange",
        "Grapes",
        "Pineapple",
    ];
    let nuts_list = vec![
        "Almond",
        "Peanut",
        "Cashew",
        "Pistachio",
        "Walnut",
        "Hazelnut",
        "Pecan",
    ];

    let fruit_iter = fruits_list.iter();

    println!("\n\nEating fruits\n");
    for fruit in fruit_iter {
        println!("Eating : {}", fruit);
    }

    // Chain two iterators - chain()
    let aggreagate_foods_list = fruits_list.iter().chain(nuts_list.iter());
    // Create a collection from multiple iterators - collect()
    let all_foods: Vec<&&str> = aggreagate_foods_list.collect();
    let all_foods_iter = all_foods.iter();

    println!("\n\nAll foods : {:?}", all_foods);

    // Altering the contents of the iterators using closures
    println!("\n\nAfter mutating the contents of iterator\n",);

    let fruits_list_strings = fruits_list.iter().map(|fruit| String::from(*fruit));
    // println!("Fruits : {:?}", fruits_list_strings);
    let new_fruits_list = fruits_list_strings.map(|mut fruit| {
        fruit.push_str(" <=== FRUIT 🍒");
        return fruit;
    });
    new_fruits_list
        .clone()
        .for_each(|fruit| println!("{}", fruit));

    println!(
        "\n\nLast iter item \n{}",
        new_fruits_list.clone().last().unwrap()
    );

    // Printing nth element of the iterator
    println!("\n\nPrinting nth element of the iterator\n");
    all_foods_iter
        .clone()
        .step_by(2)
        .for_each(|fruit| println!("{}", fruit));

    // Zipping / Joining the items from two iterators
    let first_names = vec!["John", "Jane", "Jack", "Jill"];
    let last_names = vec!["Doe", "Smith", "Johnson", "Brown"];
    let full_names = first_names.iter().zip(last_names.iter());
    println!("\n\nZipping two iterators\n");
    full_names.for_each(|e| println!("{} {}", e.0, e.1));

    // Skipping the first n elements of the iterator
    println!("\n\nSkipping the first n elements of the iterator\n");
    fruits_list
        .iter()
        .clone()
        .skip(3)
        .for_each(|food| println!("Not Skipped : {}", food));

    // Folding
    let foods_with_prices: Vec<(&str, u32)> = vec![
        ("Strawberry", 10),
        ("Banana", 5),
        ("Mango", 15),
        ("Apple", 8),
        ("Orange", 6),
        ("Grapes", 12),
        ("Pineapple", 20),
    ];
    println!("\n\nNew list with prices : {:?}\n", foods_with_prices);
    println!("Folding the iterator\n");

    let sum_of_food_with_prices = foods_with_prices
        .iter()
        .fold(0u32, |acc, food| acc + food.1);
    println!("Total price of all foods : {}", sum_of_food_with_prices);
}
