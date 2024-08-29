pub mod helpers;
pub mod closures;
pub mod match_case;


fn main() {
    println!("Hello, world!");
    /* //////////////////////////////////////////////////////////////
                                Basics 
    ////////////////////////////////////////////////////////////// */
    
    // test_func();
    // print_arr();
    // print_slice();

    
    /* //////////////////////////////////////////////////////////////
                            Modules & Sub-Modules 
    ////////////////////////////////////////////////////////////// */
    
    // let res:String = helpers::namehelpers::get_full_name("John","Doe");
    // print!("Full Name : {}",res);

    /* //////////////////////////////////////////////////////////////
                                Conditionals 
    ////////////////////////////////////////////////////////////// */
    
    // let is_drive_valid = test_if();

    /* //////////////////////////////////////////////////////////////
                                Loops
    ////////////////////////////////////////////////////////////// */
    // test_while();
    // test_loop();
    // test_for();

    
    /* //////////////////////////////////////////////////////////////
                    Anonymus Functions (Closures)
    ////////////////////////////////////////////////////////////// */
    
    // closures::test_closures();

    /* //////////////////////////////////////////////////////////////
                    Anonymus Functions (Closures)
    ////////////////////////////////////////////////////////////// */

    match_case::test_match_int();

}  



#[allow(dead_code)]
fn test_while() {
    let mut counter:u8 = 0;
    while counter <= 5{
        println!("Counter : {}",counter);
        counter += 1;
    }

}


#[allow(dead_code)]
fn test_loop() {
    let mut counter:u8 = 0;
    loop {
        print!("Counter : {}\n",counter);
        
        counter += 1;
        if counter > 5{
            break;
        }
    }
}


#[allow(dead_code)]
fn test_for(){

    let ages:[u8; 5] = [17,46,76,12,19];
    for age in ages.iter(){
        println!("Age : {}",age);
    }
}
/**
 * @dev Check if person is a valid age for a driver
 */


#[allow(dead_code)]
 fn test_if() {
    let age_to_drive = 18u8;

    println!("Enter the age of the person : ");   
    let age_string: &mut String = &mut String::from("");
    std::io::stdin().read_line(age_string).unwrap();
    
    /* //////////////////////////////////////////////////////////////
                            Parsing string inputs to integers 
    ////////////////////////////////////////////////////////////// */

    let age:u8 = age_string.replace("\n", "").trim().parse::<u8>().unwrap();

    if age > age_to_drive {
        println!("You are old enough to drive");
    }else if age == age_to_drive  {
        println!("You are almost old enough to drive");
    }else{
        println!("You are not old enough to drive");
    }
}

#[allow(dead_code)]
fn test_func(){

    let mut x:i32 = 2147483646;
    x = x+1;
    println!("Value of X : {:?}",x);
}

#[allow(dead_code)]
fn print_arr() {

    let arr: [i32;4]  = [1,2,34,5];

    println!("{:?}",arr);

}

#[allow(dead_code)]
fn print_slice(){
    let arr: [i32;7]  = [1,2,34,5,45,234,123];

    let slice:&[i32] = &arr[4..=6];

    println!("{:?}", arr);
    println!("{:?}", slice);

}