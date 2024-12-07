use std::collections::HashSet;

use crate::utils::runner::Runner;

pub struct D6 {}

#[derive(Debug)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Guard {
    direction: GuardDirection,
    coords: Point,
}

struct Map {
    guard: Guard,
    map: Vec<Vec<char>>,
}

impl D6 {
    fn build_map(&self, input: String) -> Map {
        let lines = input
            .split_terminator("\n")
            .map(|d| d.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let (guard_y, guard_x, direction) = lines
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, &ch)| match ch {
                    '^' => Some((y, x, GuardDirection::Up)),
                    'v' => Some((y, x, GuardDirection::Down)),
                    '>' => Some((y, x, GuardDirection::Right)),
                    '<' => Some((y, x, GuardDirection::Left)),
                    _ => None,
                })
            })
            .unwrap();

        Map {
            guard: Guard {
                direction,
                coords: Point {
                    x: guard_x as i32,
                    y: guard_y as i32,
                },
            },
            map: lines,
        }
    }
}

impl Runner for D6 {
    fn name(&self) -> (usize, usize) {
        (2024, 6)
    }

    // So we have a map consisting of:
    // "." to mark an empty spot
    // "#" to mark an obstruction
    // "^"/"v"/">"/"<" to mark the guard (up/down/right/left)
    // We need to map out their steps until they leave the mapped area and count how many "steps" they take
    // We can do this by creating like a while guard_y >= 0 / guard_y <= map_y and guard_x >= 0 / guard_x <= map_x
    // and while this is true, we move the guard up/down/left/right while making sure that new point isnt an obstacle
    //
    fn part_one(&self) -> Option<usize> {
        let input = self.load_input(false);
        let map = self.build_map(input);
        let (mut guard, mut map) = (map.guard, map.map);
        let mut visited_spots: HashSet<Point> = HashSet::new();

        loop {
            visited_spots.insert(guard.coords);

            match guard.direction {
                GuardDirection::Up => {
                    // if going up would take us out of bounds, break the loop
                    let above = guard.coords.y - 1;
                    if above < 0 {
                        break;
                    }

                    let above_char = map[above as usize][guard.coords.x as usize];
                    if above_char == '#' {
                        // If obstruction, pivot
                        guard.direction = GuardDirection::Right;
                    } else if above_char == '.' {
                        // Move up and insert new pos
                        map[guard.coords.y as usize][guard.coords.x as usize] = '.';
                        guard.coords.y -= 1;
                    }
                }
                GuardDirection::Down => {
                    let below = guard.coords.y + 1;
                    // break on greater than or even bcz len starts at 1, so if our cord is 3, that means its in the fourth position, which if the len is 3, means it doesnt exist and we break
                    if below >= map.len() as i32 {
                        break;
                    }

                    let below_char = map[below as usize][guard.coords.x as usize];
                    if below_char == '#' {
                        guard.direction = GuardDirection::Left;
                    } else if below_char == '.' {
                        map[guard.coords.y as usize][guard.coords.x as usize] = '.';
                        guard.coords.y += 1;
                    }
                }
                GuardDirection::Left => {
                    let left = guard.coords.x - 1;
                    if left < 0 {
                        break;
                    }

                    let left_char = map[guard.coords.y as usize][left as usize];
                    if left_char == '#' {
                        guard.direction = GuardDirection::Up;
                    } else if left_char == '.' {
                        map[guard.coords.y as usize][guard.coords.x as usize] = '.';
                        guard.coords.x -= 1;
                    }
                }
                GuardDirection::Right => {
                    let right = guard.coords.x + 1;

                    if right >= map[0].len() as i32 {
                        break;
                    }

                    let right_char = map[guard.coords.y as usize][right as usize];
                    if right_char == '#' {
                        guard.direction = GuardDirection::Down;
                    } else if right_char == '.' {
                        map[guard.coords.y as usize][guard.coords.x as usize] = '.';
                        guard.coords.x += 1;
                    }
                }
            }
        }

        Some(visited_spots.len() as usize)
    }

    // Ok so I need to figure out how i can trap the guard in an infinite loop
    // by placing a single obstruction (not where they are standing)
    //
    // i want to avoid heavily bruteforcing this by placing an item in every area and testing it
    // so the current idea is that we're trying to like "close the circuit"
    // so what if we run through the movement again, and track each turns position
    // when we've turned 3 times, we know that we can likely place a obstruction to make it loop
    // its just the math of where that should be
    fn part_two(&self) -> Option<usize> {
        let input = self.load_input(true);
        let map = self.build_map(input.clone());
        let (mut guard, mut map) = (map.guard, map.map);
        let mut visited_spots: HashSet<Point> = HashSet::new();
        let mut turns: Vec<Point> = Vec::new();
        loop {
            if turns.len() == 3 {
                break;
            }
            visited_spots.insert(guard.coords);

            match guard.direction {
                GuardDirection::Up => {
                    // if going up would take us out of bounds, break the loop
                    let above = guard.coords.y - 1;
                    if above < 0 {
                        break;
                    }

                    let above_char = map[above as usize][guard.coords.x as usize];
                    if above_char == '#' {
                        // If obstruction, pivot
                        turns.push(Point {
                            y: above,
                            x: guard.coords.x,
                        });
                        guard.direction = GuardDirection::Right;
                    } else if above_char == '.' {
                        // Move up and insert new pos
                        map[guard.coords.y as usize][guard.coords.x as usize] = '.';
                        guard.coords.y -= 1;
                    }
                }
                GuardDirection::Down => {
                    let below = guard.coords.y + 1;
                    // break on greater than or even bcz len starts at 1, so if our cord is 3, that means its in the fourth position, which if the len is 3, means it doesnt exist and we break
                    if below >= map.len() as i32 {
                        break;
                    }

                    let below_char = map[below as usize][guard.coords.x as usize];
                    if below_char == '#' {
                        turns.push(Point {
                            y: below,
                            x: guard.coords.x,
                        });
                        guard.direction = GuardDirection::Left;
                    } else if below_char == '.' {
                        map[guard.coords.y as usize][guard.coords.x as usize] = '.';
                        guard.coords.y += 1;
                    }
                }
                GuardDirection::Left => {
                    let left = guard.coords.x - 1;
                    if left < 0 {
                        break;
                    }

                    let left_char = map[guard.coords.y as usize][left as usize];
                    if left_char == '#' {
                        turns.push(Point {
                            y: guard.coords.y,
                            x: left,
                        });
                        guard.direction = GuardDirection::Up;
                    } else if left_char == '.' {
                        map[guard.coords.y as usize][guard.coords.x as usize] = '.';
                        guard.coords.x -= 1;
                    }
                }
                GuardDirection::Right => {
                    let right = guard.coords.x + 1;

                    if right >= map[0].len() as i32 {
                        break;
                    }

                    let right_char = map[guard.coords.y as usize][right as usize];
                    if right_char == '#' {
                        turns.push(Point {
                            y: guard.coords.y,
                            x: right,
                        });
                        guard.direction = GuardDirection::Down;
                    } else if right_char == '.' {
                        map[guard.coords.y as usize][guard.coords.x as usize] = '.';
                        guard.coords.x += 1;
                    }
                }
            }
        }

        // TODO: finish this (i need to sleep)
        println!("Turns: {:#?}", turns);

        println!("{}", input.clone());
        None
    }
}
