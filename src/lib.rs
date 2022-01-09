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
use std::process::Command;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

const SPEED_TEST_CMD: &str = "./SpeedTest/speedtestJson";
const _CONFIG_FILENAME: &str = "speedtracker.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config{
  /// directory where data is collected
  data_dir                  : String,
  /// should be on a path served by a webserver (apache e.g.)
  output_file               : String,
  /// number of days in the past (from today) the data should be visualized
  output_xdays              : u32,
  /// log file path and name
  log_file                  : String,
  /// maximal size of log file in kb before the file is rotated
  log_file_max_length_in_kb : u64,
}

#[derive(Debug)]
pub struct Setup{
  /// directory where data is collected
  data_dir                : String,
  /// new results should be appended to this file
  new_data_file           : Option<String>,
  /// first file name with relevant data e.g. '2022-01.json'
  first_filter_file_name  : String,
  /// last file name with relevant data e.g. '2022-02.json'
  last_filter_file_name   : String,
  /// start date from which the data should be vizualized
  from_date               : NaiveDate,
  /// end date from which the data should be vizualized
  to_date                 : NaiveDate,
  /// should be on a path served by a webserver (apache e.g.)
  output_file             : String,
}

/// good defaults for the log file, note password must be set!
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            data_dir     : String::from("./data"),
            output_file  : String::from("./index.html"),
            /// two weeks
            output_xdays : 14,
            /// standard log directory
            log_file: String::from("/var/log/speedtracker.log"),
            /// maximal size of log file in kb before the file is rotated
            log_file_max_length_in_kb: 8096,
        }
    }
}


pub fn read_config() -> Result<Config, String> {
   //TODO
    Err("".to_string())
}

pub fn config_to_setup(config: &Config) -> Result<Setup, String> {
   //TODO
   Err("".to_string())
}

pub fn update_setup(setup: &mut Setup, output_file: &str, from_date: &str, to_date: &str) {
   //TODO
}

pub fn run_speed_test() {
    let start: Instant = Instant::now();
    let output_rs = Command::new(SPEED_TEST_CMD).output();
    let stop: Instant = Instant::now();
    match output_rs {
       Ok(output) =>
           if output.status.success() {
              let json = String::from_utf8_lossy(&output.stdout);
              println!("stdout: {}", json);
              my_log(&start, &stop, Ok("jo jo"));
           } else {
              let err_msg = String::from_utf8_lossy(&output.stdout);
              my_log(&start, &stop, Err(&err_msg));
           },
       Err(e) =>
          my_log(&start, &stop, Err(&e.to_string())),
    }
}

fn my_log(start: &Instant, stop: &Instant, result:Result<&str, &str>) {
   match result {
      Ok(m)   =>  println!("jo jo {}", m),
      Err(e) =>  println!("no no {}", e),
   }
}