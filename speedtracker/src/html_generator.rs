use std::path::Path;

use crate::constants::*;
use crate::json_parser::*;
use serde::Serialize;

pub struct HtmlGenerator {}

#[derive(Serialize, Debug)]
struct Point<N> {
    x: String,
    y: N,
}

#[derive(Serialize, Debug)]
struct Dataset<N> {
    label: String,
    data: Vec<Point<N>>,
    fill: bool,
    border_color: String,
}

struct Chart<N> {
    id: String,
    datasets: Vec<Dataset<N>>,
}

struct ChartContainer {
    latency: Chart<u64>,
    jitter: Chart<u64>,
    download: Chart<f64>,
    upload: Chart<f64>,
}

impl HtmlGenerator {
    pub fn write_html(data: &Vec<ParsedEntry>, template_path: &Path, output_file: &Path) {
        let ds: Vec<Point<u64>> = data
            .iter()
            .map(|d| {
                let x: String = d.timestamp.format(DATE_TIME_FORMAT).to_string();
                let y = *d.performance.as_ref().map(|p| &p.latency).unwrap_or(&100);
                Point { x, y }
            })
            .collect();

        let json = serde_json::to_string(&ds).unwrap();

        println!("Test {}", &json); //TODO: write error also to console
    }
}
