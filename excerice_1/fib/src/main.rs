fn main() {
    let result = fib(50);
    println!("The result is: {}", result);
}

fn fib(mut n: usize) -> usize {
    n -= 1;
    let mut first = 0;
    let mut second = 1;
    let mut current: usize;
     while n > 0 {
        current = first + second;
        first = second;
        second = current;
        n -= 1;
    }
    second
}

// fn fib(n: u32) -> u32{
//     if n == 1 || n == 0 {
//         return n;
//     } else {
//         return fib(n - 1) + fib(n - 2);
//     }
// }
