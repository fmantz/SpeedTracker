use std::path::Path;

use crate::constants::*;
use crate::json_parser::*;
use serde::Serialize;
use std::f64;

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
    average: f64,
    median: f64,
    standard_deviation: f64,
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

    //latency to datasets

    //jitter to datasets

    //download to datasets

    //upload to datasets:

    //map id -> Vec<datasets>

    //table

    //average median and standard_deviation in eigener tabelle
}

pub fn create_latency_chart(
    data: &Vec<ParsedEntry>,
    error_latency: &u64, //todo add everythink into one config object
) -> Chart<u64> {
    let points: Vec<Point<u64>> = data
        .iter()
        .map(|d| {
            let x: String = d.timestamp.format(DATE_TIME_FORMAT).to_string();
            let y = *d
                .performance
                .as_ref()
                .map(|p| &p.latency)
                .unwrap_or(error_latency);
            Point { x, y }
        })
        .collect();

    let ds = Dataset {
        label: "".to_string(), //TODO
        data: points,
        fill: false,                  //TODO
        border_color: "".to_string(), //TODO
    };

    Chart {
        id: "".to_string(), //TODO
        datasets: vec![ds],
        average: 0.0, //TODO
        median: 0.0,
        standard_deviation: 0.0, //TODO
    }
}

fn standard_deviation(numbers: &Vec<f64>, average: f64) -> f64 {
    let variance: f64 = numbers
        .iter()
        .map(|x| {
            let y = x - average;
            y * y
        })
        .sum::<f64>();
    f64::sqrt(variance)
}

fn average(numbers: &Vec<f64>) -> f64 {
    let len: f64 = numbers.len() as f64;
    numbers.iter().sum::<f64>() / len
}

fn median(numbers: &mut Vec<f64>) -> f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid: usize = numbers.len() / 2;
    numbers[mid]
}
