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
        template_file: &Path,
        output_file: &Path,
        config_latency_chart: &ChartConfig<u32>,
        config_jitter_chart: &ChartConfig<u32>,
        config_download_chart: &ChartConfig<f64>,
        config_upload_chart: &ChartConfig<f64>,
    ) {
        //create chart data;
        let lat_chart = create_latency_chart(data, config_latency_chart);
        let jit_chart = create_jitter_chart(data, config_jitter_chart);
        let dwn_chart = create_download_chart(data, config_download_chart);
        let upl_chart = create_upload_chart(data, config_upload_chart);

        let stat_lat = create_statistic_table(ID_LATENCY, config_latency_chart, &lat_chart);
        let stat_jit = create_statistic_table(ID_JITTER, config_jitter_chart, &jit_chart);
        let stat_dwn = create_statistic_table(ID_DOWNLOAD, config_download_chart, &dwn_chart);
        let stat_upl = create_statistic_table(ID_UPLOAD, config_upload_chart, &upl_chart);

        let statistics_table = create_statistics_table(stat_lat, stat_jit, stat_dwn, stat_upl);

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

        write_output_file(
            template_file,
            output_file,
            data,
            &statistics_table,
            &response_time_json,
            &throughput_json,
        );
    }
}

fn write_output_file(
    template_file: &Path,
    output_file: &Path,
    data: &Vec<ParsedEntry>,
    statistics_table: &str,
    response_time_json: &str,
    throughput_json: &str,
) {
    // write files:
    let mut out_file = match fs::OpenOptions::new()
        .append(false)
        .create(true)
        .write(false) // no overwrite
        .open(output_file)
    {
        Err(e) => {
            let msg = format!("Could not create file message= '{}'", e);
            error!("{}", msg);
            panic!("{}", msg);
        }
        Ok(f) => f,
    };

    let find_replacement = Regex::new(REPLACEMENT_REGEX).unwrap();
    match fs::File::open(template_file) {
        Err(e) => {
            let msg = format!("Could not open template file message= '{}'", e);
            println!("{}", msg);
            error!("{}", msg);
        }
        Ok(f) => {
            let template_reader = BufReader::new(f);
            for maybe_line in template_reader.lines() {
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
                                        writeln!(
                                            &mut out_file,
                                           "<table id=\"rawdata\">\n
                                            <tr>\n
                                                <th class=\"ts\">timestamp</th>\n
                                                <th class=\"client\">client-wlan</th>\n
                                                <th class=\"client\">client-ip</th>\n
                                                <th class=\"client\">client-lat</th>\n
                                                <th class=\"client\">client-lon</th>\n
                                                <th class=\"client\">client-isp</th>\n
                                                <th class=\"server\">server-name</th>\n
                                                <th class=\"server\">server-sponsor</th>\n
                                                <th class=\"server\">server-distance</th>\n
                                                <th class=\"server\">server-host</th>\n
                                                <th class=\"performance\">latency</th>\n
                                                <th class=\"performance\">jitter</th>\n
                                                <th class=\"performance\">download_config</th>\n
                                                <th class=\"performance\">upload_config</th>\n
                                                <th class=\"performance\">download</th>\n
                                                <th class=\"performance\">upload</th>\n
                                             </tr>\n"
                                        );
                                        for entry in data {
                                            writeln!(
                                                &mut out_file,
                                                "<tr>
                                                  <td class=\"ts\">{:?}</td>",
                                                entry.timestamp.format(DATE_TIME_FORMAT)
                                            );
                                            if let Some(client) = &entry.client {
                                               writeln!(
                                                    &mut out_file,
                                                    "<td class=\"client\">{:?}</td>
                                                     <td class=\"client\">{:?}</td>
                                                     <td class=\"client\">{:?}</td>
                                                     <td class=\"client\">{:?}</td>
                                                     <td class=\"client\">{:?}</td>",
                                                     client.wlan,
                                                     client.ip,
                                                     client.lat,
                                                     client.lon,
                                                     client.isp
                                               );
                                            } else {
                                               writeln!(
                                                    &mut out_file,
                                                    "<td colspan=\"5\" class=\"client\"></td>"
                                               );
                                            }
                                            if let Some(server) = &entry.server {
                                                writeln!(
                                                    &mut out_file,
                                                    "<td class=\"server\">{:?}</td>
                                                     <td class=\"server\">{:?}</td>
                                                     <td class=\"server\">{:?}</td>
                                                     <td class=\"server\">{:?}</td>",
                                                     server.name,
                                                     server.sponsor,
                                                     server.distance,
                                                     server.host
                                                );
                                            } else {
                                                writeln!(
                                                    &mut out_file,
                                                    "<td colspan=\"4\" class=\"server\"></td>"
                                                );
                                            }
                                            if let Some(performance) = &entry.performance {
                                                writeln!(
                                                    &mut out_file,
                                                    "<td class=\"performance\">{:?}</td>
                                                     <td class=\"performance\">{:?}</td>
                                                     <td class=\"performance\">{:?}</td>
                                                     <td class=\"performance\">{:?}</td>
                                                     <td class=\"performance\">{:?}</td>
                                                     <td class=\"performance\">{:?}</td>",
                                                     performance.latency,
                                                     performance.jitter,
                                                     performance.download_config,
                                                     performance.upload_config,
                                                     performance.download,
                                                     performance.upload
                                                );
                                            } else {
                                                writeln!(
                                                    &mut out_file,
                                                    "<td colspan=\"6\" class=\"performance\"></td>"
                                                );
                                            }
                                            writeln!(
                                                &mut out_file,
                                                "</tr>"
                                            );
                                        }
                                        writeln!(
                                            &mut out_file,
                                            "</table>"
                                        );
                                    }
                                    REPLACEMENT_ID_STATISTICS => {
                                        writeln!(
                                            &mut out_file,
                                            "{}{}{}",
                                            prefix, statistics_table, suffix
                                        );                                    }
                                    REPLACEMENT_ID_RESPONSE_TIMES => {
                                        writeln!(
                                            &mut out_file,
                                            "{}{}{}",
                                            prefix, response_time_json, suffix
                                        );
                                    }
                                    REPLACEMENT_ID_THROUGHPUT => {
                                        writeln!(
                                            &mut out_file,
                                            "{}{}{}",
                                            prefix, throughput_json, suffix
                                        );
                                    }
                                    _ => {
                                        //ignore:
                                        writeln!(&mut out_file, "{}", &line);
                                    }
                                };
                            }
                            None => {
                                writeln!(&mut out_file, "{}", &line);
                            }
                        };
                    }
                }
            }
        }
    }
    match out_file.flush() {
        Err(e) => {
            let msg = format!("Could not write file message= '{}'", e);
            error!("{}", msg);
            panic!("{}", msg);
        }
        Ok(()) => (),
    };
}

fn create_statistics_table(
    stat_lat: String,
    stat_jit: String,
    stat_dwn: String,
    stat_upl: String,
) -> String {
    format!(
        "<table id=\"statistic\">\n
              <tr>\n
                <th>{}</th>\n
                <th>{}</th>\n
              </tr>
              <tr>\n
                <td>{}</td>\n
                <td>{}</td>\n
              </tr>\n
            </table>\n
        ",
        stat_lat, stat_jit, stat_dwn, stat_upl
    )
}

fn create_statistic_table<N: Copy>(
    id: &str,
    chart_config: &ChartConfig<N>,
    chart: &Chart<N>,
) -> String {
    format!(
        "<table id=\"statistic_{}\">\n
              <tr>\n
                <th colspan=\"3\">{}</th>\n
              </tr>
              <tr>\n
                <th>{}</th>\n
                <th>{}</th>\n
                <th>{}</th>\n
              </tr>
              <tr>\n
                <td>{}</td>\n
                <td>{}</td>\n
                <td>{}</td>\n
              </tr>\n
            </table>\n
        ",
        id,
        chart_config.label,
        STATISTIC_MEDIAN,
        STATISTIC_AVG,
        STATISTIC_STD,
        chart.median,
        chart.average,
        chart.standard_deviation
    )
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
