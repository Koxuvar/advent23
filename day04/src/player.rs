pub struct ScratchCard {
   pub winning_numbers: Vec<u32>,
   pub player_numbers: Vec<u32>
}

impl ScratchCard {
    pub fn get_numbers(nums: &str) -> ScratchCard {
        let base_vec: Vec<&str> = nums.split(':')
            .nth(1)
            .unwrap()
            .split('|')
            .collect();
   
        let winners = base_vec[0]
            .trim()
            .split(' ')
            .filter(|&x| !x.is_empty())
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mine = base_vec[1]
            .trim()
            .split(' ')
            .filter(|&x| !x.is_empty())
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        return ScratchCard {winning_numbers: winners, player_numbers: mine}
    }

    pub fn get_winning_numbers(card: ScratchCard) -> u32 {
        let mut points = 0;

        for nums in card.player_numbers {
            if card.winning_numbers.contains(&nums){
                points += 1;
            }
        }

        const BASE: u32 = 2;
        match points {
            0 => 0,
            1 => 1,
            _ => BASE.pow(points -1),
        }
    }

}
