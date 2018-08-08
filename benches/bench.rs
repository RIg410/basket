#[macro_use]
extern crate criterion;
extern crate rand;
extern crate basket;

use basket::*;

use criterion::Criterion;
use criterion::ParameterizedBenchmark;
use std::sync::Arc;

type PutData = (Price, Size, Meta);

fn make_put_data(price_count: usize, size_count: usize) -> Vec<PutData> {
    use rand;
    let mut res: Vec<PutData> = (1..price_count)
        .map(|_| rand::random::<Price>())
        .map(|p| (p, (1..size_count).map(|_| rand::random::<Size>())))
        .flat_map(|(p, r)| r.map(|s| (p, s, rand::random::<u64>() as Meta)).collect::<Vec<_>>())
        .collect();

    res.sort_by(|(_, _, m_1), (_, _, m_2)| m_1.cmp(m_2));
    res
}

fn make_split_data(data: Vec<PutData>) -> (Basket, (Price, Size)) {
    let mut basket = Basket::new();

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

fn put_bench(c: &mut Criterion, test_name: &str, data: Vec<PutData>) {
    c.bench_function(test_name, move |b| {
        b.iter_with_large_setup(
            || (Basket::new(), data.clone()),
            |(mut basket, data)| {
                for call_item in data {
                    basket.put(call_item.0, call_item.1, call_item.2);
                }
            });
    });
}

fn split_bench(c: &mut Criterion, test_name: &str, data: Vec<PutData>) {
    c.bench_function(test_name, move |b| {
        b.iter_with_large_setup(
            || make_split_data(data.clone()),
            |(mut basket, split)| {
                basket.split(split.0, split.1);
            });
    });
}

fn split_1_bench(c: &mut Criterion, test_name: &str, data: Vec<PutData>) {
    c.bench_function(test_name, move |b| {
        b.iter_with_large_setup(
            || make_split_data(data.clone()),
            |(mut basket, split)| {
                basket.split_1(split.0, split.1);
            });
    });
}

fn bench(c: &mut Criterion) {
    let data = make_put_data(100, 10);
    put_bench(c, "put_test[price 100, size: 10]", data.clone());
    split_bench(c, "split_test[price 100, size: 10]", data.clone());
    split_1_bench(c, "split_1_test[price 100, size: 10]", data.clone());

    let data = make_put_data(100, 100);
    put_bench(c, "put_test[price 100, size: 100]", data.clone());
    split_bench(c, "split_test[price 100, size: 100]", data.clone());
    split_1_bench(c, "split_1_test[price 100, size: 100]", data.clone());

    let data = make_put_data(1000, 10);
    put_bench(c, "put_test[price 1000, size: 10]", data.clone());
    split_bench(c, "split_test[price 1000, size: 10]", data.clone());
    split_1_bench(c, "split_1_test[price 1000, size: 10]", data.clone());

    let data = make_put_data(1000, 100);
    put_bench(c, "put_test[price 1000, size: 100]", data.clone());
    split_bench(c, "split_test[price 1000, size: 100]", data.clone());
    split_1_bench(c, "split_1_test[price 1000, size: 100]", data.clone());
}

criterion_group!(benches, bench);
criterion_main!(benches);

