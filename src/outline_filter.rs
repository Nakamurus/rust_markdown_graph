use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::result::Result;

use crate::Title;

pub fn outline_filter<P>(file_name: P, depth: &str) -> Result<Vec<Title>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let lines = read_lines(file_name)?;
    let titles: Vec<Title> = lines
        .filter_map(|line| line.ok())
        .filter_map(|line| capture_title(&line, depth))
        .collect();
    if titles.is_empty() {
        Err("No outlines in the specified file".into())
    } else {
        Ok(titles)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn capture_title(line: &str, depth: &str) -> Option<Title> {
    let title_pattern = format!(r"^#{{1,{}}} .+", depth);
    let re = Regex::new(&title_pattern).unwrap();

    if let Some(cap) = re.captures(line) {
        let symbol_index = cap[0].find(" ").unwrap();
        let (title_depth, title) = cap[0].split_at(symbol_index);

        return Some(Title {
            level: title_depth.len() - 1,
            content: title.to_string(),
        });
    }
    None
}
