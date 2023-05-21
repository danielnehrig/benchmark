use benchmark::sync::Benchmark;

fn fib(n: i32) -> i128 {
    let mut vec = vec![1, 1];
    for x in 2..n {
        vec.push(vec[x as usize - 1_usize] + vec[x as usize - 2_usize]);
    }
    vec[n as usize - 1_usize]
}

fn main() {
    let result = fib(10);
    assert_eq!(result, 55);
    Benchmark::default_run(|| {
        let _ = fib(10);
    })
    .save("fib");
}
