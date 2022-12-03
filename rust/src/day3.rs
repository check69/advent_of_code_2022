use std::{collections::HashSet, str::FromStr};

struct Rucksack {
    first_compartment: HashSet<char>,
    second_compartment: HashSet<char>,
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_at(s.len() / 2);
        Ok(Self {
            first_compartment: first.to_string().chars().collect(),
            second_compartment: second.to_string().chars().collect(),
        })
    }
}

fn get_value(character: &char) -> usize {
    let ascii_value = *character as usize;
    if character.is_lowercase() {
        return ascii_value - 'a' as usize + 1;
    }

    return ascii_value - 'A' as usize + 27;
}

impl Rucksack {
    fn check_repeated_elements(&self) -> usize {
        self.first_compartment
            .intersection(&self.second_compartment)
            .map(get_value)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::day3::get_value;
    use crate::day3::Rucksack;
    use crate::utils;
    use std::collections::HashSet;
    use std::hash::Hash;
    use std::path::Path;

    fn part1(data: Vec<Rucksack>) -> usize {
        data.iter()
            .map(|rucksack| rucksack.check_repeated_elements())
            .sum()
    }

    fn get_badge(rucksacks: &[String]) -> usize {
        get_value(
            rucksacks
                .iter()
                .map(|string| string.chars().collect::<HashSet<char>>())
                .into_iter()
                .reduce(|ret, otra_cosa| {
                    ret.intersection(&otra_cosa)
                        .copied()
                        .collect::<HashSet<char>>()
                })
                .unwrap()
                .iter()
                .next()
                .unwrap(),
        )
    }

    fn part2(data: Vec<String>) -> usize {
        data.chunks(3).map(get_badge).sum()
    }

    fn read_example() -> Vec<Rucksack> {
        utils::read_data(Path::new("data/day3_example")).unwrap()
    }

    fn read_data_file() -> Vec<Rucksack> {
        utils::read_data(Path::new("data/day3")).unwrap()
    }

    fn read_example_part2() -> Vec<String> {
        utils::read_data(Path::new("data/day3_example")).unwrap()
    }

    fn read_data_file_part2() -> Vec<String> {
        utils::read_data(Path::new("data/day3")).unwrap()
    }

    #[test]
    fn example1() -> std::io::Result<()> {
        let data = read_example();
        println!("{}", part1(data));
        Ok(())
    }

    #[test]
    fn exercise1() -> std::io::Result<()> {
        let data = read_data_file();
        println!("{}", part1(data));
        Ok(())
    }

    #[test]
    fn example2() -> std::io::Result<()> {
        let data = read_example_part2();
        println!("{}", part2(data));
        Ok(())
    }

    #[test]
    fn exercise2() -> std::io::Result<()> {
        let data = read_data_file_part2();
        println!("{}", part2(data));
        Ok(())
    }
}
