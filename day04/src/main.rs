mod player;

fn main() {
    let contents = std::fs::read_to_string("input.txt");
    
    let mut results: u32 = 0;
    for line in contents.unwrap().lines(){
        let card = player::ScratchCard::get_numbers(line);
        results += player::ScratchCard::get_winning_numbers(card);
    }


    println!("{}", results);
}
