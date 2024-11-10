use crate::input;

fn main() {
    let mut contents: Vec<String> = Vec::new();

    for line in std::fs::read_to_string("test.txt").unwrap().lines() {
        contents.push(line.to_string());
    }

    for line in contents {
        let index = &line.find(":").unwrap();

        let game = line.to_string().split_off(*index + 1);

        let col: Vec<String> = game.split(";").map(str::to_string).collect();

        let mut hmap: HashMap<String, u32> = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);

        for entr in &col {
            let shod: Vec<String> = entr.trim().split(",").map(str::to_string).collect();

            for msd in &shod {
                let test: Vec<String> = msd.trim().split_whitespace().map(str::to_string).collect();

                let hmap = update_hmap(hmap, test);

            }
        }
    }

        for (i, j) in hmap {
            println!("{}: {}", i, j);
        }

        println!("--------------");
}

