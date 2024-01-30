#![allow(clippy::unwrap_used)]
use chart_file_parser::chart::Chart;
use criterion::{criterion_group, criterion_main, Criterion};

fn s_hero(c: &mut Criterion) {
    let mut group = c.benchmark_group("s_hero_writing");
    for subdir in std::fs::read_dir("../charts").unwrap() {
        let entry = subdir.unwrap();
        let mut path = entry.path().clone();
        path.push("notes.chart");
        let input = std::fs::read_to_string(path).unwrap();
        let name = entry.file_name().to_str().unwrap().to_string();
        let chart = Chart::parse(&input).unwrap().1;
        group.throughput(criterion::Throughput::Bytes(input.len() as u64));
        group.bench_with_input(&format!("Write: {name}"), &chart, |b, chart| {
            b.iter(|| chart.to_string());
        });
    }
}

criterion_group!(benches, s_hero);
criterion_main!(benches);
