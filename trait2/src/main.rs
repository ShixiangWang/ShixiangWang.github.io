trait Draw {
    // Self points to Draw
    // self points to this object
    fn draw(&self) -> Self;
}

#[derive(Clone, Debug)]
struct Button;
impl Draw for Button {
    fn draw(&self) -> Self {
        return self.clone()
    }
}

fn main() {
    let button = Button;
    let newb = button.draw();
    println!("{:?}", newb);
}


