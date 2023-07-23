fn main() {
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    
    scores.insert("Blue", 10);
    
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));
    
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));
    
    let v = scores.entry("Yello").or_insert(5);
    assert_eq!(*v, 5);
    
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", map);
}
