use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    
    println!("{} + {}i", result.re, result.im);
    
    let x = '中';
    println!("字符'中'占有了{}字节的内存大小", std::mem::size_of_val(&x));
}
