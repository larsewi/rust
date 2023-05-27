fn main() {
    const N: u32 = 7;
    let res = fib(N);
    println!("fib({N}) = {res}");
}

fn fib(n: u32) -> u32 {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for _ in 1..n {
        let tmp = b;
        b = a + b;
        a = tmp;
    }
    b
}
