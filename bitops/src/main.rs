fn main() {
    // 二进制为00000010
    let a: i32 = 2;
    // 二进制为00000011
    let b: i32 = 3;
    
    println!("(a & b) value is {}", a & b);
    println!("(a | b) value is {}", a | b);
    println!("(a ^ b) value is {}", a ^ b);
    println!("(!b) value is {}", !b);
    println!("(a << b) value is {}", a << b);
    println!("(a >> b) value is {}", a >> b);
    
    let mut a = a;
    a <<= b;
    println!("(a << b) value is {}", a);
    
    // Range
    for i in 1..=5 {
        println!("{}", i);
    }
    
    for i in 'a'..='z' {
        println!("{}", i);
    }
}
