#![allow(clippy::unwrap_used)]
use chart_file_parser::chart::chart;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn s_hero_individual(c: &mut Criterion) {
    for subdir in std::fs::read_dir("../charts").unwrap() {
        let entry = subdir.unwrap();
        let mut path = entry.path().clone();
        path.push("notes.chart");
        let input = std::fs::read_to_string(path).unwrap();
        let name = entry.file_name().to_str().unwrap().to_string();
        c.bench_function(&format!("Parse: {name}"), |b| {
            b.iter(|| chart(black_box(&input)));
        });
    }
}

fn s_hero_total(c: &mut Criterion) {
    let mut data = vec![];
    for subdir in std::fs::read_dir("../charts").unwrap() {
        let entry = subdir.unwrap();
        let mut path = entry.path().clone();
        path.push("notes.chart");
        let input = std::fs::read_to_string(path).unwrap();
        data.push(input);
    }
    c.bench_function(&format!("Parse S hero"), |b| {
        b.iter(|| {
            for input in &data {
                chart(black_box(&input)).unwrap();
            }
        });
    });
}

criterion_group!(benches, s_hero_individual, s_hero_total);
criterion_main!(benches);
