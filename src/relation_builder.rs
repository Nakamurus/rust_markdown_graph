use crate::Title;

pub fn relation_builder<'a>(titles: &'a Vec<Title>) -> Vec<(&'a str, &'a str)> {
    let mut connected_hierarchy = Vec::new();
    let mut heading_path = vec![""; 7];
    for i in 0..titles.len() - 1 {
        heading_path[titles[i].level] = &titles[i].content;
        if titles[i].level >= titles[i + 1].level {
            heading_path[titles[i].level + 1..].fill("");
            for j in (1..heading_path.len()).take_while(|&x| heading_path[x] != "") {
                connected_hierarchy.push((heading_path[j - 1], heading_path[j]));
            }
        }
    }
    heading_path[titles.last().unwrap().level..].fill("");
    for j in (1..heading_path.len()).take_while(|&x| heading_path[x] != "") {
        connected_hierarchy.push((heading_path[j - 1], heading_path[j]));
    }
    connected_hierarchy
}
