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
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

const SPEED_TEST_CMD: &'static str = "./SpeedTest/speedtestJson";
const CONFIG_FILENAME: &'static str = "speedtracker.toml";

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

/// transform config to setup to run it in mode 1
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
    }
}

/// transform config to setup to run it in mode 2
pub fn config_to_setup_for_mode_2(
    config: Config,
    from_date: &str,
    to_date: &str,
    output_file: &str,
) -> Setup {
    //note: console errors are ok here since it is used as console command:
    let from_date_as_nd = NaiveDate::parse_from_str(from_date, "%Y-%m-%d").expect(&format!(
        "Invalid date format: {} (e.g. 2022-01-15)",
        from_date
    ));
    let to_date_as_nd = NaiveDate::parse_from_str(to_date, "%Y-%m-%d").expect(&format!(
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
    }
}

//create a file name from a date
//all the data from one month is stored in the same file
//the name is in a format so that names are ordered according to the timeline
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

fn check_file_full_access(file: &Path) -> bool {
    let dir = match file.parent() {
        Some(d) => d,
        None => {
            error!("File has an invalid path: {}", file.display());
            return false;
        }
    };
    let mut rs = check_path_full_access(dir);
    if rs {
        if file.exists() {
            if !file.writable() {
                error!("No write permission for file: {}", file.display());
            }
        }
    }
    rs
}

pub fn run_speed_test(output_file: &Path) {
    let start: Instant = Instant::now();
    let output_rs = Command::new(SPEED_TEST_CMD).output();
    let stop: Instant = Instant::now();
    match output_rs {
        Ok(output) => {
            if output.status.success() {
                let json = String::from_utf8_lossy(&output.stdout);
                match append_json_to_file(output_file, &json) {
                    Ok(()) => info!("run_speed_test OK from {:?} to {:?}", start, stop),
                    Err(err) => error!(
                        "run_speed_test ERROR from {:?} to {:?} message = {}",
                        start, stop, &err
                    ),
                };
            } else {
                let err_msg = String::from_utf8_lossy(&output.stdout);
                error!(
                    "run_speed_test ERROR from {:?} to {:?} message = {}",
                    start, stop, &err_msg
                );
            }
        }
        Err(e) => error!(
            "run_speed_test ERROR from {:?} to {:?} message = {}",
            start,
            stop,
            &e.to_string()
        ),
    }
}

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
