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
    median: f64,
    average: f64,
    standard_deviation: f64,
}

struct ChartContainer {
    latency: Chart<u32>,
    jitter: Chart<u32>,
    download: Chart<f64>,
    upload: Chart<f64>,
}

impl HtmlGenerator {
    pub fn write_html(data: &Vec<ParsedEntry>, template_path: &Path, output_file: &Path) {
        /*
          let ds: Vec<Point<u32>> = data
              .iter()
              .map(|d| {
                  let x: String = d.timestamp.format(DATE_TIME_FORMAT).to_string();
                  let y = *d.performance.as_ref().map(|p| &p.latency).unwrap_or(&100);
                  Point { x, y }
              })
              .collect();
        */
        let ds = create_latency_chart(&data, &100).datasets;

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

fn create_latency_chart(
    data: &Vec<ParsedEntry>,
    error_latency: &u32, //todo add everythink into one config object
) -> Chart<u32> {
    let points: Vec<Point<u32>> = data
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

    let mut values: Vec<f64> = data
        .iter()
        .flat_map(|d| {
            let y = d.performance.as_ref().map(|p| &p.latency);
            y
        })
        .map(|x| *x as f64)
        .collect();

    let med: f64 = median(&mut values); //is also sorting!
    let avg: f64 = average(&values);
    let std: f64 = standard_deviation(&values, &avg);

    Chart {
        id: "".to_string(),
        datasets: vec![ds],
        median: med,
        average: avg,
        standard_deviation: std,
    }
}

fn median(numbers: &mut Vec<f64>) -> f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid: usize = numbers.len() / 2;
    numbers[mid]
}

fn average(numbers: &Vec<f64>) -> f64 {
    let len: f64 = numbers.len() as f64;
    numbers.iter().sum::<f64>() / len
}

fn standard_deviation(numbers: &Vec<f64>, average: &f64) -> f64 {
    let variance: f64 = numbers
        .iter()
        .map(|x| {
            let y = x - average;
            y * y
        })
        .sum::<f64>();
    f64::sqrt(variance)
}
