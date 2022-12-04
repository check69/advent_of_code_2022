use std::str::FromStr;

struct ElfAssignment {
    start: usize,
    end: usize,
}

struct Assignments {
    first_elf: ElfAssignment,
    second_elf: ElfAssignment,
}

impl FromStr for Assignments {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x1, y1, x2, y2): (usize, usize, usize, usize) =
            sscanf::scanf!(s, "{}-{},{}-{}", usize, usize, usize, usize).unwrap();

        Ok(Self {
            first_elf: ElfAssignment { start: x1, end: y1 },
            second_elf: ElfAssignment { start: x2, end: y2 },
        })
    }
}

fn is_subset(first: &ElfAssignment, second: &ElfAssignment) -> bool {
    first.start <= second.start && first.end >= second.end
}

impl Assignments {
    fn is_fully_overlapping(&self) -> bool {
        if self.first_elf.start < self.second_elf.start || self.second_elf.end < self.first_elf.end
        {
            return is_subset(&self.first_elf, &self.second_elf);
        }
        is_subset(&self.second_elf, &self.first_elf)
    }

    fn is_overlapping(&self) -> bool {
        if self.first_elf.start <= self.second_elf.start && self.first_elf.end >= self.second_elf.start{
            return true;
        }
        if self.second_elf.start <= self.first_elf.start && self.second_elf.end >= self.first_elf.start{
            return true;
        }
        if self.first_elf.start <= self.second_elf.end && self.first_elf.end >= self.second_elf.end{
            return true;
        }
        if self.second_elf.start <= self.first_elf.end && self.second_elf.end >= self.first_elf.end{
            return true;
        }

        return false;
    }
}

#[cfg(test)]
mod test {
    use crate::day4::Assignments;
    use crate::utils;
    use std::path::Path;

    fn part1(data: Vec<Assignments>) -> usize {
        data.iter()
            .map(|assignment| assignment.is_fully_overlapping() as usize)
            .sum()
    }

    fn part2(data: Vec<Assignments>) -> usize {
        data.iter()
            .map(|assignment| assignment.is_overlapping() as usize)
            .sum()
    }

    fn read_example() -> Vec<Assignments> {
        utils::read_data(Path::new("data/day4_example")).unwrap()
    }

    fn read_data_file() -> Vec<Assignments> {
        utils::read_data(Path::new("data/day4")).unwrap()
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
        let data = read_example();
        println!("{}", part2(data));
        Ok(())
    }

    #[test]
    fn exercise2() -> std::io::Result<()> {
        let data = read_data_file();
        println!("{}", part2(data));
        Ok(())
    }
}
