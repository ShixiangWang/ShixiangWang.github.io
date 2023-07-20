pub trait Summary {
    //fn summarize(&self) -> String;
    // or detail
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}


// Use trait as parameters
// Any one impl the summary could be treated as parameter
//pub fn notify(item: &impl Summary) {
//    println!("Breaking news! {}", item.summarize());
//}

// Constrain
// pub fn notify(item: &(impl Summary + Display)) {};

// More complex
//fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
//{}


use std::fmt::Display;

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    println!("Hello, world!");
    
    let p = Pair::new(3, 5);
    println!("{:?}", p.cmp_display());
}
