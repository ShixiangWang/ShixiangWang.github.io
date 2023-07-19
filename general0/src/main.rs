fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: other.y }
    }
}

fn main() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add f64: {}", add(1.23, 1.23));
    
    let p1 = Point {x: 5, y: 10.5};
    let p2 = Point {x: "Hello", y: 'c'};
    
    let p3 = p1.mixup(p2);
    
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    
    // Const general
    fn display_arry<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }
    
    let arr: [i32; 3] = [1, 2, 3];
    display_arry(arr);
    
    let arr: [i32; 2] = [1, 2];
    display_arry(arr);
    
    let integer = Some(5);
    let float = Some(5.0);
    
}
