fn greet_world() {
    // Rust 原生支持 UTF-8 编码的字符串
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello!";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region)
    }
}

fn main() {
    greet_world();
}


