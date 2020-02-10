use std::env::args;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn get_title() -> String{
    env!("CARGO_PKG_NAME").to_string()
}

fn get_banner() -> String {
    let title = get_title();
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");
    let mut banner = title;
    banner.push_str(" (v");
    banner.push_str(version);
    banner.push_str("), ");
    banner.push_str(description);
    return banner;
}

fn usage() {
    let title = get_title();
    let authors = env!("CARGO_PKG_AUTHORS");
    println!("{}", get_banner());
    println!("Written by {}", authors);
    println!("Usage: {} <somefile.md>", title);
}


fn parse_line(line_contents:&str) -> String {
    let mut _output_line = String::new();
    let _line_chars : Vec<char> = line_contents.chars().collect();
    match _line_chars.len() {
        0 => {},
        _ => {
            match _line_chars[0] {
                '#' => {
                        _output_line.push_str("<H1>");
                        match _line_chars[1] {
                            ' ' =>  _output_line.push_str(&line_contents[2..]),
                            _   =>  _output_line.push_str(&line_contents[1..])
                        }
                        _output_line.push_str("</H1>");
                    },
                _   => {
                        _output_line.push_str("<P>");
                        _output_line.push_str(&line_contents[0..]);
                        _output_line.push_str("</P>");
                    }
            }
        }
    }
    return _output_line;
}


fn parse_file(_filename:&str) -> Vec<String> {
    println!("{}", get_banner());
    println!("Parsing {} ...", _filename);
    let _input_filename = Path::new(_filename);
    let _file = File::open(&_input_filename).expect("[ ERROR ] Failed to open file.");
    let _htag = false;
    let _ptag = false;
    let mut _tokens : Vec<String> = Vec::new();
    let reader = BufReader::new(_file);
    for line in reader.lines() {
        _tokens.push(parse_line(&line.unwrap()));
    }

    return _tokens;
}

fn save_to_file(_tokens : Vec<String>,_filename:&str) {
    let _end_idx = _filename.len()-3;
    let mut _new_file_name = String::new();
    _new_file_name.push_str(&_filename[.._end_idx]);
    _new_file_name.push_str(".html");
    let mut _output_file = File::create(_new_file_name).expect("[ERROR] Could not create file!");
    for line in _tokens {
        _output_file.write_all(line.as_bytes()).expect("Could not write to output file!");
    }
}

fn main() {
    //let args : Vec<String> = std::env::args().collect();
    let args : Vec<String> = args().collect();
    match args.len() {
        2 => {
                let _tokens = parse_file(&args[1]);
                save_to_file(_tokens, &args[1]);
                println!("Done parsing & saving to html.");
            },
        _ => usage()
    }
}
