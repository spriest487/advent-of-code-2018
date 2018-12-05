fn react(a: char, b: char) -> bool {
    a.to_ascii_uppercase() == b.to_ascii_uppercase()
        && a.is_ascii_uppercase() != b.is_ascii_uppercase()
}

fn react_all(polymer: &str) -> String {
    let mut chars: Vec<_> = polymer.chars().collect();

    let mut pos = 0;
    loop {
        match chars.get(pos + 1).cloned() {
            Some(next) => {
                if react(chars[pos], next) {
                    chars.remove(pos);
                    chars.remove(pos);

                    if pos > 0 {
                        pos -= 1;
                    }
                } else {
                    pos += 1;
                }
            }
            None => break chars.into_iter().collect(),
        }
    }
}

fn remove_units(polymer: &str, unit: char) -> String {
    let mut units: Vec<_> = polymer.chars().collect();
    let mut i = 0;
    while i < units.len() {
        if units[i].to_ascii_lowercase() == unit {
            units.remove(i);
        } else {
            i += 1;
        }
    }

    units.into_iter().collect()
}

fn main() {
    let base_polymer = include_str!("day_5.txt");
    println!("base polymer length after reaction: {}", react_all(base_polymer).len());

    let simplified_polymers: Vec<_> = ('a' as u8..'z' as u8)
        .map(|unit| unit as char)
        .map(|unit| (unit, remove_units(base_polymer, unit)))
        .collect();

    let mut reacted_polymers = Vec::new();
    for (unit, polymer) in &simplified_polymers {
        let reacted_polymer = react_all(polymer);
        println!("length of polymer without unit {}: {}", unit, reacted_polymer.len());

        reacted_polymers.push((unit, reacted_polymer));
    }

    let (removed_unit, shortest_polymer) = reacted_polymers.iter()
        .min_by_key(|(_unit, polymer)| polymer.len())
        .unwrap();

    println!("best unit to remove: {}, length: {}", removed_unit, shortest_polymer.len());
}