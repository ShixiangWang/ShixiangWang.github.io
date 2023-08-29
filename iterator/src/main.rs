struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));

    for i in Counter::new() {
        println!("{}", i);
    }

    for i in Counter::new().skip(1) {
        println!("{}", i);
    }

    for i in Counter::new().zip(Counter::new().skip(1)) {
        println!("{:?}", i);
    }

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("{}", sum);

    let v = vec![1u64, 2, 3, 4, 5, 6];
    for (i, v) in v.iter().enumerate() {
        println!("{} th value is {}", i, v);
    }

    println!("{}", [1f64, 2.0, 3.0].iter().sum::<f64>());
}
