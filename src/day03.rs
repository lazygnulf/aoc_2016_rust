use crate::util::Day;

const DAY_NR: u8 = 3;
const PROBLEM_TITLE: &str = "Squares With Three Sides";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    input
        .lines()
        .map(|l| Triangle::new(l.trim()))
        .filter(|t| t.possible())
        .count()
        .to_string()
}

#[derive(Debug)]
struct Triangle {
    a: u16,
    b: u16,
    c: u16,
}

impl Triangle {
    pub fn new(s: &str) -> Self {
        let parts: Vec<&str> = s.split_whitespace().collect();
        Self {
            a: parts[0].parse().unwrap(),
            b: parts[1].parse().unwrap(),
            c: parts[2].parse().unwrap(),
        }
    }

    pub fn possible(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.b + self.c > self.a
    }
}

fn solve_part2(input: &str) -> String {
    parse_part2(&input)
        .iter()
        .filter(|t| t.possible())
        .count()
        .to_string()
}

fn parse_part2(input: &str) -> Vec<Triangle> {
    let mut triangles = Vec::<Triangle>::new();

    let numbers: Vec<Vec<u16>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();

    if numbers.len() % 3 != 0 {
        panic!("wrong number of lines");
    }

    let mut row = 0;
    while row < numbers.len() {
        for col in 0..3 {
            let t = Triangle {
                a: numbers[row][col],
                b: numbers[row + 1][col],
                c: numbers[row + 2][col],
            };
            triangles.push(t);
        }

        row += 3;
    }

    triangles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(Triangle::new("5 10 25").possible(), false);
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "982");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "1826");
    }
}
