use clap::{App, Arg};

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
        .get_matches();

    if let Some(i) = matches.value_of("INPUT") {
        println!("Value for input: {}", i);
    }
}
