// sub-module namehelpers
pub mod namehelpers{
    pub fn get_full_name(first: &str, last : &str) -> String {
        let full_name: String = format!("{} {}",first,last);
        return full_name;
    }   
}

pub mod mathhelpers{
    pub fn add(x:i32,y:i32) -> i32 {
        return x+y;
    }
    pub fn sub(x:i32,y:i32) -> i32 {
        return x-y;
    }
    pub fn mul(x:i32,y:i32) -> i32 {
        return x*y;
    }
    pub fn div(x:i32,y:i32) -> i32 {
        return x/y;
    }
}
    
