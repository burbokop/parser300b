

use std::mem::MaybeUninit;

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use parser300b::{internal::{generate_combinations, expand_combinations_iter, Ctx}, Grammar};
use trim_margin::MarginTrimmable;


fn combinations_benchmark(c: &mut Criterion) {
    // 9.2668 µs
    // 5.7665 µs marks = with_capacity
    // 11.000 µs marks = array

    c.bench_function("generate", |b| 
        b.iter(|| 
            generate_combinations(
                black_box(2), 
                black_box(16), 
                black_box(3)
            )
        )
    );


    //45.169 µs
    c.bench_function("expand:light", |b| 
        b.iter(|| {
                expand_combinations_iter(vec![
                    black_box(vec![ 1, 2, 3 ].into_iter()),
                    black_box(vec![ 4, 5, 6 ].into_iter()),
                    black_box(vec![ 7 ].into_iter()),
                    black_box(vec![ 8, 9, 10, 11 ].into_iter()),
                    black_box(vec![ 12, 13, 14, 15 ].into_iter()),
                    black_box(vec![ 16, 17, 18, 19 ].into_iter()),
                ].into_iter())
                    .map(|v|v.collect::<Vec<_>>())
                    .collect::<Vec<_>>()
        })
    );

    #[derive(Clone)]
    struct S {
        pub _data: Vec<u8>
    }

    impl S {
        fn new() -> Self {
            Self { _data: vec![0; 1024] }
        }
    }

    //74.823 µs
    c.bench_function("expand:heavy", |b| 

        b.iter(|| {
            expand_combinations_iter(vec![
                black_box(vec![ S::new(), S::new(), S::new() ].into_iter()),
                black_box(vec![ S::new(), S::new(), S::new() ].into_iter()),
                black_box(vec![ S::new() ].into_iter()),
                black_box(vec![ S::new(), S::new(), S::new(), S::new() ].into_iter()),
                black_box(vec![ S::new(), S::new(), S::new(), S::new() ].into_iter()),
                black_box(vec![ S::new(), S::new(), S::new(), S::new() ].into_iter()),
            ].into_iter())
                .map(|v|v.collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
    );

    //
    {
        let tokens: Vec<String> = Vec::new();
        let grammar = Grammar { productions: vec![] };
        let ctx = Ctx::test(0, 7, &tokens, &grammar);

        let mut group = c.benchmark_group("split");
        for combination in ctx.combinations(6) {
            group.bench_with_input(BenchmarkId::from_parameter(combination.clone()), &combination, |b, combination| {
                b.iter(|| ctx.split(black_box(combination.clone())));
            });
        }
        group.finish();
    }
}

criterion_group!(benches, combinations_benchmark);
criterion_main!(benches);