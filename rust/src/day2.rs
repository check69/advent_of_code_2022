use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Values {
    Error = 0,
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Values {
    fn win(self) -> Self {
        use Values::*;
        match self {
            Rock => Paper,
            Paper => Scissor,
            Scissor => Rock,
            _ => Error,
        }
    }

    fn lose(self) -> Self {
        use Values::*;
        match self {
            Rock => Scissor,
            Paper => Rock,
            Scissor => Paper,
            _ => Error,
        }
    }

    fn to_usize(self) -> usize {
        self as usize
    }

    fn is_winner(self, oponent: Self) -> bool {
        use Values::*;
        match (self, oponent) {
            (Rock, Scissor) => true,
            (Paper, Rock) => true,
            (Scissor, Paper) => true,
            _ => false,
        }
    }
}

fn char_to_value(s: &str) -> Values {
    use Values::*;
    match s {
        "A" | "X" => Rock,
        "B" | "Y" => Paper,
        "C" | "Z" => Scissor,
        _ => Error,
    }
}

struct Strategy {
    oponent: Values,
    me: Values,
}

impl FromStr for Strategy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut characters = s.split(" ");
        Ok(Self {
            oponent: char_to_value(characters.next().unwrap()),
            me: char_to_value(characters.next().unwrap()),
        })
    }
}

impl Strategy {
    fn score1(&self) -> usize {
        let mut ret: usize = 0;
        if self.me.is_winner(self.oponent) {
            ret += 6;
        } else if self.me == self.oponent {
            ret += 3;
        }
        ret + self.me.to_usize()
    }

    fn score2(&self) -> usize {
        use Values::*;
        match self.me {
            Rock => self.oponent.lose().to_usize(),
            Paper => self.oponent.to_usize() + 3,
            Scissor => self.oponent.win().to_usize() + 6,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day2::Strategy;
    use crate::day2::Values;
    use crate::utils;
    use std::path::Path;

    fn part1(strategy: Vec<Strategy>) -> usize {
        strategy.iter().map(|strat| strat.score1()).sum()
    }

    fn part2(strategy: Vec<Strategy>) -> usize {
        strategy.iter().map(|strat| strat.score2()).sum()
    }

    #[test]
    fn example1() -> std::io::Result<()> {
        let strategy: Vec<Strategy> = utils::read_data(Path::new("data/day2_example")).unwrap();
        println!("{}", part1(strategy));
        Ok(())
    }

    #[test]
    fn exercise1() -> std::io::Result<()> {
        let strategy: Vec<Strategy> = utils::read_data(Path::new("data/day2")).unwrap();
        println!("{}", part1(strategy));
        Ok(())
    }

    #[test]
    fn example2() -> std::io::Result<()> {
        let strategy: Vec<Strategy> = utils::read_data(Path::new("data/day2_example")).unwrap();
        println!("{}", part2(strategy));
        Ok(())
    }

    #[test]
    fn exercise2() -> std::io::Result<()> {
        let strategy: Vec<Strategy> = utils::read_data(Path::new("data/day2")).unwrap();
        println!("{}", part2(strategy));
        Ok(())
    }
}
