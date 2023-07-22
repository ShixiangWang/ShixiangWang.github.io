trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// This method goes first
impl Human {
    fn fly(&self) {
        println!("*waving arms furiouly*")
    }
}
    
fn main() {
    let h = Human;
    h.fly(); // call method in struct
    Pilot::fly(&h); // call method in trait Pilot
    Wizard::fly(&h);
}
