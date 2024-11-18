mod player;

fn main() {

    let contents = std::fs::read_to_string("input.txt").unwrap();

    let (seeds, charts) = player::read_input(&contents);
   
    let answer = seeds
        .iter()
        .map(|seed| map_seed_to_location(seed, &charts))
        .min()
        .unwrap();

    println!("Lowest location is: {}", answer);
}

pub fn map_seed_to_location(seed: &u64, charts: &Vec<player::ConversionChart>) -> u64 {
    let mut location = *seed;
    for chart in charts {
        location = chart.map_number_to_number(&location);
    }

    location
}
