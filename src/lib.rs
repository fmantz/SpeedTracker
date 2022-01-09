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
use std::path::Path;
use std::process::Command;
use std::time::Instant;

const SPEED_TEST_CMD: &str = "./SpeedTest/speedtestJson";
const CONFIG_FILENAME: &str = "speedtracker.toml";

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
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            data_dir: String::from("./data"),
            output_file: String::from("./index.html"),
            /// two weeks
            output_xdays: 14,
            log_file: String::from("./speedtracker.log"),
            /// maximal size of log file in kb before the file is rotated
            log_file_max_length_in_kb: 8096,
        }
    }
}

/// read config file (create if not exists).
/// write example to console (if not exists).
pub fn read_config() -> Config {
    let path = env::current_dir().unwrap().join(CONFIG_FILENAME);
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

pub fn config_to_setup_for_mode_1(config: Config) -> Setup {
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

    let rs = Setup {
        data_dir: config.data_dir,
        new_data_file: Some(new_data_file),
        first_filter_file_name: first_filter_file_name,
        last_filter_file_name: last_filter_file_name,
        from_date: pastday,
        to_date: today,
        output_file: config.output_file,
    };

    if check_path_full_access(Path::new(&rs.data_dir))
        | check_path_full_access(Path::new(&rs.output_file))
    {
        panic!("PLEASE CORRECT CONFIG!");
    }

    rs
}

//pub fn config_to_setup_for_mode_2(config: &Config, output_file: &str, from_date: &str, to_date: &str) -> Setup {
//    //TODO
//}

fn get_data_file_name(date: &NaiveDate) -> String {
    date.format("%Y-%m-DATA.json").to_string()
}

fn check_path_full_access(path: &Path) -> bool {
    if !path.exists() {
        error!("Path does not exists: {}", path.display());
        return false;
    }
    let mut rs = true;
    if !path.readable() {
        error!("No read permission on path: {}", path.display());
        rs = false;
    }
    if !path.writable() {
        error!("No write permission on path: {}", path.display());
        rs = false;
    }
    rs
}

pub fn run_speed_test() {
    let start: Instant = Instant::now();
    let output_rs = Command::new(SPEED_TEST_CMD).output();
    let stop: Instant = Instant::now();
    match output_rs {
        Ok(output) => {
            if output.status.success() {
                let json = String::from_utf8_lossy(&output.stdout);
                println!("stdout: {}", json);
                my_log(&start, &stop, Ok("jo jo"));
            } else {
                let err_msg = String::from_utf8_lossy(&output.stdout);
                my_log(&start, &stop, Err(&err_msg));
            }
        }
        Err(e) => my_log(&start, &stop, Err(&e.to_string())),
    }
}

fn my_log(start: &Instant, stop: &Instant, result: Result<&str, &str>) {
    match result {
        Ok(m) => println!("jo jo {}", m),
        Err(e) => println!("no no {}", e),
    }
}
