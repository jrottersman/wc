extern crate clap;
use clap::{App, Arg};

use std::fs;

use wc;

fn main() {
    /* TODO: Create flags
     * READ File
     * Split file by char if -c by new line if -l or by space default
     * count number of elements that are produced
     * output count to stdout
    */
    
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
            .index(1)
            .required(true),
            )
        .get_matches();


    let file = matches.value_of("input").unwrap();
    println!("Input is: {}", file);
    let contents = fs::read_to_string(file)
        .expect("Someting went wrong reading the file");
    println!("file contains: {}", contents);

    if matches.is_present("chars") {
        println!("by char");
        wc::wc_char(&contents);
    } else if matches.is_present("lines") {
        println!("by line");
        wc::wc_line(&contents);
    } else if matches.is_present("bytes") {
        println!("by bytes");
        wc::wc_byte(&contents);
    } else {
        println!("by word");
        wc::wc_word(&contents);
    }

}
