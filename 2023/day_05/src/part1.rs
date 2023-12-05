use std::iter::Peekable;
use std::str::Lines;

use crate::custom_error::AocError;

fn parse_numbers_to_vec(num_string: &str) -> Vec<u32> {
    num_string
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

struct FarmMap {
    ranges: Vec<u32>,
    sources: Vec<u32>,
    dests: Vec<u32>,
}

impl FarmMap {
    pub fn get(&self, input: u32) -> u32 {
        for index in 0..self.ranges.len() {
            if input >= self.sources[index] && self.sources[index] + self.ranges[index] > input {
                return self.dests[index] + input - self.sources[index];
            }
        }
        input
    }
}

struct Farm {
    seeds: Vec<u32>,
    seed_to_soil: FarmMap,
    soil_to_fertilizer: FarmMap,
    fertilizer_to_water: FarmMap,
    water_to_light: FarmMap,
    light_to_temperature: FarmMap,
    temperature_to_humidity: FarmMap,
    humidity_to_location: FarmMap,
}

impl Farm {
    fn skip_whitespace(lines: &mut Peekable<Lines<'_>>) {
        while lines.peek().unwrap().trim().is_empty() {
            lines.next().unwrap();
        }
    }

    fn parse_map(lines: &mut Peekable<Lines<'_>>) -> FarmMap {
        let _ = lines.next().unwrap();
        let mut ranges = Vec::new();
        let mut sources = Vec::new();
        let mut dests = Vec::new();
        while lines.peek().is_some() && !lines.peek().unwrap().trim().is_empty() {
            let numbers = parse_numbers_to_vec(lines.next().unwrap());
            sources.push(numbers[1]);
            dests.push(numbers[0]);
            ranges.push(numbers[2]);
        }
        FarmMap {
            ranges,
            sources,
            dests,
        }
    }

    fn get_locations_for_seed(&self) -> Vec<u32> {
        let mut res = vec![0; self.seeds.len()];
        for (index, seed) in self.seeds.iter().enumerate() {
            let soil = self.seed_to_soil.get(*seed);
            let fertilizer = self.soil_to_fertilizer.get(soil);
            let water = self.fertilizer_to_water.get(fertilizer);
            let light = self.water_to_light.get(water);
            let temperature = self.light_to_temperature.get(light);
            let humidity = self.temperature_to_humidity.get(temperature);
            let location = self.humidity_to_location.get(humidity);
            res[index] = location
        }
        res
    }

    pub fn parse(input: &str) -> Self {
        let mut lines_iter = input.lines().peekable();
        let (_, seeds) = lines_iter.next().unwrap().split_once(':').unwrap();
        Self::skip_whitespace(&mut lines_iter);
        let seed_to_soil = Self::parse_map(&mut lines_iter);
        Self::skip_whitespace(&mut lines_iter);
        let soil_to_fertilizer = Self::parse_map(&mut lines_iter);
        Self::skip_whitespace(&mut lines_iter);
        let fertilizer_to_water = Self::parse_map(&mut lines_iter);
        Self::skip_whitespace(&mut lines_iter);
        let water_to_light = Self::parse_map(&mut lines_iter);
        Self::skip_whitespace(&mut lines_iter);
        let light_to_temperature = Self::parse_map(&mut lines_iter);
        Self::skip_whitespace(&mut lines_iter);
        let temperature_to_humidity = Self::parse_map(&mut lines_iter);
        Self::skip_whitespace(&mut lines_iter);
        let humidity_to_location = Self::parse_map(&mut lines_iter);
        Self {
            seeds: parse_numbers_to_vec(seeds),
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let farm = Farm::parse(input);

    Ok(format!(
        "{}",
        farm.get_locations_for_seed().iter().min().unwrap()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
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
        assert_eq!("35", process(input)?);
        Ok(())
    }
}
