use std::path::Path;

use crate::json_parser::*;

pub struct HtmlGenerator {}

impl HtmlGenerator {
    pub fn write_html(data: &Vec<ParsedEntry>, template_path: &Path, output_file: &Path) {
        println!("Test"); //TODO: write error also to console
    }
}
