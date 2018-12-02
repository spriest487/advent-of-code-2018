use {
    std::{ 
        env,
        collections::HashSet,
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

fn main() {
    match env::args().skip(1).next() {
        None => {
            println!("usage: aoc <day number>");
        }
        
        Some(arg) => {
            match arg.as_str() {
                "1" => day_1(),
                _ => println!("day {} not available", arg),
            }
        }
    }
}
