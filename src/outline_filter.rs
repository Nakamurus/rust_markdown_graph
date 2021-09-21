use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::result::Result;

use crate::Title;

pub fn outline_filter<P>(filename: P, level: &str) -> Result<Vec<Title>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename)?;
    let titles: Vec<Title> = lines
        .filter_map(|line| line.ok())
        .filter_map(|line| capture_title(&line, level))
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

fn capture_title(line: &str, level: &str) -> Option<Title> {
    let heading_pattern = format!(r"^#{{1,{}}} .+", level);
    let re = Regex::new(&heading_pattern).unwrap();
    if re.is_match(line) {
        let content_index = line.find(" ").unwrap();
        let (heading_level, heading) = line.split_at(content_index);
        return Some(Title {
            level: heading_level.len() - 1,
            content: heading.to_string(),
        });
    }
    None
}
