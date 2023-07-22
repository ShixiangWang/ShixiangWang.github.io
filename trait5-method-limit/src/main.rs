trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    //println!("A baby dog is called a {}", Dog::baby_name()); // Ok
    //println!("A baby dog is called a {}", Animal::baby_name()); // Error if without limit
    
    // 完全限定语法
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
