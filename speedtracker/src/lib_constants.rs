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

pub const SPEED_TEST_CMD: &str = "speedtestJson";

// file names:
pub const CONFIG_FILENAME: &str = "speedtracker.toml";
pub const TEMPLATE_FILENAME: &str = "template.html";

// date formats:
pub const DATE_FILE_NAME_FORMAT: &str = "%Y-%m-DATA.json";
pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

// config defaults:
pub const DEFAULT_DATA_DIR: &str = "./";
pub const DEFAULT_OUTPUT_FILE: &str = "./index.html";
pub const DEFAULT_OUTPUT_XDAYS: u32 = 14;
pub const DEFAULT_LOG_FILE: &str = "./speedtracker.log";
pub const DEFAULT_LOG_FILE_MAX_LENGTH_IN_KB: u64 = 8096;

// charts:
pub const DEFAULT_FILL: bool = false;

pub const MEGA_BIT_FACTOR: f64 = 1000000.0;
pub const MULT_DIV_NEUTRAL: f64 = 1.0;

// latency:
pub const DEFAULT_LATENCY_LABEL: &str = "latency";
pub const DEFAULT_LATENCY_COLOR: &str = "green";
/// in milliseconds
pub const DEFAULT_LATENCY_VALUE: u32 = 100;

// jitter:
pub const DEFAULT_JITTER_LABEL: &str = "jitter";
pub const DEFAULT_JITTER_COLOR: &str = "red";
/// in milliseconds
pub const DEFAULT_JITTER_VALUE: u32 = 100;

// download:
pub const DEFAULT_DOWNLOAD_LABEL: &str = "download";
pub const DEFAULT_DOWNLOAD_COLOR: &str = "green";
/// in megabits per second, note 1 megabit = 1000000 bits bits per second
pub const DEFAULT_DOWNLOAD_VALUE: f64 = 0.0;
pub const DEFAULT_EXPECTED_DOWNLOAD_LABEL: &str = "expected download";
pub const DEFAULT_EXPECTED_DOWNLOAD_COLOR: &str = "lime";
/// in megabits per second, note 1 megabit = 1000000 bits bits per second
pub const DEFAULT_EXPECTED_DOWNLOAD_VALUE: f64 = 250.0;

// upload:
pub const DEFAULT_UPLOAD_LABEL: &str = "upload";
pub const DEFAULT_UPLOAD_COLOR: &str = "red";
/// in megabits per second, note 1 megabit = 1000000 bits bits per second
pub const DEFAULT_UPLOAD_VALUE: f64 = 0.0;
pub const DEFAULT_EXPECTED_UPLOAD_LABEL: &str = "expected upload";
pub const DEFAULT_EXPECTED_UPLOAD_COLOR: &str = "orange";
/// in megabits per second, note 1 megabit = 1000000 bits bits per second
pub const DEFAULT_EXPECTED_UPLOAD_VALUE: f64 = 25.0;

pub const REPLACEMENT_REGEX: &str = r"\$\{([[:alpha:]]|_)*\}";
pub const REPLACEMENT_ID_STATISTICS: &str = "STATISTICS";
pub const REPLACEMENT_ID_RESPONSE_TIMES: &str = "RESPONSE_TIMES";
pub const REPLACEMENT_ID_THROUGHPUT: &str = "THROUGHPUT";
pub const REPLACEMENT_ID_RAW_DATA: &str = "RAW_DATA";

pub const ID_LATENCY: &str = "latency";
pub const ID_JITTER: &str = "jitter";
pub const ID_DOWNLOAD: &str = "download";
pub const ID_UPLOAD: &str = "upload";

pub const STATISTIC_MEDIAN: &str = "median";
pub const STATISTIC_AVG: &str = "average";
pub const STATISTIC_STD: &str = "standard-deviation";
