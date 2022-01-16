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
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartConfig<T> {
    /// id that is used to position the output in the template
    pub id: String,
    /// label for the data
    pub label: String,
    /// fill the data or not
    pub fill: bool,
    /// color of the line
    pub border_color: String,
    /// default value is a value is missing
    pub default_value: T,
    /// expect data:
    pub expected_value: Option<ExpectedConfig<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedConfig<T> {
    /// label for the data
    pub label: String,
    /// fill the data or not
    pub fill: bool,
    /// color of the line
    pub border_color: String,
    /// default value is a value is missing
    pub value: T,
}
