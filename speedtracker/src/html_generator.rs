use std::path::Path;

use crate::chart_config::*;
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
#[serde(rename_all = "camelCase")]
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
    pub fn write_html(
        data: &Vec<ParsedEntry>,
        template_path: &Path,
        output_file: &Path,
        latency_chart: &ChartConfig<u32>,
        jitter_chart: &ChartConfig<u32>,
        download_chart: &ChartConfig<f64>,
        upload_chart: &ChartConfig<f64>,
    ) {
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
        let ds = create_latency_chart(data, latency_chart).datasets;

        let json = serde_json::to_string(&ds).unwrap();

        println!("Test {}", &json); //TODO: write error also to console
    }

    //latency to datasets

    //jitter to datasets

    //download to datasets

    //upload to datasets:

    //map id -> Vec<datasets>

    //table1

    //performance ...

    //table2

    //average median and standard_deviation in eigener tabelle
}

fn create_latency_chart(data: &Vec<ParsedEntry>, config: &ChartConfig<u32>) -> Chart<u32> {
    let points: Vec<Point<u32>> = data
        .iter()
        .map(|d| {
            let x: String = d.timestamp.format(DATE_TIME_FORMAT).to_string();
            let y = *d
                .performance
                .as_ref()
                .map(|p| &p.latency)
                .unwrap_or(&config.default_value);
            Point { x, y }
        })
        .collect();

    let dss = create_datasets(&config, points);

    let mut values: Vec<f64> = data
        .iter()
        .flat_map(|d| {
            let y = d.performance.as_ref().map(|p| p.latency);
            y
        })
        .map(|x| x as f64)
        .collect();

    create_chart(&config, dss, &mut values)
}

fn create_jitter_chart(data: &Vec<ParsedEntry>, config: &ChartConfig<u32>) -> Chart<u32> {
    let points: Vec<Point<u32>> = data
        .iter()
        .map(|d| {
            let x: String = d.timestamp.format(DATE_TIME_FORMAT).to_string();
            let y: u32 = d
                .performance
                .as_ref()
                .map(|p| p.jitter.unwrap_or(config.default_value))
                .unwrap();
            Point { x, y }
        })
        .collect();

    let dss = create_datasets(&config, points);

    let mut values: Vec<f64> = data
        .iter()
        .flat_map(|d| {
            let y = d.performance.as_ref().map(|p| p.jitter);
            y
        })
        .flatten()
        .map(|x| x as f64)
        .collect();

    create_chart(&config, dss, &mut values)
}

// helper methods:

fn create_datasets<T: Copy>(config: &ChartConfig<T>, points: Vec<Point<T>>) -> Vec<Dataset<T>> {
    let expected_ds = config
        .expected_value
        .as_ref()
        .map(|c| create_dataset_expected(c, &points));

    let ds = create_dataset(&config, points);
    vec![Some(ds), expected_ds].into_iter().flatten().collect()
}

/// dataset containing the real data:
fn create_dataset<T: Copy>(config: &ChartConfig<T>, points: Vec<Point<T>>) -> Dataset<T> {
    Dataset {
        label: String::from(&config.label),
        data: points,
        fill: config.fill,
        border_color: String::from(&config.border_color),
    }
}

/// dataset for expected line:
fn create_dataset_expected<T: Copy>(config: &ExpectedConfig<T>, points: &Vec<Point<T>>) -> Dataset<T> {
    let default_x = &String::from("");
    let first_x: &str = points.first().map(|p| &p.x).unwrap_or(default_x);
    let last_x: &str = points.last().map(|p| &p.x).unwrap_or(default_x);
    Dataset {
        label: String::from(&config.label),
        data: vec![
            Point {
                x: String::from(first_x),
                y: config.value,
            },
            Point {
                x: String::from(last_x),
                y: config.value,
            },
        ],
        fill: config.fill,
        border_color: String::from(&config.border_color),
    }
}

/// create chart an do some statistics:
fn create_chart<T:Copy>(
    config: &ChartConfig<T>,
    dss: Vec<Dataset<T>>,
    mut values: &mut Vec<f64>,
) -> Chart<T> {
    let med: f64 = median(&mut values); //is also sorting!
    let avg: f64 = average(&values);
    let std: f64 = standard_deviation(&values, &avg);

    Chart {
        id: String::from(&config.id),
        datasets: dss,
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
