use player::read_to_map;
use std::collections::{HashMap, HashSet};

use player::{EngineSymbol, EngineSymbolType, Map, get_engine_symbol_map};

mod player;

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();

    let map = read_to_map(&contents);

    let engine_symbols = get_engine_symbol_map(&map);

    let sum = sum_adjacent_numbers(&map, &engine_symbols);

    let gear_sum  = sum_gear_ratios(&map, &engine_symbols);

    println!("Sum of all non-adjacent numbers: {}", sum);
    println!("Gear Ratio Sum: {}", gear_sum);
}

fn sum_adjacent_numbers(map: &Map, map_lookup: &HashMap<(usize, usize), EngineSymbol>) -> u32 {
    let mut visited_ids: HashSet<usize> = HashSet::new();
    let adjacent_neighbors: Vec<(i32, i32)> = vec![
        (0,1),
        (1,0),
        (1,1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut sum = 0;
    for y in 0..map.height{
        for x in 0..map.width{
            let symbol = map_lookup.get(&(x,y)).unwrap();
            if let EngineSymbolType::Number(num) = symbol.symbol_type {
                if visited_ids.contains(&symbol.id) {
                    continue;
                }

                let mut adjacent = false;

                for (x2, y2) in adjacent_neighbors.iter() {
                    let x2 = x2 + x as i32;
                    let y2 = y2 + y as i32;
                    if x2 < 0 || y2 < 0 {
                        continue;
                    }
                    if x2 >= map.width as i32 || y2 >= map.height as i32 {
                        continue;
                    }
                    if let Some(symbol) = map_lookup.get(&(x2 as usize, y2 as usize)) {
                        if let EngineSymbolType::Symbol(c) = symbol.symbol_type {
                            println!("{} at ({}, {}) is adjacent to {}", num, x, y, c);
                            adjacent = true;
                            break;
                        }
                    }
                }

                if adjacent {
                    sum += num;
                    visited_ids.insert(symbol.id);
                }
            }
        }
    }

    sum

}


fn sum_gear_ratios(map: &Map, map_lookup: &HashMap<(usize, usize), EngineSymbol>) -> u32 {
    let mut gear_ratios_sum:u32 = 0;
    let adjacent_neighbors: Vec<(i32, i32)> = vec![
        (0,1),
        (1,0),
        (1,1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    for y in 0..map.height {
        for x in 0..map.width {
            if let Some(symbol) = map_lookup.get(&(x,y)) {
                if let EngineSymbolType::Symbol('*') = symbol.symbol_type {
                    let mut visited_ids: HashSet<usize> = HashSet::new();
                    let mut adjacent_numbers = 0;
                    let mut product = 1;

                    for (x2, y2) in adjacent_neighbors.iter() {
                        let x3 = x2 + x as i32;
                        let y3 = y2 + y as i32;

                        if x3 >= 0 && y3 >= 0 && x3 < map.width as i32 && y3 < map.height as i32 {
                            if let Some(adjacent_symbol) = map_lookup.get(&(x3 as usize, y3 as usize)) {
                                if visited_ids.contains(&adjacent_symbol.id) {
                                    continue;
                                }
                                if let EngineSymbolType::Number(num) = adjacent_symbol.symbol_type {
                                    adjacent_numbers += 1;
                                    product *= num;
                                    visited_ids.insert(adjacent_symbol.id);
                                }
                            }
                        }
                    }

                    if adjacent_numbers == 2 {
                        gear_ratios_sum += product;
                    }
                }
            }
        }
    }

    gear_ratios_sum
}
