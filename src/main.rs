use markdown_outliner::input_handler;
use markdown_outliner::outline_filter;
use markdown_outliner::relation_builder;
use markdown_outliner::view::to_graphviz::visualize;

fn main() {
    let (filename, level, direction) = input_handler().unwrap();
    let titles = outline_filter(&filename, &level).unwrap();
    let connected = relation_builder(&titles);
    let filename_wo_extension = filename.split('.').next().unwrap();
    if let Ok(_) = visualize(&filename_wo_extension, &direction, connected) {
        println!("Created {}.dot successfully", filename_wo_extension);
    } else {
        println!("Could not create dot file");
    }
}
