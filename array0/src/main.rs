fn main() {
    println!("Hello, world!");
    
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    println!("#{:#?}", array);
    
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    //dbg!(assert_eq!(slice, &[2, 1]));
    
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];
    
    
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];
    println!("{:?}", arrays);
    
    for a in &arrays {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n+10);
        }
        
        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}
