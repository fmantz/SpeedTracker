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

use crate::constants::*;

use chrono::naive::NaiveDateTime;
use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Client {
 wlan: Option<String>,
 ip: String,
 lat: String,
 lon: String,
 isp: String
}

#[derive(Debug, Deserialize)]
pub struct Server {
 name: String,
 sponsor: String,
 distance: String,
 host: String,
}

#[derive(Debug, Deserialize)]
pub struct Performance {
 latency: f64,
 jitter:  Option<f64>,
 downloadConfig: Option<String>,
 uploadConfig: Option<String>,
 download: Option<f64>,
 upload: Option<f64>
}

#[derive(Debug, Deserialize)]
pub struct ParsedEntry {

    #[serde(deserialize_with = "naive_date_time_from_str")]
    timestamp: NaiveDateTime,

    client : Option<Client>,
    server : Option<Server>,
    performance: Option<Performance>
}

pub struct Parser {}
impl Parser {
    pub fn parse(serialized_json: &str) {
        let rs: ParsedEntry = serde_json::from_str(&serialized_json).unwrap();
        println!("TEST {:?}", rs);
    }
}

fn naive_date_time_from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, DATE_TIME_FORMAT).map_err(de::Error::custom)
}