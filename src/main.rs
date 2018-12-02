use {
    levenshtein::levenshtein,
    std::{
        collections::HashSet,
        env,
    },
};

fn day_1() {
    let input = include_str!("day_1.txt");
    let values: Vec<_> = input.lines()
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect();

    println!("total value: {}", values.iter().sum::<i64>());

    let mut prev_vals = HashSet::new();
    let mut values_cycle = values.iter().cycle();

    let mut total = 0;
    let first_repeated = loop {
        total += values_cycle.next().unwrap();

        if !prev_vals.insert(total) {
            break total;
        }
    };

    println!("first repeated value: {}", first_repeated);
}

fn day_2() {
    let input = include_str!("day_2.txt");
    let ids: Vec<String> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let count_ids_with_n_repeated_chars = |n| {
        ids.iter()
            .filter(|line| {
                line.chars()
                    .map(|a| line.chars().filter(|b| a == *b).count())
                    .any(|count| count == n)
            })
            .count()
    };

    let ids_2_repeated = count_ids_with_n_repeated_chars(2);
    let ids_3_repeated = count_ids_with_n_repeated_chars(3);

    let checksum = ids_2_repeated * ids_3_repeated;

    println!("checksum: {}", checksum);

    let mut similar_ids = None;
    for id_a in ids.iter() {
        for id_b in ids.iter().skip(1) {
            if levenshtein(id_a, id_b) == 1 {
                similar_ids = Some((id_a, id_b));
                break;
            }
        }

        if let Some((id_a, id_b)) = similar_ids {
            println!("similar IDs:");
            println!("  {}", id_a);
            println!("  {}", id_b);

            let mut common_letters = String::new();
            for i in 0..id_a.len() {
                let c = id_a.chars().skip(i).next().unwrap();
                if c == id_b.chars().skip(i).next().unwrap() {
                    common_letters.push(c);
                }
            }

            println!("common letters: {}", common_letters);
            break;
        }
    }
}

fn main() {
    match env::args().skip(1).next() {
        None => {
            println!("usage: aoc <day number>");
        }

        Some(arg) => {
            match arg.as_str() {
                "1" => day_1(),
                "2" => day_2(),
                _ => println!("day {} not available", arg),
            }
        }
    }
}
