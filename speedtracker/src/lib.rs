// MIT License
//
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

use chrono::NaiveDate;
use chrono::{Duration, Local, TimeZone};
use confy;
use confy::ConfyError;
use faccess::{AccessMode, PathExt};
use flexi_logger::*;
use log::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, prelude::*, BufReader, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

use crate::chart_config::*;
use crate::constants::*;
use crate::html_generator::HtmlGenerator;
use crate::json_parser::JsonParser;
use crate::json_parser::ParsedEntry;

mod chart_config;
mod constants;
mod html_generator;
mod json_parser;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// directory where data is collected
    data_dir: String,
    /// should be on a path served by a webserver (apache e.g.)
    output_file: String,
    /// number of days in the past (from today) the data should be visualized
    output_xdays: u32,
    /// log file path and name
    log_file: String,
    /// maximal size of log file in kb before the file is rotated
    log_file_max_length_in_kb: u64,
    /// latency_chart configuration
    latency_chart: ChartConfig<u32>,
    /// jitter_chart configuration
    jitter_chart: ChartConfig<u32>,
    /// download_chart configuration
    download_chart: ChartConfig<f64>,
    /// upload_chart configuration
    upload_chart: ChartConfig<f64>,
}

#[derive(Debug)]
pub struct Setup {
    /// directory where data is collected
    data_dir: String,
    /// new results should be appended to this file
    new_data_file: Option<String>,
    /// first file name with relevant data e.g. '2022-01.json'
    first_filter_file_name: String,
    /// last file name with relevant data e.g. '2022-02.json'
    last_filter_file_name: String,
    /// start date from which the data should be vizualized
    from_date: NaiveDate,
    /// end date from which the data should be vizualized
    to_date: NaiveDate,
    /// should be on a path served by a webserver (apache e.g.)
    output_file: String,
    /// different to the standard working dir, it is the same
    /// dir the program is stored. so the program can be called
    /// from anywhere and read the config an template file always
    /// from the same place
    working_dir: String,
    /// latency_chart configuration
    latency_chart: ChartConfig<u32>,
    /// jitter_chart configuration
    jitter_chart: ChartConfig<u32>,
    /// download_chart configuration
    download_chart: ChartConfig<f64>,
    /// upload_chart configuration
    upload_chart: ChartConfig<f64>,
}

impl Setup {
    /// run speed test in mode 1
    /// or do nothing  in mode 2
    pub fn maybe_speed_test(&self) {
        if let Some(f) = &self.new_data_file {
            run_speed_test(Path::new(&self.working_dir), Path::new(&f));
        }
    }
    /// parse data in specific time range
    pub fn read_data(&self) -> Vec<ParsedEntry> {
        let maybe_file_list = read_data_file_paths(
            Path::new(&self.data_dir),
            &self.first_filter_file_name,
            &self.last_filter_file_name,
        );
        if let Some(file_list) = maybe_file_list {
            file_list
                .iter()
                .flat_map(|file| {
                    parse_output_file(&Path::new(&file), &self.from_date, &self.to_date)
                })
                .flatten()
                .collect()
        } else {
            Vec::new()
        }
    }
    /// generate html file by transforming data and template:
    pub fn generate_html(&self, data: &Vec<ParsedEntry>) {
        HtmlGenerator::write_html(
            data,
            &Path::new(&self.working_dir).join(TEMPLATE_FILENAME),
            &Path::new(&self.output_file),
            &self.latency_chart,
            &self.jitter_chart,
            &self.download_chart,
            &self.upload_chart,
        );
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            data_dir: String::from(DEFAULT_DATA_DIR),
            output_file: String::from(DEFAULT_OUTPUT_FILE),
            output_xdays: DEFAULT_OUTPUT_XDAYS,
            log_file: String::from(DEFAULT_LOG_FILE),
            log_file_max_length_in_kb: DEFAULT_LOG_FILE_MAX_LENGTH_IN_KB,
            latency_chart: ChartConfig {
                label: String::from(DEFAULT_LATENCY_LABEL),
                fill: DEFAULT_FILL,
                border_color: String::from(DEFAULT_LATENCY_COLOR),
                default_value: DEFAULT_LATENCY_VALUE,
                expected_value: None,
            },
            jitter_chart: ChartConfig {
                label: String::from(DEFAULT_JITTER_LABEL),
                fill: DEFAULT_FILL,
                border_color: String::from(DEFAULT_JITTER_COLOR),
                default_value: DEFAULT_JITTER_VALUE,
                expected_value: None,
            },
            download_chart: ChartConfig {
                label: String::from(DEFAULT_DOWNLOAD_LABEL),
                fill: DEFAULT_FILL,
                border_color: String::from(DEFAULT_DOWNLOAD_COLOR),
                default_value: DEFAULT_DOWNLOAD_VALUE,
                expected_value: Some(ExpectedConfig {
                    label: String::from(DEFAULT_EXPECTED_DOWNLOAD_LABEL),
                    fill: DEFAULT_FILL,
                    border_color: String::from(DEFAULT_EXPECTED_DOWNLOAD_COLOR),
                    value: DEFAULT_EXPECTED_DOWNLOAD_VALUE,
                }),
            },
            upload_chart: ChartConfig {
                label: String::from(DEFAULT_UPLOAD_LABEL),
                fill: DEFAULT_FILL,
                border_color: String::from(DEFAULT_UPLOAD_COLOR),
                default_value: DEFAULT_UPLOAD_VALUE,
                expected_value: Some(ExpectedConfig {
                    label: String::from(DEFAULT_EXPECTED_UPLOAD_LABEL),
                    fill: DEFAULT_FILL,
                    border_color: String::from(DEFAULT_EXPECTED_UPLOAD_COLOR),
                    value: DEFAULT_EXPECTED_UPLOAD_VALUE,
                }),
            },
        }
    }
}

/// read config file (create if not exists).
/// write example to console (if not exists).
pub fn read_config(working_dir: &Path) -> Config {
    let path = working_dir.join(CONFIG_FILENAME);
    let maybe_config: Result<Config, ConfyError> = confy::load_path(&path);
    match maybe_config {
        Err(ex) => panic!("Abort {:?}", ex),
        Ok(cfg) => cfg,
    }
}

/// init logger to write in the log file.
/// rotate log file after log_file_max_length_in_kb
pub fn init_logger(config: &Config) {
    match Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory(&config.log_file))
        .format(opt_format)
        .append()
        .rotate(
            Criterion::Size(config.log_file_max_length_in_kb * 1024),
            Naming::Numbers,
            Cleanup::Never,
        )
        .start()
    {
        Err(_) => (),
        Ok(_) => (),
    }
}

/// transform config to setup to run it in mode 1
pub fn config_to_setup_for_mode_1(working_dir: &Path, config: Config) -> Setup {
    //last data file name is:
    let now = chrono::Local::now();
    let today: NaiveDate = now.naive_local().date();
    let last_filter_file_name: String = get_data_file_name(&today);

    //last_data_file with full path:
    let new_data_file: String = Path::new(&config.data_dir)
        .join(&last_filter_file_name)
        .to_str()
        .unwrap()
        .to_string();

    //first data file name is:
    let past = now - Duration::days(config.output_xdays as i64);
    let pastday: NaiveDate = past.naive_local().date();
    let first_filter_file_name: String = get_data_file_name(&pastday);

    //do some checks first:
    if !check_path_full_access(Path::new(&config.data_dir))
        | !check_file_full_access(Path::new(&new_data_file))
        | !check_file_full_access(Path::new(&config.output_file))
    {
        panic!("PLEASE CORRECT CONFIG!");
    }

    Setup {
        data_dir: config.data_dir,
        new_data_file: Some(new_data_file),
        first_filter_file_name: first_filter_file_name,
        last_filter_file_name: last_filter_file_name,
        from_date: pastday,
        to_date: today,
        output_file: config.output_file,
        working_dir: working_dir.to_str().unwrap().to_string(),
        latency_chart: config.latency_chart,
        jitter_chart: config.jitter_chart,
        download_chart: config.download_chart,
        upload_chart: config.upload_chart,
    }
}

/// transform config to setup to run it in mode 2
pub fn config_to_setup_for_mode_2(
    working_dir: &Path,
    config: Config,
    from_date: &str,
    to_date: &str,
    output_file: &str,
) -> Setup {
    //note: console errors are ok here since it is used as console command:
    let from_date_as_nd = NaiveDate::parse_from_str(from_date, DATE_FORMAT).expect(&format!(
        "Invalid date format: {} (e.g. 2022-01-15)",
        from_date
    ));
    let to_date_as_nd = NaiveDate::parse_from_str(to_date, DATE_FORMAT).expect(&format!(
        "Invalid date format: {} (e.g. 2022-01-15)",
        to_date
    ));

    if from_date_as_nd > to_date_as_nd {
        panic!(
            "from_date > to_date,  {} > {} ",
            from_date_as_nd, to_date_as_nd
        );
    }

    let first_filter_file_name = get_data_file_name(&from_date_as_nd);
    let last_filter_file_name = get_data_file_name(&to_date_as_nd);

    Setup {
        data_dir: config.data_dir,
        new_data_file: None,
        first_filter_file_name: first_filter_file_name,
        last_filter_file_name: last_filter_file_name,
        from_date: from_date_as_nd,
        to_date: to_date_as_nd,
        output_file: output_file.to_string(),
        working_dir: working_dir.to_str().unwrap().to_string(),
        latency_chart: config.latency_chart,
        jitter_chart: config.jitter_chart,
        download_chart: config.download_chart,
        upload_chart: config.upload_chart,
    }
}

//create a file name from a date
//all the data from one month is stored in the same file
//the name is in a format so that names are ordered according to the timeline
fn get_data_file_name(date: &NaiveDate) -> String {
    date.format(DATE_FILE_NAME_FORMAT).to_string()
}

fn check_path_full_access(path: &Path) -> bool {
    if !path.exists() {
        print_and_log_error(format!("Path does not exists: {:?}", path));
        return false;
    }
    let mut rs = true;
    if !path.readable() {
        print_and_log_error(format!("No read permission on path: {:?}", path));
        rs = false;
    }
    if !path.writable() {
        print_and_log_error(format!("No write permission on path: {:?}", path));
        rs = false;
    }
    rs
}

fn check_file_full_access(file: &Path) -> bool {
    let dir = match file.parent() {
        Some(d) => d,
        None => {
            print_and_log_error(format!("File has an invalid path: {:?}", file));
            return false;
        }
    };
    let rs = check_path_full_access(dir);
    if rs {
        if file.exists() {
            if !file.writable() {
                print_and_log_error(format!("No write permission for file: {:?}", file));
            }
        }
    }
    rs
}

fn print_and_log_error(msg: String) {
    println!("{}", msg);
    error!("{}", msg);
}

fn print_and_log_info(msg: String) {
    println!("{}", msg);
    info!("{}", msg);
}

/// run the speed_test and append the json output to a data_file
fn run_speed_test(working_dir: &Path, output_file: &Path) {
    let start: String = Local::now().format(DATE_TIME_FORMAT).to_string();
    let output_rs = Command::new(working_dir.join(SPEED_TEST_CMD).to_str().unwrap()).output();
    let stop: String = Local::now().format(DATE_TIME_FORMAT).to_string();
    match output_rs {
        Ok(output) => {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            if output.status.success() {
                let json = String::from_utf8_lossy(&output.stdout);
                match append_json_to_file(output_file, &json) {
                    Ok(()) => {
                        if !err_msg.trim().eq("") {
                            // speed_test run normally but could not find a server e.g:
                            print_and_log_error(format!(
                                "run_speed_test ERROR from {} to {} message = '{}'",
                                start, stop, &err_msg
                            ));
                        } else {
                            // everything is fine:
                            print_and_log_info(format!(
                                "run_speed_test OK from {} to {}",
                                start, stop
                            ));
                        }
                    }
                    Err(err) => print_and_log_error(format!(
                        // could not write speed_test result:
                        "run_speed_test ERROR from {} to {} message = '{}'",
                        start, stop, &err
                    )),
                };
            } else {
                // speed_test crashed:
                print_and_log_error(format!(
                    "run_speed_test ERROR from {} to {} message = '{}'",
                    start, stop, &err_msg
                ));
            }
        }
        // could not get any output speed_test
        Err(e) => print_and_log_error(format!(
            "run_speed_test ERROR from {} to {} message = '{}'",
            start,
            stop,
            &e.to_string()
        )),
    }
}

/// append to a file, create the output_file if it does not exist
fn append_json_to_file(output_file: &Path, json: &str) -> Result<(), Box<dyn Error>> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open(output_file)?;
    file.write_all(json.as_bytes())?;
    file.flush()?;
    Ok(())
}

///read paths from the data_dir that have a name that is
///first_filter_file_name <= file_name <= last_filter_file_name
fn read_data_file_paths(
    data_dir: &Path,
    first_filter_file_name: &str,
    last_filter_file_name: &str,
) -> Option<Vec<String>> {
    let mut rs: Vec<String> = fs::read_dir(data_dir)
        .ok()?
        .filter_map(|entry| {
            Some(
                entry
                    .ok()?
                    .path()
                    .strip_prefix(data_dir)
                    .ok()?
                    .to_str()
                    .unwrap()
                    .to_string(),
            )
        })
        .filter(|file_name| {
            (first_filter_file_name <= file_name) && (last_filter_file_name >= file_name)
        })
        .map(|file_name| {
            Path::new(data_dir)
                .join(file_name)
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect();
    rs.sort();
    Some(rs)
}

fn parse_output_file(
    data_dir: &Path,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
) -> Option<Vec<ParsedEntry>> {
    let file = fs::File::open(data_dir).ok()?;
    let reader = BufReader::new(file);
    let rs: Vec<ParsedEntry> = reader
        .lines()
        .flat_map(|read_line| {
            match read_line {
                Ok(line) => {
                    match JsonParser::parse(&line) {
                        //Filter:
                        Ok(parsed_entry) => {
                            let entry_date = &parsed_entry.timestamp.date();
                            if from_date <= entry_date && entry_date <= to_date {
                                Some(parsed_entry)
                            } else {
                                None
                            }
                        }
                        Err(e) => {
                            error!("Could not parse json: '{}', message = '{}'", line, e);
                            None
                        }
                    }
                }
                Err(e) => {
                    error!("could not read data line message = '{}'", e);
                    None
                }
            }
        })
        .collect();
    Some(rs)
}
