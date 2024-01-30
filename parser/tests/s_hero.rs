#![allow(clippy::unwrap_used)]
use chart_file_parser::chart::Chart;

#[test]
fn s_hero_individual() {
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
