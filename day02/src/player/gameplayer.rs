
#[derive(Debug, PartialEq)]
pub struct GameSet {
    green: u32,
    red: u32,
    blue: u32,
}

impl GameSet{
    pub fn is_possible(&self, red_max:u32, green_max:u32, blue_max: u32) -> bool {
        self.red <= red_max && self.green <= green_max && self.blue <= blue_max
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    sets: Vec<GameSet>,

}

impl Game{
    pub fn is_possible(&self, red_max: u32, green_max:u32, blue_max:u32) -> bool {
        self.sets
            .iter()
            .all(|set| set.is_possible(red_max, green_max, blue_max))
    }

    pub fn get_min_power_required(&self) -> u32 {
        let mut red_max: u32 = 0;
        let mut green_max: u32 = 0;
        let mut blue_max: u32 = 0;

        for set in &self.sets {
            if set.red > red_max {
                red_max = set.red;
            }
            if set.green > green_max {
                green_max = set.green;
            }
            if set.blue > blue_max {
                blue_max = set.blue;
            }
        }

        red_max * green_max * blue_max
    }

}

pub fn get_game_id(input: &str) -> u32 {
   let mut game_id_str: String = String::new();
   for c in input.chars() {
       if c.is_numeric() {
           game_id_str.push(c);
        } else if c == ':' {
            break;
        }
    }

   game_id_str.parse::<u32>().unwrap()
}

fn parse_game_set(input: &str) -> GameSet {
    let mut green: u32 = 0;
    let mut red: u32 = 0;
    let mut blue: u32 = 0;

    let cubes_split = input.split(',');

    for cube in cubes_split {
        let cube_split: Vec<&str> = cube.trim().split(' ').collect();
        let num = cube_split[0].parse::<u32>().unwrap();
        let color = cube_split[1];

        match color {
            "green" => green = num,
            "red" => red = num,
            "blue" => blue = num,
            _ => panic!("Invalid color: {}", color),
        }
    }

    GameSet { green, red, blue }

}

pub fn parse_game(input: &str) -> Game{
    let game_id = get_game_id(input);

    let mut input_split = input.split(':');

    let sets_split = input_split.nth(1).unwrap().split(';');

    let sets: Vec<GameSet> = sets_split.map(parse_game_set).collect();

    Game {id: game_id, sets }
}

pub fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(parse_game).collect()
}
