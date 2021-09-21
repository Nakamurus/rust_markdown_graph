use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn visualize(
    file_name: &str,
    direction: &str,
    connected: Vec<(&str, &str)>,
) -> std::io::Result<()> {
    let output = format!("{}.dot", file_name);
    let f = File::create(output).expect("unable to create file");
    let mut f = BufWriter::new(f);
    writeln!(f, "digraph {{").unwrap();
    writeln!(f, "rankdir = {}", direction).unwrap();
    for i in ["graph", "node", "edge"].iter() {
        writeln!(f, r#"{} [fontname = "UD デジタル 教科書体"]"#, i).unwrap();
    }
    let mut used = BTreeSet::new();
    for (a, b) in connected.iter() {
        if used.insert((a, b)) {
            writeln!(f, "{} -> {}", a, b).unwrap();
        }
    }
    writeln!(f, "}}")
}
