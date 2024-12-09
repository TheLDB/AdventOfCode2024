use std::collections::{HashMap, HashSet};

use num::integer::gcd;

use crate::utils::runner::Runner;

pub struct D8 {}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Mapping {
    bounds: Point,
    signals: HashMap<char, Vec<Point>>,
}

impl D8 {
    fn build_mapping(&self, test: bool) -> Mapping {
        let input = self.load_input(test);

        let mut map: HashMap<char, Vec<Point>> = HashMap::new();

        let mut y_bounds = 0;
        let mut x_bounds = 0;

        for (y, line) in input.lines().enumerate() {
            if y == 0 {
                x_bounds = line.len() - 1;
            } else {
                y_bounds += 1;
            }
            for (x, char) in line.chars().enumerate() {
                if char != '.' {
                    map.entry(char).or_insert(vec![]).push(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }

        let map = map
            .into_iter()
            .filter(|(_, v)| v.len() > 1)
            .collect::<HashMap<char, Vec<Point>>>();

        Mapping {
            bounds: Point {
                x: x_bounds as i32,
                y: y_bounds as i32,
            },
            signals: map,
        }
    }
}

impl Runner for D8 {
    fn name(&self) -> (usize, usize) {
        (2024, 8)
    }

    // so we have a map of dots and "signal frequencies"
    // for each signal frequency, we need to find its "pairs" that are within range
    // the way that it works is that if you have an "a" signal
    // you need to find other "a" signals
    // so if we have an "a" at 2,2
    // and another a at "4,4"
    // that means we went down 2, and over 2
    // so we need to place our first "a"'s antinode at 0,0 (up 2, back 2)
    // we don't need to worry about the second nodes antinode, because it'll figure that out once it scans

    // we do need to make sure that we're only scanning within range though
    // for example for our "a" at 2,2
    // theres no point in scanning for other signals past the 4y level, since the antinode would be oob
    // and if the map is only 10y and we have an "A" at 10,1
    // theres no reason to scan down
    //
    // in terms of actually building
    // i think we can do a full pass through the map, and store signals in a map of Signal: (y, x)
    // doing this, for one we can discard signals with only one point, since no matching signals will be found
    // then, for each signal in the mapping, we can loop through, grab the entry, and identify ones with valid antinodes

    fn part_one(&self) -> Option<usize> {
        let map = self.build_mapping(false);

        let mut antinodes: HashSet<Point> = HashSet::new();

        for (_, value) in &map.signals {
            for (i, signal) in value.iter().enumerate() {
                for (j, other_signal) in value.iter().enumerate() {
                    if i != j {
                        let y_distance = signal.y - other_signal.y;
                        let x_distance = signal.x - other_signal.x;

                        let antinode_y = signal.y + y_distance;
                        let antinode_x = signal.x + x_distance;

                        if (antinode_y < 0 || antinode_x < 0)
                            || (antinode_y > map.bounds.y || antinode_x > map.bounds.x)
                        {
                            continue;
                        }

                        antinodes.insert(Point {
                            x: antinode_x,
                            y: antinode_y,
                        });
                    }
                }
            }
        }
        Some(antinodes.len())
    }

    // ok so now we need to plot antinodes across the whole map
    // based on the pattern between two antennas
    // so we have to get the distance between antennas again
    // and then find the smallest possible step
    // then we can walk across the plot and place antinodes at every step needed
    fn part_two(&self) -> Option<usize> {
        let map = self.build_mapping(false);

        let mut antinodes: HashSet<Point> = HashSet::new();

        // For each frequency
        for (_, antennas) in &map.signals {
            for i in 0..antennas.len() {
                for j in (i + 1)..antennas.len() {
                    let a = antennas[i];
                    let b = antennas[j];

                    let dx = b.x - a.x;
                    let dy = b.y - a.y;

                    let divisor = gcd(dx, dy);
                    let step_x = dx / divisor;
                    let step_y = dy / divisor;

                    {
                        let mut x = a.x;
                        let mut y = a.y;
                        while x >= 0 && y >= 0 && x <= map.bounds.x && y <= map.bounds.y {
                            antinodes.insert(Point { x, y });
                            x += step_x;
                            y += step_y;
                        }
                    }

                    {
                        let mut x = a.x;
                        let mut y = a.y;
                        while x >= 0 && y >= 0 && x <= map.bounds.x && y <= map.bounds.y {
                            antinodes.insert(Point { x, y });
                            x -= step_x;
                            y -= step_y;
                        }
                    }
                }
            }
        }

        Some(antinodes.len())
    }
}
