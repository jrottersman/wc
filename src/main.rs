extern crate clap;

use std::process;
use clap::{App, Arg};
use wc_jake_toy;

fn main() {

    
    let matches = App::new("wc")
        .version("0.1")
        .author("Jake Rottersman <rottersmanjake@gmail.com>")
        .about("Count words")
        .arg(
            Arg::with_name("lines")
            .help("count by lines")
            .short("l"),
            )
       .arg(
             Arg::with_name("chars")
            .help("Count by chars")
            .short("c"),
            )
       .arg(
           Arg::with_name("bytes")
           .help("Count by bytes")
           .short("b"),
           )
        .arg(
            Arg::with_name("input")
            .help("the input file to use")
            .index(1),
            )
        .get_matches();

    if let Err(e) = wc_jake_toy::run(matches) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

}
