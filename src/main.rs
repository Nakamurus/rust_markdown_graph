use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let matches = App::new("markdown_outliner")
        .version("0.1")
        .author("Nakamura")
        .about("parse markdown file to create file outline")
        .arg(
            Arg::with_name("INPUT")
                .help("File path to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("depth")
                .takes_value(true)
                .short("d")
                .default_value("6")
                .help("Outline depth. depth 1 will catch only H1 title")
                .possible_values(&["1", "2", "3", "4", "5", "6"]),
        )
        .get_matches();
    let file_name: &str = matches
        .value_of("INPUT")
        .expect("Please specify a markdown file argument");
    let depth: &str = matches.value_of("depth").unwrap();

    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(ip) = line {
                if let Some(title) = capture_title(&ip, depth) {
                    println!("{:?}", title);
                }
            }
        }
    } else {
        panic!("File does not exists in the specified path");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn capture_title(line: &str, depth: &str) -> Option<(usize, String)> {
    let title_pattern = format!(r"^#{{1,{}}} .+", depth);
    let re = Regex::new(&title_pattern).unwrap();

    if let Some(cap) = re.captures(line) {
        let symbol_index = cap[0].find(" ").unwrap();
        let (title_depth, title) = cap[0].split_at(symbol_index);

        return Some((title_depth.len(), title.to_string()));
    }
    None
}
