

use performance::{sort_algo_1,sort_algo_2};


use criterion::{criterion_group, criterion_main, Criterion};


fn sort_benchmark(c: &mut Criterion){
    let mut numbers = vec![
        1,2,3,5,6,8,3,8,90,67,34,22,44,65,11,90,88,65
    ];

    c.bench_function("sorting Algorithm", |b|{
        b.iter(|| sort_algo_2(&mut numbers))
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);