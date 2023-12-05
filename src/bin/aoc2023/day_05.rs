use crate::Runner;
use rayon::prelude::*;
use std::{ops::Range, usize};

#[derive(Debug)]
struct Mapping {
    source: Range<usize>,
    destination: Range<usize>,
}

impl Mapping {
    pub fn get_corresponds(&self, item: usize) -> usize {
        return item - self.source.start + self.destination.start;
    }
}

fn find_corresponding_item(item: usize, mappings: &[Mapping]) -> usize {
    mappings
        .par_iter()
        .find_first(|mapping| mapping.source.contains(&item))
        .map_or(item, |mapping| mapping.get_corresponds(item))
}

#[derive(Default)]
pub struct Day05 {
    input: String,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn solution01(&self) -> u32 {
        let maps = self
            .input
            .split("\n\n")
            .collect::<Vec<_>>()
            .iter()
            .map(|line| line.split(":").collect::<Vec<_>>()[1])
            .collect::<Vec<_>>();

        let seeds = maps[0]
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();

        let mappings = maps[1..]
            .iter()
            .map(|line| {
                line.split("\n")
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|line| {
                        let values: Vec<usize> = line
                            .split_whitespace()
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        self.mapper(values[0], values[1], values[2])
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let final_lowest = seeds
            .par_iter()
            .map(|&seed| {
                let mut item = seed;

                for mapping in mappings.iter() {
                    item = find_corresponding_item(item, mapping);
                }

                item
            })
            .reduce(|| std::usize::MAX, |a, b| if a < b { a } else { b });
        final_lowest as u32
    }

    pub fn solution02(&self) -> u32 {
        let maps = self
            .input
            .split("\n\n")
            .collect::<Vec<_>>()
            .iter()
            .map(|line| line.split(":").collect::<Vec<_>>()[1])
            .collect::<Vec<_>>();

        let seeds = maps[0]
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();

        let mappings = maps[1..]
            .iter()
            .map(|line| {
                line.split("\n")
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|line| {
                        let values: Vec<usize> = line
                            .split_whitespace()
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        self.mapper(values[0], values[1], values[2])
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        println!("++++++++++++++ {:?}", mappings.len());
        let parsed_seeds = self.parse_seed_ranges(&seeds);
        println!("++++++++++++++ {:?}", parsed_seeds.len());

        let final_lowest = parsed_seeds
            .par_iter()
            .map(|&seed| {
                let mut item = seed;

                for mapping in mappings.iter() {
                    item = find_corresponding_item(item, mapping);
                }

                item
            })
            .reduce(|| std::usize::MAX, |a, b| if a < b { a } else { b });
        final_lowest as u32
    }

    fn mapper(&self, destination: usize, start: usize, length: usize) -> Mapping {
        let map = Mapping {
            source: start..(start + length),
            destination: destination..(destination + length),
        };

        map
    }

    fn parse_seed_ranges(&self, seed_ranges: &Vec<usize>) -> Vec<usize> {
        let seeds = seed_ranges
            .par_chunks_exact(2)
            .flat_map(|chunk| {
                let start = chunk[0];
                let end = start + chunk[1];
                (start..end).into_par_iter()
            })
            .collect();

        seeds
    }
}

impl Runner for Day05 {
    fn name(&self) -> (usize, usize) {
        (2023, 5)
    }

    fn parse(&mut self) {
        let file = include_str!("../../../input/2023-05-01.txt");

        self.input = file.to_string()
    }

    fn part1(&mut self) -> Vec<String> {
        let output = self.solution01();
        crate::output(output)
    }

    fn part2(&mut self) -> Vec<String> {
        let output = self.solution02();
        crate::output(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day() -> Day05 {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4";

        let mut day = Day05::new();
        day.input = input.to_string();

        day
    }

    #[test]
    fn test_part01() {
        let output = day().solution01();

        assert_eq!(35, output);
    }

    #[test]
    fn test_part02() {
        let output = day().solution02();

        assert_eq!(46, output);
    }
}
