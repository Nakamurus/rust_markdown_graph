use Title
pub fn visualize_tree(titles: Vec<Title>) {
    println!("Title: {}", titles[0]);
    for &title in titles.iter() {
        let prefix = "{}│   ".repeat(title.level);
        println!("{}└── {}", prefix, title.content);
    }
}

// fn walk(paths: &Vec<String>, prefix: &str, visited:&mut BTreeSet<&str>) {
//     for &path in paths.iter() {
//         if !visited[&path] {
//             println!("{}└── {}", prefix, path);
//             visited.insert(&path);

//         } else {

//         }
//     }
// }