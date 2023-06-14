use benchmark::prelude::{Benchmark, BenchmarkBuilder};

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

    BenchmarkBuilder::new()
        .passes(100)
        .done()
        .run(|| {
            let _ = fib(30);
        })
        .save("fib2_100");
}
