use super::Puzzle;

pub struct Puzzle03 {
    input: String,
}

impl Puzzle03 {
    pub fn solve_part1(&self, grid: &Grid) -> i64 {
        let slope = (1, 3);

        (0..)
            .map(|n| (n * slope.0, n * slope.1))
            .map(|(x, y)| grid.item_at(x, y))
            .take_while(|item| item.is_some())
            .filter(|item| match item.unwrap() {
                GridItem::Tree => true,
                _ => false,
            })
            .count() as i64
    }

    pub fn solve_part2(&self, grid: &Grid) -> i64 {
        let slopes: Vec<(usize, usize)> = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

        slopes
            .into_iter()
            .map(|slope| {
                (0..)
                    .map(|n| (n * slope.0, n * slope.1))
                    .map(|(x, y)| grid.item_at(x, y))
                    .take_while(|item| item.is_some())
                    .filter(|item| match item.unwrap() {
                        GridItem::Tree => true,
                        _ => false,
                    })
                    .count() as i64
            })
            .product()
    }
}

impl Puzzle<(i64, i64)> for Puzzle03 {
    fn build(input: String) -> Self {
        Puzzle03 { input }
    }

    fn solve(&self) -> (i64, i64) {
        let grid = Grid::parse(&self.input);

        let part1 = self.solve_part1(&grid);
        let part2 = self.solve_part2(&grid);

        (part1, part2)
    }
}

#[derive(Copy, Clone, Debug)]
enum GridItem {
    OpenSquare,
    Tree,
}

pub struct Grid {
    width: usize,
    grid: Vec<GridItem>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let width = input.find('\n').unwrap();
        let grid = input
            .chars()
            .filter_map(|c| match c {
                '.' => Some(GridItem::OpenSquare),
                '#' => Some(GridItem::Tree),
                '\n' => None,
                ' ' => None,
                c => panic!(format!("Invalid character found: '{}'", c)),
            })
            .collect();

        Grid { width, grid }
    }

    fn item_at(&self, x: usize, y: usize) -> Option<GridItem> {
        match self.grid.get(x * self.width + y % self.width) {
            Some(item) => Some(*item),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1_example() {
        let input = "\
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#";

        let puzzle = Puzzle03::build(input.to_string());
        let solution = puzzle.solve();

        assert_eq!(solution.0, 7);
    }

    #[test]
    fn part1() {
        let input = read_to_string("inputs/puzzle03.input").unwrap();
        let puzzle = Puzzle03::build(input.to_string());
        let solution = puzzle.solve();

        assert_eq!(solution.0, 228);
    }

    #[test]
    fn part2_example() {
        let input = "\
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#";

        let puzzle = Puzzle03::build(input.to_string());
        let solution = puzzle.solve();

        assert_eq!(solution.1, 336);
    }

    #[test]
    fn part2() {
        let input = read_to_string("inputs/puzzle03.input").unwrap();
        let puzzle = Puzzle03::build(input.to_string());
        let solution = puzzle.solve();

        assert_eq!(solution.1, 6818112000);
    }
}
