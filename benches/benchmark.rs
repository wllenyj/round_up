use criterion::{criterion_group, criterion_main, Criterion};

use round_up::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench_div_round_up", |b| {
        b.iter(|| {
            let _ = div_round_up(4123412, 8);
            let _ = div_round_up(4123412, 16);
            let _ = div_round_up(4123412, 32);
            let _ = div_round_up(4123412, 64);
            let _ = div_round_up(4123412, 2048);
            let _ = div_round_up(4123412, 4096);
        })
    });
    c.bench_function("bench_round_up", |b| {
        b.iter(|| {
            let _ = round_up(4123412, 8);
            let _ = round_up(4123412, 16);
            let _ = round_up(4123412, 32);
            let _ = round_up(4123412, 64);
            let _ = round_up(4123412, 2048);
            let _ = round_up(4123412, 4096);
        })
    });
    c.bench_function("bench_round_up_div", |b| {
        b.iter(|| {
            let _ = round_up_div(4123412, 8);
            let _ = round_up_div(4123412, 16);
            let _ = round_up_div(4123412, 32);
            let _ = round_up_div(4123412, 64);
            let _ = round_up_div(4123412, 2048);
            let _ = round_up_div(4123412, 4096);
        })
    });
    c.bench_function("bench_100000000_4_div_round_up", |b| {
        b.iter(|| {
            for i in 0..100000000 {
                let _ = div_round_up(i, 4);
            }
        })
    });
    c.bench_function("bench_100000000_4_round_up", |b| {
        b.iter(|| {
            for i in 0..100000000 {
                let _ = round_up(i, 4);
            }
        })
    });
    c.bench_function("bench_100000000_4_round_up_div", |b| {
        b.iter(|| {
            for i in 0..100000000 {
                let _ = round_up_div(i, 4);
            }
        })
    });
    c.bench_function("bench_100000000_0x1000000_div_round_up", |b| {
        b.iter(|| {
            for i in 0..100000000 {
                let _ = div_round_up(i, 0x1000000);
            }
        })
    });
    c.bench_function("bench_100000000_0x1000000_round_up", |b| {
        b.iter(|| {
            for i in 0..100000000 {
                let _ = round_up(i, 0x1000000);
            }
        })
    });
    c.bench_function("bench_100000000_0x1000000_round_up_div", |b| {
        b.iter(|| {
            for i in 0..100000000 {
                let _ = round_up_div(i, 0x1000000);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
