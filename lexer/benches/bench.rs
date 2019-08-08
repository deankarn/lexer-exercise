#[macro_use]
extern crate criterion;

use criterion::{Benchmark, Criterion, Throughput};

fn criterion_benchmark(c: &mut Criterion) {
    let input = r#"
    {
        "key1": "value1",
        "key2": "value2",
        "key3": "value3"
    }
    "#;
    c.bench(
        "lex",
        Benchmark::new("lex_small", move |b| b.iter(|| input == ""))
            .throughput(Throughput::Bytes(input.as_bytes().len() as u32)),
    );

    let input = r#"
    {
        "key1": "value1",
        "key2": "value2",
        "key3": "value3",
        "nested": {
            "nested_key1":"nested_value1",
            "nested_key2": "nested_value2",
            "nested_key3": "nested_value3"
        }
        "key4": "value4",
        "key5": "value5",
        "key6": "value6",
    }
    "#;
    c.bench(
        "lex",
        Benchmark::new("lex_medium", move |b| b.iter(|| input == ""))
            .throughput(Throughput::Bytes(input.as_bytes().len() as u32)),
    );

    let input = r#"
    {
        "key1": "value1",
        "key2": "value2",
        "key3": "value3",
        "nested": {
            "nested_key1":"nested_value1",
            "nested_key2": "nested_value2",
            "nested_key3": "nested_value3",
            "nested_key4": [
                {
                    "nested_2x_key1":"nested_2x_value1",
                    "nested_2x_key2": "nested_2x_value2",
                    "nested_2x_key3": "nested_2x_value3"
                },
                {
                    "nested_2x_key1":"nested_2x_value1",
                    "nested_2x_key2": "nested_2x_value2",
                    "nested_2x_key3": "nested_2x_value3"
                },
                {
                    "nested_2x_key1":"nested_2x_value1",
                    "nested_2x_key2": "nested_2x_value2",
                    "nested_2x_key3": "nested_2x_value3"
                }
            ]
        }
        "key4": "value4",
        "key5": "value5",
        "key6": "value6",
        "key7": "value7",
        "key8": "value8",
        "key9": "value9",
        "key10": "value10",
    }
    "#;
    c.bench(
        "lex",
        Benchmark::new("lex_large", move |b| b.iter(|| input == ""))
            .throughput(Throughput::Bytes(input.as_bytes().len() as u32)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
