pub trait Summary {
    //fn summarize(&self) -> String;
    // or detail
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

// Reload the default method
// impl Summary for Post {}
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("title {}, author is {}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} sent a post {}", self.username, self.content)
    }
}


fn main() {
    let post = Post {
        title: "Rust intro".to_string(),
        author: "Shixiang".to_string(),
        content: "Rust is awesome".to_string()
    };
    
    let weibo = Weibo {
        username: "Shixiang".to_string(),
        content: "Weibo is not good as tweet".to_string()
    };
    
    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
    
}
