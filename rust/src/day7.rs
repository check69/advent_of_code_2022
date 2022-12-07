#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;
    use std::{
        collections::HashMap, env::current_exe, hash::Hash, path::Path, thread::current, vec,
    };

    fn add_size_to_parent(parent_dir: String, current_dir: &str, folder_tree: &mut HashMap<String, usize>) {
        let entry = folder_tree
            .entry(parent_dir.clone() + "/" + current_dir)
            .or_insert(0);
        *folder_tree.entry(parent_dir).or_insert(0) += *entry;
    }

    fn get_folder_tree(data: Vec<String>) -> HashMap<String, usize> {
        let mut folder_tree: HashMap<String, usize> = HashMap::new();
        let mut current_dir: &str = "";
        let mut dir_tree: Vec<&str> = vec![];
        for line in &data {
            let x = line.split(" ").collect::<Vec<&str>>();
            match x.as_slice() {
                &["$", "cd", ".."] => {
                    add_size_to_parent(dir_tree.join("/"), current_dir, &mut folder_tree);
                    current_dir = dir_tree.pop().unwrap();
                }
                &["$", "cd", dir] => {
                    dir_tree.push(current_dir);
                    current_dir = dir;
                }
                &["$", "ls"] => {}
                &["dir", name] => {}
                &[size, name] => {
                    let dir_name = dir_tree.join("/") + "/" + current_dir;
                    let mut entry = folder_tree.entry(dir_name).or_insert(0);
                    *entry += size.parse::<usize>().unwrap();
                }
                _ => {}
            }
        }

        while dir_tree.len() > 0 {
            add_size_to_parent(dir_tree.join("/"), current_dir, &mut folder_tree);
            current_dir = dir_tree.pop().unwrap();
        }

        folder_tree
    }

    fn part1(data: Vec<String>) -> usize {
        get_folder_tree(data)
            .into_values()
            .filter(|v| *v < 100000)
            .sum()
    }

    fn part2(data: Vec<String>) -> usize {
        let total_size: usize = 70000000;
        let space_need: usize = 30000000;
        let folder_tree = get_folder_tree(data);
        let free_space_need = space_need - (total_size - folder_tree.get("//").unwrap());

        *folder_tree.values().filter(|v| **v > free_space_need).min().unwrap()
    }

    fn read_example() -> Vec<String> {
        utils::read_data(Path::new("data/day7_example")).unwrap()
    }

    fn read_data_file() -> Vec<String> {
        utils::read_data(Path::new("data/day7")).unwrap()
    }

    #[test]
    fn example1() -> std::io::Result<()> {
        let data = read_example();
        assert_eq!(part1(data), 95437);
        Ok(())
    }

    #[test]
    fn exercise1() -> std::io::Result<()> {
        let data = read_data_file();
        assert_eq!(part1(data), 1086293);
        Ok(())
    }

    #[test]
    fn example2() -> std::io::Result<()> {
        let data = read_example();
        assert_eq!(part2(data), 24933642);
        Ok(())
    }

    #[test]
    fn exercise2() -> std::io::Result<()> {
        let data = read_data_file();
        assert_eq!(part2(data), 366028);
        Ok(())
    }
}
