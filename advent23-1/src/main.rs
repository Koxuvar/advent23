use std::collections::HashMap;

fn main() {
    let words_match = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero", "1", "2",
        "3", "4", "5", "6", "7", "8", "9", "0",
    ];

    let mut contents: Vec<String> = std::fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|s| {
            if s.len() > 0 {
                s.to_string()
            } else {
                "------".to_string()
            }
        })
        .collect();

    contents.pop();
    let mut result = 0;

    for line in contents {
        if line.len() < 1 {
            break;
        }
        let mut res_array: HashMap<i32, &str> = HashMap::new();
        let mut r_res_array: HashMap<i32, &str> = HashMap::new();

        for word in &words_match[..] {
            match line.find(word) {
                Some(e) => res_array.insert(e as i32, word),
                None => None,
            };

            match line.rfind(word) {
                Some(e) => r_res_array.insert(e as i32, word),
                None => None,
            };
        }

        let mut highest_key = 0;
        let mut lowest_key = line.len() as i32;

        for k in res_array.keys() {
            if *k < lowest_key {
                lowest_key = *k;
            };
        }

        for k in r_res_array.keys() {
            if *k > highest_key {
                highest_key = *k;
            };
        }

        let first_digit: i32 = get_digit(&res_array, &lowest_key).expect("There was an error!");

        let second_digit: i32 = get_digit(&r_res_array, &highest_key).expect("There was an error!");

        result += format!("{}{}", first_digit, second_digit)
            .parse::<i32>()
            .unwrap();
    }

    println!("{}", result);
}

fn get_digit(hmap: &HashMap<i32, &str>, key: &i32) -> anyhow::Result<i32> {
    match hmap[&key] {
        "one" | "1" => Ok(1),
        "two" | "2" => Ok(2),
        "three" | "3" => Ok(3),
        "four" | "4" => Ok(4),
        "five" | "5" => Ok(5),
        "six" | "6" => Ok(6),
        "seven" | "7" => Ok(7),
        "eight" | "8" => Ok(8),
        "nine" | "9" => Ok(9),
        "zero" | "0" => Ok(0),
        _ => panic!("no number in res_array"),
    }
}
