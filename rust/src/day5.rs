use std::str::FromStr;

struct Crate {
    crates: Vec<char>,
}

impl Crate {
    pub fn mut_crates(&mut self) -> &mut Vec<char> {
        &mut self.crates
    }
}

struct Instructions {
    movement: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instructions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (movement, from, to): (usize, usize, usize) =
            sscanf::scanf!(s, "move {} from {} to {}", usize, usize, usize).unwrap();

        Ok(Self {
            movement: movement,
            from: from,
            to: to,
        })
    }
}

impl Instructions {
    fn move_crate(&self, crates: &mut Vec<Crate>) -> Vec<char> {
        let mut crate_from = crates.get_mut(self.from - 1).unwrap().mut_crates();
        crate_from
            .drain(crate_from.len() - self.movement..)
            .rev()
            .collect()
    }
    fn move_crate_part2(&self, crates: &mut Vec<Crate>) -> Vec<char> {
        let mut crate_from = crates.get_mut(self.from - 1).unwrap().mut_crates();
        crate_from
            .drain(crate_from.len() - self.movement..)
            .collect()
    }
    fn add_crate(&self, crates: &mut Vec<Crate>, data: Vec<char>) {
        crates
            .get_mut(self.to - 1)
            .unwrap()
            .mut_crates()
            .extend(data);
    }
}

#[cfg(test)]
mod test {
    use crate::day5::Crate;
    use crate::utils;
    use std::path::Path;

    use super::Instructions;

    fn part1(data: Vec<Instructions>, crates: &mut Vec<Crate>) -> String {
        for instruction in data {
            let d = instruction.move_crate(crates);
            instruction.add_crate(crates, d);
        }
        crates
            .into_iter()
            .map(|x| *x.crates.last().unwrap())
            .collect::<String>()
    }

    fn part2(data: Vec<Instructions>, crates: &mut Vec<Crate>) -> String {
        for instruction in data {
            let d = instruction.move_crate_part2(crates);
            instruction.add_crate(crates, d);
        }
        crates
            .into_iter()
            .map(|x| *x.crates.last().unwrap())
            .collect::<String>()
    }

    fn read_example() -> Vec<Instructions> {
        utils::read_data(Path::new("data/day5_example")).unwrap()
    }

    fn read_data_file() -> Vec<Instructions> {
        utils::read_data(Path::new("data/day5")).unwrap()
    }

    fn init_crates_example() -> Vec<Crate> {
        let mut crates: Vec<Crate> = vec![
            Crate {
                crates: vec!['Z', 'N'],
            },
            Crate {
                crates: vec!['M', 'C', 'D'],
            },
            Crate { crates: vec!['P'] },
        ];

        crates
    }

    fn init_crates() -> Vec<Crate> {
        let mut crates: Vec<Crate> = vec![
            Crate {
                crates: vec!['Z', 'J', 'G'],
            },
            Crate {
                crates: vec!['Q', 'L', 'R', 'P', 'W', 'F', 'V', 'C'],
            },
            Crate {
                crates: vec!['F', 'P', 'M', 'C', 'L', 'G', 'R'],
            },
            Crate {
                crates: vec!['L', 'F', 'B', 'W', 'P', 'H', 'M'],
            },
            Crate {
                crates: vec!['G', 'C', 'F', 'S', 'V', 'Q'],
            },
            Crate {
                crates: vec!['W', 'H', 'J', 'Z', 'M', 'Q', 'T', 'L'],
            },
            Crate {
                crates: vec!['H', 'F', 'S', 'B', 'V'],
            },
            Crate {
                crates: vec!['F', 'J', 'Z', 'S'],
            },
            Crate {
                crates: vec!['M', 'C', 'D', 'P', 'F', 'H', 'B', 'T'],
            },
        ];
        crates
    }

    #[test]
    fn example1() -> std::io::Result<()> {
        let data = read_example();
        let mut crates = init_crates_example();
        println!("{}", part1(data, &mut crates));
        Ok(())
    }

    #[test]
    fn exercise1() -> std::io::Result<()> {
        let data = read_data_file();
        let mut crates = init_crates_example();
        println!("{}", part2(data, &mut crates));
        Ok(())
    }

    #[test]
    fn example2() -> std::io::Result<()> {
        let data = read_example();
        let mut crates = init_crates_example();
        println!("{}", part2(data, &mut crates));
        Ok(())
    }

    #[test]
    fn exercise2() -> std::io::Result<()> {
        let data = read_data_file();
        let mut crates = init_crates();
        println!("{}", part2(data, &mut crates));
        Ok(())
    }
}
