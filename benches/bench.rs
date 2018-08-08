#[macro_use]
extern crate criterion;
extern crate rand;
extern crate basket;

use basket::*;

use criterion::Criterion;
use criterion::ParameterizedBenchmark;
use std::sync::Arc;

fn make_put_data(price_count: usize, size_count: usize) -> Vec<(Price, Size, Meta)> {
    use rand;
    let mut res: Vec<(Price, Size, Meta)> = (1..price_count)
        .map(|_| rand::random::<Price>())
        .map(|p| (p, (1..size_count).map(|_| rand::random::<Size>())))
        .flat_map(|(p, r)| r.map(|s| (p, s, rand::random::<u64>() as Meta)).collect::<Vec<_>>())
        .collect();

    res.sort_by(|(_, _, m_1), (_, _, m_2)| m_1.cmp(m_2));
    res
}

fn make_split_data(price_count: usize, size_count: usize) -> (Basket, (Price, Size)) {
    let mut basket = Basket::new();
    let data = make_put_data(price_count, size_count);

    data.iter()
        .for_each(|(price, size, meta)| {
            basket.put(*price, *size, *meta)
        });

    let (price, _, _) = data[data.len() / 2];

    let size = data.iter()
        .filter(|(p, _, _)| *p < price)
        .map(|(_, size, _)| size)
        .sum();

    (basket, (price, size))
}

fn put_test(mut basket: Basket, data: Vec<(Price, Size, Meta)>) {
    for call_item in data {
        basket.put(call_item.0, call_item.1, call_item.2);
    }
}

fn put_bench(c: &mut Criterion, price_count: usize, size_count: usize) {
    c.bench_function(&format!("put_test[price {0}, size: {1}]", price_count, size_count), move |b| {
        b.iter_with_large_setup(
            || (Basket::new(), make_put_data(price_count, size_count)),
            |(mut basket, data)| {
                for call_item in data {
                    basket.put(call_item.0, call_item.1, call_item.2);
                }
            });
    });
}

fn split_bench(c: &mut Criterion, price_count: usize, size_count: usize) {
    c.bench_function(&format!("split_test[price {0}, size: {1}]", price_count, size_count), move |b| {
        b.iter_with_large_setup(
            || make_split_data(price_count, size_count),
            |(mut basket, split)| {
                basket.split(split.0, split.1);
            });
    });
}

fn bench(c: &mut Criterion) {
    put_bench(c, 100, 10);
    split_bench(c, 100, 10);

    put_bench(c, 100, 100);
    split_bench(c, 100, 100);

    put_bench(c, 1000, 10);
    split_bench(c, 1000, 10);

    put_bench(c, 1000, 100);
    split_bench(c, 1000, 100);
}

criterion_group!(benches, bench);
criterion_main!(benches);

