use std::collections::HashSet;

use crate::util::Day;

const DAY_NR: u8 = 1;
const PROBLEM_TITLE: &str = "No Time for a Taxicab";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    follow_directions(input, false)
}

fn solve_part2(input: &str) -> String {
    follow_directions(input, true)
}

fn follow_directions(input: &str, is_part_2: bool) -> String {
    let mut visited = HashSet::new();

    let directions = vec![
        vec![0, -1], // North
        vec![1, 0],  // East
        vec![0, 1],  // South
        vec![-1, 0], // West
    ];
    let mut current_dir = 0; // index in directions vector
    let mut pos = vec![0i32, 0i32];

    if is_part_2 {
        visited.insert(pos.clone());
    }

    'cmd_loop: for cmd in input.split(", ") {
        let (turn, steps_as_string) = cmd.split_at(1);
        let steps: i32 = steps_as_string
            .parse()
            .expect("Error parsing integer (steps):");

        match turn {
            "R" => current_dir = (current_dir + 1) % 4,
            "L" => current_dir = (current_dir + 3) % 4,
            _ => panic!("Invalid direction"),
        }

        if is_part_2 {
            for _ in 0..steps {
                pos[0] += directions[current_dir][0];
                pos[1] += directions[current_dir][1];

                if visited.contains(&pos) {
                    break 'cmd_loop;
                } else {
                    visited.insert(pos.clone());
                }
            }
        } else {
            pos[0] += directions[current_dir][0] * steps;
            pos[1] += directions[current_dir][1] * steps;
        }
    }

    let distance = pos[0].abs() + pos[1].abs();

    distance.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(solve_part1("R2, L3"), "5");
        assert_eq!(solve_part1("R2, R2, R2"), "2");
        assert_eq!(solve_part1("R5, L5, R5, R3"), "12");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "288");
    }

    #[test]
    fn test_part2_with_examples() {
        assert_eq!(solve_part2("R8, R4, R4, R8"), "4");
    }

    #[test]
    fn test_part2_with_input() {
        //assert_eq!(solve_part2(&get_day().read_input()), "42");
    }
}
