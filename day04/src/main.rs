mod player;
use player::ScratchCard;


fn main() {
    let contents = std::fs::read_to_string("input.txt");

    let results: u32 = contents.unwrap()
        .lines()
        .map(|line| ScratchCard::get_numbers(line))
        .map(|x| ScratchCard::get_winning_numbers(x).1)
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    let conts = std::fs::read_to_string("input.txt");
    let total_cards: Vec<u32> = conts.unwrap()
        .lines()
        .map(|line| ScratchCard::get_numbers(line))
        .map(|x| ScratchCard::get_winning_numbers(x).0)
        .collect::<Vec<u32>>();

    let answer = calculcate_num_scratchcards(&total_cards);

   println!("Total points: {}", results);
   println!("Total Cards: {}", answer);
}

fn calculcate_num_scratchcards(matches: &Vec<u32>) -> u32 {
    let mut copies: Vec<u32> = vec![0; matches.len()];
    let mut scratchcards: u32 = 0;

    for (i, match_count) in matches.iter().enumerate() {
        let card_copies = *copies.get(i).unwrap_or(&0);
        scratchcards += 1 + card_copies;

        if *match_count > 0 {
            for j in 0..*match_count {
                let next_card = i + j as usize + 1;
                if next_card < matches.len() {
                    let next_card_copies = copies.get_mut(next_card).unwrap();
                    *next_card_copies += card_copies + 1;
                }
            }
        }
    }


    scratchcards
}
