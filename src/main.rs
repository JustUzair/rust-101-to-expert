pub mod helpers;
fn main() {
    println!("Hello, world!");
    // test_func();
    // print_arr();
    // print_slice();
    let res:String = helpers::get_full_name("John","Doe");
    print!("Full Name : {}",res);
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