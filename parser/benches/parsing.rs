#![allow(clippy::unwrap_used)]
use chart_file_parser::chart::Chart;
use criterion::{criterion_group, criterion_main, Criterion};

fn s_hero(c: &mut Criterion) {
    let mut group = c.benchmark_group("s_hero_parsing");
    for subdir in std::fs::read_dir("../charts").unwrap() {
        let entry = subdir.unwrap();
        let mut path = entry.path().clone();
        path.push("notes.chart");
        let input = std::fs::read_to_string(path).unwrap();
        let name = entry.file_name().to_str().unwrap().to_string();
        group.throughput(criterion::Throughput::Bytes(input.len() as u64));
        group.bench_with_input(&format!("Parse: {name}"), &input, |b, input| {
            b.iter(|| Chart::parse(input));
        });
    }
    group.finish();
}

criterion_group!(benches, s_hero);
criterion_main!(benches);
