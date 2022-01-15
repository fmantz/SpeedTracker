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
