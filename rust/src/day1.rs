#[cfg(test)]
mod test {
    use crate::utils::{self, read_multiple_data};
    use std::path::Path;

    fn part1(elfs: Vec<Vec<usize>>) -> usize {
        elfs.iter()
            .map(|calories| calories.iter().sum())
            .max()
            .unwrap_or(0)
    }

    fn part2(elfs: Vec<Vec<usize>>) -> usize {
        let mut maximums: Vec<usize> = elfs.iter().map(|calories| calories.iter().sum()).collect();
        maximums.sort();
        maximums[maximums.len() - 3..].iter().sum()
    }

    #[test]
    fn example1() -> std::io::Result<()> {
        let mut elfs: Vec<Vec<usize>> =
            utils::read_multiple_data(Path::new("data/day1_example")).unwrap();
        println!("{}", part1(elfs));

        Ok(())
    }

    #[test]
    fn exercise1() -> std::io::Result<()> {
        let mut elfs: Vec<Vec<usize>> = utils::read_multiple_data(Path::new("data/day1")).unwrap();
        println!("{}", part1(elfs));

        Ok(())
    }

    #[test]
    fn example2() -> std::io::Result<()> {
        let mut elfs: Vec<Vec<usize>> =
            utils::read_multiple_data(Path::new("data/day1_example")).unwrap();
        println!("{}", part2(elfs));
        Ok(())
    }

    #[test]
    fn exercise2() -> std::io::Result<()> {
        let mut elfs: Vec<Vec<usize>> = utils::read_multiple_data(Path::new("data/day1")).unwrap();
        println!("{}", part2(elfs));

        Ok(())
    }
}
