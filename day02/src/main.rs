use player::gameplayer::parse_games;

pub mod player;

fn main() {
    
    let input= std::fs::read_to_string("input.txt");
    let games = parse_games(&input.unwrap());

    const RED_MAX:u32 = 12;
    const GREEN_MAX:u32 = 13;
    const BLUE_MAX:u32 = 14;

    let possible_sum: u32 = games
        .iter()
        .filter(|g| g.is_possible(RED_MAX, GREEN_MAX, BLUE_MAX))
        .map(|g| g.id)
        .sum();

    let power_total: u32 = games
        .iter()
        .map(|g| g.get_min_power_required())
        .sum();

    println!("Possible Sum: {}", possible_sum);
    println!("power: {}", power_total);
}

