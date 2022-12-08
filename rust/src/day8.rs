#[cfg(test)]
mod test {
    use crate::utils;
    use std::path::Path;

    fn check_row(grid: &Vec<Vec<usize>>, row: usize, column: usize, right: bool) -> bool {
        let mut range = 0..row;
        if right {
            range = row + 1..grid.len();
        }
        for i in range {
            if grid[row][column] <= grid[i][column] {
                return false;
            }
        }
        true
    }

    fn check_column(grid: &Vec<Vec<usize>>, row: usize, column: usize, down: bool) -> bool {
        let mut range = 0..column;
        if down {
            range = column + 1..grid.last().unwrap().len();
        }
        for i in range {
            if grid[row][column] <= grid[row][i] {
                return false;
            }
        }
        true
    }

    fn is_visible(grid: &Vec<Vec<usize>>, position: (usize, usize)) -> bool {
        if check_row(grid, position.0, position.1, false)
            || check_row(grid, position.0, position.1, true)
            || check_column(grid, position.0, position.1, false)
            || check_column(grid, position.0, position.1, true)
        {
            return true;
        }

        false
    }

    fn internal_trees_visibles(grid: &Vec<Vec<usize>>) -> usize {
        let mut count: usize = 0;

        for i in 1..grid.len() - 1 {
            for j in 1..grid.last().unwrap().len() - 1 {
                if is_visible(grid, (i, j)) {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_edge_count(grid: &Vec<Vec<usize>>) -> usize {
        grid.len() * 2 + (grid.last().unwrap().len() - 2) * 2
    }

    fn part1(data: Vec<Vec<char>>) -> usize {
        let grid: Vec<Vec<usize>> = data
            .iter()
            .map(|x| x.iter().map(|y| *y as usize - 0x30).collect())
            .collect();

        get_edge_count(&grid) + internal_trees_visibles(&grid)
    }

    fn right_score(grid: &Vec<Vec<usize>>, row: usize, column: usize) -> usize {
        let mut count: usize = 0;
        for i in (0..row).rev() {
            if grid[row][column] <= grid[i][column] {
                return count + 1;
            }
            count += 1;
        }

        count
    }
    fn left_score(grid: &Vec<Vec<usize>>, row: usize, column: usize) -> usize {
        let mut count: usize = 0;
        for i in (row + 1)..grid.len() {
            if grid[row][column] <= grid[i][column] {
                return count + 1;
            }
            count += 1;
        }

        count
    }

    fn up_score(grid: &Vec<Vec<usize>>, row: usize, column: usize) -> usize {
        let mut count: usize = 0;
        for i in (0..column).rev() {
            if grid[row][column] <= grid[row][i] {
                return count + 1;
            }
            count += 1;
        }

        count
    }
    fn down_score(grid: &Vec<Vec<usize>>, row: usize, column: usize) -> usize {
        let mut count: usize = 0;
        for i in (column + 1)..grid.last().unwrap().len() {
            if grid[row][column] <= grid[row][i] {
                return count + 1;
            }
            count += 1;
        }

        count
    }

    fn tree_scenic_score(grid: &Vec<Vec<usize>>, position: (usize, usize)) -> usize {
        let (right, left, up, down) = (
            right_score(grid, position.0, position.1),
            left_score(grid, position.0, position.1),
            up_score(grid, position.0, position.1),
            down_score(grid, position.0, position.1),
        );
        right * left * up * down
    }

    fn get_scenic_score(grid: Vec<Vec<usize>>) -> usize {
        let mut scenic_score: Vec<usize> = vec![];
        for i in 1..grid.len() - 1 {
            for j in 1..grid.last().unwrap().len() - 1 {
                scenic_score.push(tree_scenic_score(&grid, (i, j)));
            }
        }
        *scenic_score.iter().max().unwrap()
    }

    fn part2(data: Vec<Vec<char>>) -> usize {
        let grid: Vec<Vec<usize>> = data
            .iter()
            .map(|x| x.iter().map(|y| *y as usize - 0x30).collect())
            .collect();

        get_scenic_score(grid)
    }

    fn read_example() -> Vec<Vec<char>> {
        let mut data: Vec<String> = utils::read_data(Path::new("data/day8_example")).unwrap();
        data.iter()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect()
    }

    fn read_data_file() -> Vec<Vec<char>> {
        let mut data: Vec<String> = utils::read_data(Path::new("data/day8")).unwrap();
        data.iter()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect()
    }

    #[test]
    fn example1() -> std::io::Result<()> {
        let data = read_example();
        assert_eq!(part1(data), 21);
        Ok(())
    }

    #[test]
    fn exercise1() -> std::io::Result<()> {
        let data = read_data_file();
        let result = part1(data);
        assert_eq!(result, 1779);
        Ok(())
    }

    #[test]
    fn example2() -> std::io::Result<()> {
        let data = read_example();
        let result = part2(data);
        assert_eq!(result, 8);
        Ok(())
    }

    #[test]
    fn exercise2() -> std::io::Result<()> {
        let data = read_data_file();
        let result = part2(data);
        assert_eq!(result, 172224);
        Ok(())
    }
}
