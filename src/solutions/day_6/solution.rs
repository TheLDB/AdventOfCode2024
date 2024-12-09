use std::collections::HashSet;

use crate::utils::runner::Runner;

pub struct D6 {}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    direction: GuardDirection,
    coords: Point,
}

#[derive(Clone)]
struct Map {
    guard: Guard,
    map: Vec<Vec<char>>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct ObstructionHit {
    point: Point,
    direction: GuardDirection,
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

    fn simulate_guard(&self, map: Map, obstruction: Point) -> bool {
        let (mut guard, mut grid) = (map.guard, map.map);
        let mut obstruction_hits: HashSet<ObstructionHit> = HashSet::new();
        let max_steps = 20000;
        let mut steps = 0;

        loop {
            steps += 1;
            if steps > max_steps {
                return true;
            }

            let next_point = match guard.direction {
                GuardDirection::Up => Point {
                    x: guard.coords.x,
                    y: guard.coords.y - 1,
                },
                GuardDirection::Down => Point {
                    x: guard.coords.x,
                    y: guard.coords.y + 1,
                },
                GuardDirection::Left => Point {
                    x: guard.coords.x - 1,
                    y: guard.coords.y,
                },
                GuardDirection::Right => Point {
                    x: guard.coords.x + 1,
                    y: guard.coords.y,
                },
            };

            if next_point == obstruction {
                let hit = ObstructionHit {
                    point: obstruction,
                    direction: guard.direction,
                };

                if !obstruction_hits.insert(hit) {
                    println!("Loop detected! Guard hit obstruction at ({}, {}) twice from {:?} direction",
                        obstruction.x, obstruction.y, guard.direction);
                    return true;
                }
            }

            match guard.direction {
                GuardDirection::Up => {
                    let above = guard.coords.y - 1;
                    if above < 0 {
                        break;
                    }

                    let above_char = grid[above as usize][guard.coords.x as usize];
                    if above_char == '#' {
                        guard.direction = GuardDirection::Right;
                    } else if above_char == '.' {
                        grid[guard.coords.y as usize][guard.coords.x as usize] = '.';
                        guard.coords.y -= 1;
                    }
                }
                GuardDirection::Down => {
                    let below = guard.coords.y + 1;
                    if below >= grid.len() as i32 {
                        break;
                    }

                    let below_char = grid[below as usize][guard.coords.x as usize];
                    if below_char == '#' {
                        guard.direction = GuardDirection::Left;
                    } else if below_char == '.' {
                        grid[guard.coords.y as usize][guard.coords.x as usize] = '.';
                        guard.coords.y += 1;
                    }
                }
                GuardDirection::Left => {
                    let left = guard.coords.x - 1;
                    if left < 0 {
                        break;
                    }

                    let left_char = grid[guard.coords.y as usize][left as usize];
                    if left_char == '#' {
                        guard.direction = GuardDirection::Up;
                    } else if left_char == '.' {
                        grid[guard.coords.y as usize][guard.coords.x as usize] = '.';
                        guard.coords.x -= 1;
                    }
                }
                GuardDirection::Right => {
                    let right = guard.coords.x + 1;
                    if right >= grid[0].len() as i32 {
                        break;
                    }

                    let right_char = grid[guard.coords.y as usize][right as usize];
                    if right_char == '#' {
                        guard.direction = GuardDirection::Down;
                    } else if right_char == '.' {
                        grid[guard.coords.y as usize][guard.coords.x as usize] = '.';
                        guard.coords.x += 1;
                    }
                }
            }

            if guard.coords.y < 0
                || guard.coords.y >= grid.len() as i32
                || guard.coords.x < 0
                || guard.coords.x >= grid[0].len() as i32
            {
                return false;
            }
        }

        false
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
        let input = self.load_input(true);
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
    // this is maybe some of the worst code ever
    // got impatient and *heavily* bruteforced it
    // this should be like 100x faster
    fn part_two(&self) -> Option<usize> {
        let input = self.load_input(false);
        let original_map = self.build_map(input);
        let mut valid_positions = 0;

        for y in 0..original_map.map.len() {
            for x in 0..original_map.map[0].len() {
                if original_map.map[y][x] != '.'
                    || (x as i32 == original_map.guard.coords.x
                        && y as i32 == original_map.guard.coords.y)
                {
                    continue;
                }

                println!("Testing obstruction at position ({}, {})", x, y);

                let obstruction = Point {
                    x: x as i32,
                    y: y as i32,
                };

                let mut test_map = original_map.clone();
                test_map.map[y][x] = '#';

                if self.simulate_guard(test_map, obstruction) {
                    valid_positions += 1;
                }
            }
        }

        Some(valid_positions)
    }
}
