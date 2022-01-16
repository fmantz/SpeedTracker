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

// program names:
pub const PROGRAM_NAME: &'static str = "speedtracker";
pub const EXIT_SUCCESS: i32 = 0;

pub const SPEED_TEST_CMD: &'static str = "speedtestJson";

// file names:
pub const CONFIG_FILENAME: &'static str = "speedtracker.toml";
pub const TEMPLATE_FILENAME: &'static str = "template.html";

// date formats:
pub const DATE_FILE_NAME_FORMAT: &'static str = "%Y-%m-DATA.json";
pub const DATE_FORMAT: &'static str = "%Y-%m-%d";
pub const DATE_TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

// config defaults:
pub const DEFAULT_DATA_DIR: &'static str = "./";
pub const DEFAULT_OUTPUT_FILE: &'static str = "./index.html";
pub const DEFAULT_OUTPUT_XDAYS: u32 = 14;
pub const DEFAULT_LOG_FILE: &'static str = "./speedtracker.log";
pub const DEFAULT_LOG_FILE_MAX_LENGTH_IN_KB: u64 = 8096;

// charts:
pub const DEFAULT_FILL: bool = false;

// latency:
pub const DEFAULT_LATENCY_LABEL: &'static str = "latency";
pub const DEFAULT_LATENCY_COLOR: &'static str = "green";
/// in milliseconds
pub const DEFAULT_LATENCY_VALUE: u32 = 30;

// jitter:
pub const DEFAULT_JITTER_LABEL: &'static str = "jitter";
pub const DEFAULT_JITTER_COLOR: &'static str = "red";
/// in milliseconds
pub const DEFAULT_JITTER_VALUE: u32 = 30;

// download:
pub const DEFAULT_DOWNLOAD_LABEL: &'static str = "download";
pub const DEFAULT_DOWNLOAD_COLOR: &'static str = "green";
/// in bits per second, note 1 megabit = 1000000 bits bits per second
pub const DEFAULT_DOWNLOAD_VALUE: f64 = 0.0;
pub const DEFAULT_EXPECTED_DOWNLOAD_LABEL: &'static str = "expected download";
pub const DEFAULT_EXPECTED_DOWNLOAD_COLOR: &'static str = "lime";
/// in bits per second, note 1 megabit = 1000000 bits bits per second
pub const DEFAULT_EXPECTED_DOWNLOAD_VALUE: f64 = 250000000.0;

// upload:
pub const DEFAULT_UPLOAD_LABEL: &'static str = "upload";
pub const DEFAULT_UPLOAD_COLOR: &'static str = "red";
/// in bits per second, note 1 megabit = 1000000 bits bits per second
pub const DEFAULT_UPLOAD_VALUE: f64 = 0.0;
pub const DEFAULT_EXPECTED_UPLOAD_LABEL: &'static str = "expected upload";
pub const DEFAULT_EXPECTED_UPLOAD_COLOR: &'static str = "orange";
/// in bits per second, note 1 megabit = 1000000 bits bits per second
pub const DEFAULT_EXPECTED_UPLOAD_VALUE: f64 = 25000000.0;

pub const REPLACEMENT_REGEX: &'static str = r"\$\{[[:alpha:]]*\}";
pub const REPLACEMENT_ID_STATISTICS: &'static str = "STATISTICS";
pub const REPLACEMENT_ID_RAW_DATA: &'static str = "RAW_DATA";
pub const REPLACEMENT_ID_RESPONSE_TIMES: &'static str = "RESPONSE_TIMES";
pub const REPLACEMENT_ID_THROUGHPUT: &'static str = "THROUGHPUT";
