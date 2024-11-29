use crate::util::Day;

const DAY_NR: u8 = 3;
const PROBLEM_TITLE: &str = "Squares With Three Sides";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    let triangles: Vec<Triangle>;

    triangles = input.lines().map(|l| Triangle::new(l.trim())).collect();

    let mut count = 0;
    for t in triangles {
        if t.possible() {
            count += 1;
        }
    }

    count.to_string()
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

fn solve_part2(_input: &str) -> String {
    "42".to_string()
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
    fn test_part2_with_examples() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "42");
    }
}
