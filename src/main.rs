use markdown_outliner::input_handler;
use markdown_outliner::outline_filter;
use markdown_outliner::relation_builder;
use markdown_outliner::view::to_graphviz::visualize;

fn main() {
    let (file_name, depth) = input_handler().unwrap();
    let titles = outline_filter(&file_name, &depth).unwrap();
    let connected = relation_builder(&titles);
    if let Ok(_) = visualize(&file_name, "LR", connected) {
        println!("Created {}.dot successfully", file_name);
    } else {
        println!("Could not create dot file");
    }
}
