#[cfg(test)]
mod test {
    use crate::utils;
    use std::{collections::HashSet, path::Path};

    fn get_packet_position(data: Vec<char>, packet_size: usize) -> usize {
        data.windows(packet_size)
            .enumerate()
            .find_map(|(i, window)| {
                if window.iter().collect::<HashSet<_>>().len() == packet_size {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap()
            + packet_size
    }

    fn part1(data: &Vec<char>) -> usize {
        get_packet_position(data.clone(), 4)
    }

    fn part2(data: &Vec<char>) -> usize {
        get_packet_position(data.clone(), 14)
    }

    fn read_example() -> Vec<Vec<char>> {
        let data: Vec<String> = utils::read_data(Path::new("data/day6_example")).unwrap();
        data.iter()
            .map(|x| x.to_string().chars().collect())
            .collect()
    }

    fn read_other_example(example: &str) -> Vec<char> {
        example.to_string().chars().collect()
    }

    fn read_data_file() -> Vec<char> {
        let data: Vec<String> = utils::read_data(Path::new("data/day6")).unwrap();
        data.last().unwrap().to_string().chars().collect()
    }

    #[test]
    fn example1() -> std::io::Result<()> {
        let data = read_example();
        assert_eq!(
            data.iter().map(part1).collect::<Vec<usize>>(),
            vec![7, 5, 6, 10, 11]
        );

        Ok(())
    }

    #[test]
    fn exercise1() -> std::io::Result<()> {
        let data = read_data_file();
        assert_eq!(part1(&data), 1987);
        Ok(())
    }

    #[test]
    fn example2() -> std::io::Result<()> {
        let data = read_example();
        assert_eq!(
            data.iter().map(part2).collect::<Vec<usize>>(),
            vec![19, 23, 23, 29, 26]
        );
        Ok(())
    }

    #[test]
    fn exercise2() -> std::io::Result<()> {
        let data = read_data_file();
        println!("{}", part2(&data));
        assert_eq!(part2(&data), 3059);
        Ok(())
    }
}
