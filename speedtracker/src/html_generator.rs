// Copyright (c) 2022 Florian Mantz
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
use log::error;
use regex::Regex;
use serde::Serialize;
use std::f64;
use std::fs;
use std::io::{self, prelude::*, BufReader, Write};
use std::path::Path;

use crate::chart_config::*;
use crate::constants::*;
use crate::json_parser::*;

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
        //create chart data;
        let lat_chart = create_latency_chart(data, latency_chart);
        let jit_chart = create_jitter_chart(data, jitter_chart);
        let dwn_chart = create_download_chart(data, download_chart);
        let upl_chart = create_upload_chart(data, upload_chart);

        //transform chart data to json
        let response_time_dss: Vec<&Dataset<u32>> = lat_chart
            .datasets
            .iter()
            .chain(jit_chart.datasets.iter())
            .collect();
        let throughput_dss: Vec<&Dataset<f64>> = dwn_chart
            .datasets
            .iter()
            .chain(upl_chart.datasets.iter())
            .collect();

        let response_time_json = serde_json::to_string(&response_time_dss).unwrap();
        let throughput_json = serde_json::to_string(&throughput_dss).unwrap();

        // write files:
        let find_replacement = Regex::new(REPLACEMENT_REGEX).unwrap();
        match fs::File::open(template_path) {
            Err(e) => {
                let msg = format!("Could not open template file message= '{}'", e);
                println!("{}", msg);
                error!("{}", msg);
            }
            Ok(f) => {
                let reader = BufReader::new(f);
                for maybe_line in reader.lines() {
                    match maybe_line {
                        Err(e) => {
                            let msg = format!("Could not read template file message= '{}'", e);
                            println!("{}", msg);
                            error!("{}", msg);
                        }
                        Ok(line) => {
                            match find_replacement.find(&line) {
                                Some(mat) => {
                                    //matched string without '${' and '}'
                                    let matched_string = &line[mat.start() + 2..mat.end() - 1];
                                    let prefix = &line[0..mat.start()];
                                    let suffix = &line[mat.end()..line.len()];
                                    match matched_string {
                                        REPLACEMENT_ID_RAW_DATA => {
                                            println!("{}", replace(prefix, "1", suffix))
                                        }
                                        REPLACEMENT_ID_STATISTICS => {
                                            println!("{}", replace(prefix, "2", suffix))
                                        }
                                        REPLACEMENT_ID_RESPONSE_TIMES => {
                                            println!("{}", replace(prefix, "3", suffix))
                                        }
                                        REPLACEMENT_ID_THROUGHPUT => {
                                            println!("{}", replace(prefix, "4", suffix))
                                        }
                                        _ => {
                                            //ignore:
                                            println!("{}", &line)
                                        } //restore
                                    };
                                    println!("{}", suffix);
                                }
                                None => {
                                    println!("nix")
                                }
                            };
                        }
                    }
                }
            }
        }
    }
}

fn replace(prefix: &str, replacement: &str, suffix: &str) -> String {
    format!("{}{}{}", prefix, replacement, suffix)
}

/// prepare data to show latency:
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

/// prepare data to show jitter:
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

/// prepare data to download speed:
fn create_download_chart(data: &Vec<ParsedEntry>, config: &ChartConfig<f64>) -> Chart<f64> {
    let points: Vec<Point<f64>> = data
        .iter()
        .map(|d| {
            let x: String = d.timestamp.format(DATE_TIME_FORMAT).to_string();
            let y: f64 = d
                .performance
                .as_ref()
                .map(|p| p.download.unwrap_or(config.default_value))
                .unwrap();
            Point { x, y }
        })
        .collect();

    let dss = create_datasets(&config, points);

    let mut values: Vec<f64> = data
        .iter()
        .flat_map(|d| {
            let y = d.performance.as_ref().map(|p| p.download);
            y
        })
        .flatten()
        .collect();

    create_chart(&config, dss, &mut values)
}

/// prepare data to upload speed:
fn create_upload_chart(data: &Vec<ParsedEntry>, config: &ChartConfig<f64>) -> Chart<f64> {
    let points: Vec<Point<f64>> = data
        .iter()
        .map(|d| {
            let x: String = d.timestamp.format(DATE_TIME_FORMAT).to_string();
            let y: f64 = d
                .performance
                .as_ref()
                .map(|p| p.upload.unwrap_or(config.default_value))
                .unwrap();
            Point { x, y }
        })
        .collect();

    let dss = create_datasets(&config, points);

    let mut values: Vec<f64> = data
        .iter()
        .flat_map(|d| {
            let y = d.performance.as_ref().map(|p| p.upload);
            y
        })
        .flatten()
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
fn create_dataset_expected<T: Copy>(
    config: &ExpectedConfig<T>,
    points: &Vec<Point<T>>,
) -> Dataset<T> {
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
fn create_chart<T: Copy>(
    config: &ChartConfig<T>,
    dss: Vec<Dataset<T>>,
    mut values: &mut Vec<f64>,
) -> Chart<T> {
    let med: f64 = median(&mut values); //is also sorting!
    let avg: f64 = average(&values);
    let std: f64 = standard_deviation(&values, &avg);

    Chart {
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
