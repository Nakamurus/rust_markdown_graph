use crate::Title;

pub fn relation_builder<'a>(titles: &'a Vec<Title>) -> Vec<(&'a str, &'a str)> {
    let mut connected_hierarchy = Vec::new();
    let mut outline_path = vec![""; 7];
    for i in 0..titles.len() - 1 {
        outline_path[titles[i].level] = &titles[i].content;
        if titles[i].level >= titles[i + 1].level {
            outline_path[titles[i].level + 1..].fill("");
            for j in (1..outline_path.len()).take_while(|&x| outline_path[x] != "") {
                connected_hierarchy.push((outline_path[j - 1], outline_path[j]));
            }
        }
    }
    outline_path[titles.last().unwrap().level..].fill("");
    for j in (1..outline_path.len()).take_while(|&x| outline_path[x] != "") {
        connected_hierarchy.push((outline_path[j - 1], outline_path[j]));
    }
    connected_hierarchy
}
