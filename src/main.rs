use std::collections::HashMap;
use std::cmp;

fn main() {
    // lazy...
    let data = include_str!("../input.txt");
    let strings: Vec<&str> = data.split_whitespace().collect();

    println!("Words:\n {:?}", strings);

    let mut map: HashMap<(usize, char), usize> = HashMap::new();

    for s in strings.iter() {
        for (i, c) in s.chars().enumerate() {
            // println!("Inspecting char {} at index {} of word {}", c, i, s);
            if let Some(key) = map.get(&(i, c)) {
                map.insert((i, c), key + 1);
            } else {
                map.insert((i, c), 1);
            };
        }
    }
    let mut sorted: Vec<_> = map.iter().collect();
    // println!("Printing HashMap converted to Vector: {:?}", sorted);

    sorted.sort_by_key(|item|(item.0.0, cmp::Reverse(item.1)));
    // println!("Printing sorted Vector: {:?}", sorted);

    for (key, value) in sorted.iter() {
        println!("[POS, CHAR: {:?}] - [COUNT: {:?}]", key, value);
    }
}
