#![allow(clippy::unwrap_used)]
use chart_file_parser::chart::Chart;

#[test]
fn s_hero_parses() {
    for subdir in std::fs::read_dir("../charts").unwrap() {
        let entry = subdir.unwrap();
        let mut path = entry.path().clone();
        path.push("notes.chart");
        let input = std::fs::read_to_string(path).unwrap();
        let name = entry.file_name().to_str().unwrap().to_string();
        println!("{name}");
        Chart::parse(&input).map_err(|_| &name).expect(&name);
    }
}

#[test]
fn s_hero_write_reparse() {
    for subdir in std::fs::read_dir("../charts").unwrap() {
        let entry = subdir.unwrap();
        let mut path = entry.path().clone();
        path.push("notes.chart");
        let input = std::fs::read_to_string(path).unwrap();
        let name = entry.file_name().to_str().unwrap().to_string();
        println!("{name}");
        let (_, chart) = Chart::parse(&input).map_err(|_| &name).expect(&name);
        let written = chart.to_string();
        let (_, chart2) = Chart::parse(&written).map_err(|_| &name).expect(&name);
        assert_eq!(chart, chart2);
    }
}
