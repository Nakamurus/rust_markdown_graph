use clap::{App, Arg};
use petgraph::dot::{Config, Dot};
use petgraph::Graph;
use regex::Regex;
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct Title {
    level: usize,
    content: String,
}

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
    let mut titles: Vec<Title> = Vec::new();
    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(ip) = line {
                if let Some(title) = capture_title(&ip, depth) {
                    titles.push(title);
                }
            }
        }
        let relations: Vec<(&str, &str)> = dfs(&titles, 0, &mut HashSet::new(), &mut vec![""; 6])
            .into_iter()
            .collect();
        let mut graph: Graph<&str, u32> = Graph::new();
        let mut added = BTreeMap::new();
        for (a, b) in relations.iter() {
            let x = if added.contains_key(a) {
                *added.get(a).unwrap()
            } else {
                graph.add_node(a)
            };
            let y = if added.contains_key(b) {
                *added.get(b).unwrap()
            } else {
                graph.add_node(b)
            };
            added.insert(a, x);
            added.insert(b, y);
            graph.update_edge(x, y, 0);
        }
        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        let mut f = File::create("example.dot").unwrap();
        let output = format!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        f.write_all(&output.as_bytes())
            .expect("could not write file");
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

fn dfs<'a>(
    graph: &'a Vec<Title>,
    cur: usize,
    relations: &mut HashSet<(&'a str, &'a str)>,
    path: &mut Vec<&'a str>,
) -> HashSet<(&'a str, &'a str)> {
    path[graph[cur].level..].fill("");
    path[graph[cur].level] = &graph[cur].content;
    if cur == graph.len() - 1 {
        for r in (1..path.len()).take_while(|&x| path[x] != "") {
            relations.insert((path[r - 1], path[r]));
        }
        let res = relations.clone();
        return res;
    }
    if graph[cur].level >= graph[cur + 1].level {
        for r in (1..path.len()).take_while(|&x| path[x] != "") {
            relations.insert((path[r - 1], path[r]));
        }
    }
    dfs(graph, cur + 1, relations, path)
}
