fn main() {
    let a = i8::MAX;
    println!("{}", a);
    
    let b = 100_i8 as i32;
    let c = 'a' as u8;
    println!("{},{},{}", a, b, c);
    
    
    // TryInto
    use std::convert::TryInto;
    
    let a: u8 = 10;
    let b: u16 = 1500;
    
    let b_: u8 = b.try_into().unwrap();
    
    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}
