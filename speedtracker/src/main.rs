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

use mylib::*;
use std::env;
use std::path::Path;
use std::process;

const PROGRAM_NAME: &'static str = "speedtracker";
const SUCCESS: i32 = 0;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len != 2 && args_len != 4 {
        print_usage();
        process::exit(SUCCESS);
    } else {
        let working_dir: &Path = Path::new(&args[0]).parent().unwrap();
        let config = read_config(working_dir);

        init_logger(&config);

        // decide setup by command line arguments:
        let setup = match args_len {
            2 if &args[1] == "run" => config_to_setup_for_mode_1(working_dir, config),
            4 => config_to_setup_for_mode_2(working_dir, config, &args[1], &args[2], &args[3]),
            _ => {
                print_usage();
                process::exit(SUCCESS);
            }
        };

        // run speed test:
        //setup.maybe_speed_test();

        // parse and filter data:
        setup.read_data();

        //write output
    }
}

fn print_usage() {
    println!("FOR MODE 1 'speedtest + standard.html generation' run:");
    println!("{} run\n", PROGRAM_NAME);
    println!("FOR MODE 2: 'only' output.html generation run:");
    println!("{} from_date to_date output_file\n", PROGRAM_NAME);
    println!("e.g. {} 2022-01-01 2021-12-31 ./index.html\n", PROGRAM_NAME);
}
