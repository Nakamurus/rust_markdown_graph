use clap::{App, Arg, Error};

pub fn input_handler<'a>() -> Result<(String, String, String), Error> {
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
            Arg::with_name("level")
                .takes_value(true)
                .short("l")
                .default_value("6")
                .help("Heading level. level 1 will catch only H1 title")
                .possible_values(&["1", "2", "3", "4", "5", "6"]),
        )
        .arg(
            Arg::with_name("direction")
                .takes_value(true)
                .short("d")
                .default_value("LR")
                .help(
                    "Graphviz image direction. L=Left, R=Right, T=Top, B=Bottom. LR=Left to Right",
                )
                .possible_values(&["LR", "RL", "TB", "BT"]),
        )
        .get_matches();
    let file_name = matches.value_of("INPUT").unwrap().to_string();
    let level = matches.value_of("level").unwrap().to_string();
    let direction = matches.value_of("direction").unwrap().to_string();
    Ok((file_name, level, direction))
}
