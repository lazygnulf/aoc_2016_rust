use crate::util::Day;

const DAY_NR: u8 = 2;
const PROBLEM_TITLE: &str = "Bathroom Security";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    let keypad: Vec<Vec<char>> = vec![
        ".....".chars().collect(),
        ".123.".chars().collect(),
        ".456.".chars().collect(),
        ".789.".chars().collect(),
        ".....".chars().collect(),
    ];

    bathroom_code(input, &keypad, (2, 3))
}

fn solve_part2(input: &str) -> String {
    let keypad: Vec<Vec<char>> = vec![
        ".......".chars().collect(),
        "...1...".chars().collect(),
        "..234..".chars().collect(),
        ".56789.".chars().collect(),
        "..ABC..".chars().collect(),
        "...D...".chars().collect(),
        ".......".chars().collect(),
    ];

    bathroom_code(input, &keypad, (3, 1))
}

fn bathroom_code(input: &str, keypad: &Vec<Vec<char>>, start: (usize, usize)) -> String {
    let mut bathroom_code = String::new();

    let mut pos_x = start.0;
    let mut pos_y = start.1;

    for line in input.lines().map(|l| l.trim()) {
        for cmd in line.chars() {
            let mut new_pos_x = pos_x;
            let mut new_pos_y = pos_y;
            match cmd {
                'U' => new_pos_x -= 1,
                'D' => new_pos_x += 1,
                'L' => new_pos_y -= 1,
                'R' => new_pos_y += 1,
                _ => panic!("unknown command"),
            }
            if keypad[new_pos_x][new_pos_y] != '.' {
                pos_x = new_pos_x;
                pos_y = new_pos_y;
            }
        }
        bathroom_code.push(keypad[pos_x][pos_y]);
    }

    bathroom_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        let input = r#"ULL
            RRDDD
            LURDL
            UUUUD"#;
        assert_eq!(solve_part1(&input), "1985");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "84452");
    }

    #[test]
    fn test_part2_with_examples() {
        let input = r#"ULL
            RRDDD
            LURDL
            UUUUD"#;
        assert_eq!(solve_part2(&input), "5DB3");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "D65C3");
    }
}
