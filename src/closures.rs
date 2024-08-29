pub fn test_closures(){
    let test = || {
        println!("Testing Closures");
    };
    test();

    let add = |x:i8,y:i8| -> i16 {
        println!("Adding {} and {}",x,y);
        return x as i16 + y as i16;
    };
    let mut res = add(127,127);
    println!("Add() closure result {:?}",res);

    let mut mutate_res = || {
        res +=1;
        println!("Mutated Result inside closure = previous result + 1 : {}",res);
    };
    mutate_res();
    println!("Mutated Result outside of closure : {}",res);



}