use std::collections::{HashMap, HashSet};

pub fn read_to_map(input: &str) -> Map {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        width = row.len();
        map.push(row);
        height += 1;
    }

    Map { map, width, height }
}


pub struct Map {
    pub map: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, PartialEq)]
pub enum EngineSymbolType {
    Number(u32),
    Empty,
    Symbol(char),
}

pub struct EngineSymbol {
    pub symbol_type: EngineSymbolType,
    pub id: usize,
}


pub fn get_engine_symbol_map(map: &Map) -> HashMap<(usize, usize), EngineSymbol> {
    let mut engine_symbols: HashMap<(usize, usize), EngineSymbol> = HashMap::new();
    let mut id = 0;

    for y in 0..map.height {
        let mut visited: HashSet<u32> = HashSet::new();
        for x in 0..map.width {
            if visited.contains(&(x as u32)) {
                continue;
            }

            let c = map.map[y][x];

            if c.is_ascii_digit() {
                let num: u32 = c.to_digit(10).unwrap();
                let mut x2 = x + 1;
                let mut digits: Vec<u32> = vec![num];

                while x2 < map.width {
                    let c2 = map.map[y][x2];
                    if c2.is_ascii_digit() {
                        visited.insert(x2 as u32);
                        let num = c2.to_digit(10).unwrap();
                        digits.push(num);
                        x2 += 1;
                    } else {
                        break;
                    }
                }

                let num = digits.iter().fold(0, |acc, x| acc * 10 + x);

                for (i, _num) in digits.iter().enumerate() {
                    engine_symbols.insert(
                        (x+i, y),
                        EngineSymbol {
                            symbol_type: EngineSymbolType::Number(num),
                            id,
                        },
                    );
                }

                engine_symbols.insert(
                    (x, y),
                    EngineSymbol {
                        symbol_type: EngineSymbolType::Number(num),
                        id,
                    },
                );
            } else if c == '.' {
                engine_symbols.insert(
                    (x, y),
                    EngineSymbol {
                        symbol_type: EngineSymbolType::Empty,
                        id,
                    },
                );
            } else {
                engine_symbols.insert(
                    (x,y),
                    EngineSymbol {
                        symbol_type: EngineSymbolType::Symbol(c),
                        id,
                    },
                );
            }
            id += 1;
        }
    }

    engine_symbols
}

