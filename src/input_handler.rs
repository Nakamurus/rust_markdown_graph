use clap::{App, Arg, Error};

pub fn input_handler<'a>() -> Result<(String, String), Error> {
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
    let file_name = matches.value_of("INPUT").unwrap().to_string();
    let depth = matches.value_of("depth").unwrap().to_string();
    Ok((file_name, depth))
}
