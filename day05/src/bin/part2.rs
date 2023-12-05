use anyhow::Context;

#[derive(Debug)]
struct Almanac {
    seed_ranges: Vec<SeedRange>,
    categories: Vec<CategoryMap>,
}

#[derive(Debug)]
struct CategoryMap {
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct SeedRange {
    start: u64,
    length: u64,
}

impl Almanac {
    fn parse(input: &str) -> anyhow::Result<Self> {
        let mut parts = input.split("\n\n");

        let seeds_numbers = parts
            .next()
            .with_context(|| "Seeds part not found")?
            .strip_prefix("seeds: ")
            .with_context(|| "Seeds prefix not found")?
            .split_ascii_whitespace()
            .map(|seed| seed.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;

        let seed_ranges = seeds_numbers
            .chunks(2)
            .map(|chunk| SeedRange {
                start: chunk[0],
                length: chunk[1],
            })
            .collect();

        let categories = parts.map(CategoryMap::parse).collect::<Result<_, _>>()?;

        Ok(Self {
            seed_ranges,
            categories,
        })
    }
}

impl CategoryMap {
    fn parse(input: &str) -> anyhow::Result<Self> {
        let ranges = input
            .lines()
            .skip(1)
            .map(Range::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self { ranges })
    }

    fn find_destination_value(&self, source: u64) -> u64 {
        self.ranges
            .iter()
            .find(|range| {
                source >= range.source_start && source < range.source_start + range.length
            })
            .map(|range| source - range.source_start + range.destination_start)
            .unwrap_or(source)
    }
}

impl Range {
    fn parse(line: &str) -> anyhow::Result<Self> {
        let mut parts = line
            .split_ascii_whitespace()
            .map(|part| part.parse::<u64>());

        Ok(Self {
            destination_start: parts
                .next()
                .with_context(|| "Range destination start not found")??,
            source_start: parts
                .next()
                .with_context(|| "Range source start not found")??,
            length: parts.next().with_context(|| "Range length not found")??,
        })
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input).unwrap();
    println!("{}", output);
}

fn process(input: &str) -> anyhow::Result<u64> {
    let almanac = Almanac::parse(input)?;

    // Don't have time to make it efficient
    let mut ids = almanac
        .seed_ranges
        .iter()
        .flat_map(|range| range.start..range.start + range.length)
        .collect::<Vec<_>>();

    for category in &almanac.categories {
        for id in &mut ids {
            *id = category.find_destination_value(*id);
        }
    }

    Ok(*ids.iter().min().with_context(|| "No seeds provided")?)
}

