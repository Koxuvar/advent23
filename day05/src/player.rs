use regex::Regex;
use std::{ops::Range, vec};

pub struct ConversionChart {
    pub name: String,
    pub conversions: Vec<Conversion>,
}

pub struct Conversion {
    pub source: u64,
    pub destination: u64,
    pub range_len: u64,
}

struct RangeMapping {
    converted_range: Option<Range<u64>>,
    unconverted_ranges: Vec<Range<u64>>,
}

impl ConversionChart {
    pub fn map_number_to_number(&self, source: &u64) -> u64 {
        let destination: u64 = *source;

        for conversion in &self.conversions {
            let possible = conversion.map_number_to_number(destination);

            if let Some(new_destination) = possible {
                return new_destination;
            }
        }
        destination
    }


    pub fn map_range_to_ranges(&self, range: Range<u64>) -> Vec<Range<u64>> {
        for conversion in &self.conversions {
            let mapping = conversion.map_range_to_ranges(range.clone());
            if let Some(converted_range) = mapping.converted_range {
                let mut result_ranges = vec![converted_range];
                let recursive_results: Vec<Range<_>> = mapping
                    .unconverted_ranges
                    .iter()
                    .flat_map(|r| self.map_range_to_ranges(r.clone()))
                    .collect();
                result_ranges.extend(recursive_results);
                return result_ranges;
            }
        }

        vec![range]
    }
}

impl Conversion {
    pub fn map_number_to_number(&self, source: u64) -> Option<u64> {
        if source >= self.source && source < self.source + self.range_len {
            Some(source - self.source + self.destination)
        } else {
            None
        }
    }

    fn map_range_to_ranges(&self, range: Range<u64>) -> RangeMapping {
        // scenario 4: range is entirely outside the source range
        if (range.end < self.source) || (range.start > self.source + self.range_len) {
            return RangeMapping {
                converted_range: None,
                unconverted_ranges: vec![range],
            };
        } else if (range.start >= self.source) && (range.end <= self.source + self.range_len) {
            // scenario 1: range is entirely within the source range
            let converted_range = range.start - self.source + self.destination
                ..range.end - self.source + self.destination;
            RangeMapping {
                converted_range: Some(converted_range),
                unconverted_ranges: Vec::new(),
            }
        } else if range.start < self.source {
            if range.end <= self.source + self.range_len {
                // scenario 2.1: range starts before the source range, but ends within it
                let converted_range = self.destination..range.end - self.source + self.destination;
                let unconverted_range = range.start..self.source;
                RangeMapping {
                    converted_range: Some(converted_range),
                    unconverted_ranges: vec![unconverted_range],
                }
            } else {
                // scenario 3: range starts before the source range, and ends after source + len
                let converted_range = self.destination..self.destination + self.range_len;
                let left_unconverted_range = range.start..self.source;
                let right_unconverted_range = self.source + self.range_len..range.end;
                RangeMapping {
                    converted_range: Some(converted_range),
                    unconverted_ranges: vec![left_unconverted_range, right_unconverted_range],
                }
            }
        } else {
            // scenario 2.2: range starts after the source range, and ends after it
            let converted_range =
                self.destination..self.destination + (self.range_len - (range.start - self.source));
            let unconverted_range = self.source + self.range_len..range.end;
            RangeMapping {
                converted_range: Some(converted_range),
                unconverted_ranges: vec![unconverted_range],
            }
        }
    }
}



pub fn read_input(input: &str) -> (Vec<u64>, Vec<ConversionChart>) {
    let mut seeds: Vec<u64> = Vec::new();
    let mut charts: Vec<ConversionChart> = Vec::new();

    let re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    let blank_re = Regex::new(r"^\s*\n?$").unwrap();

    for line in input.lines() {
        if line.contains("seeds") {
            line.split(':')
                .nth(1)
                .unwrap()
                .split(' ')
                .filter(|s| !s.is_empty())
                .for_each(|s| seeds.push(s.parse::<u64>().unwrap()));
        } else if line.contains("map") {
            let stripped = line.trim().split(' ').next().unwrap();
            charts.push(ConversionChart{
                name: stripped.to_string(),
                conversions: Vec::new(),
            });
        } else if re.is_match(line) {
            let caps = re.captures(line).unwrap();
            let source = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let destination = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let range_len = caps.get(3).unwrap().as_str().parse::<u64>().unwrap();
            let chart = charts.last_mut().unwrap();
            chart.conversions.push(Conversion {
                source,
                destination,
                range_len,
            });
        } else if blank_re.is_match(line) {
            //blank
        } else {
            panic!("Unrecognized lines: {}", line);
        }
    }

    (seeds, charts)
}
