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

pub struct Game {
    pub id: u32,
    sets: vec<GameSet>,

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
        } else if c == ":" {
            break;
        }
    }

   game_id_str.parse::<u32>().unwrap()
}


