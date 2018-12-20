struct RecipeBoard {
    recipes: Vec<usize>,
}

impl RecipeBoard {
    fn new(val1: usize, val2: usize) -> Self {
        Self {
            recipes: vec![val1, val2]
        }
    }

    fn mix_recipes(&mut self, i1: usize, i2: usize) -> usize {
        let recipe1 = self.recipes[i1];
        let recipe2 = self.recipes[i2];

        let new_recipes: Vec<_> = (recipe1 + recipe2).to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let new_count = new_recipes.len();

        self.recipes.extend(new_recipes);
        new_count
    }
}

fn main() {
    let mut board = RecipeBoard::new(3, 7);

    let mut elf1_cursor = 0;
    let mut elf2_cursor = 1;

    const TARGET: usize = 110201;

    while board.recipes.len() < TARGET + 10 {
        board.mix_recipes(elf1_cursor, elf2_cursor);

        elf1_cursor = (elf1_cursor + 1 + board.recipes[elf1_cursor]) % board.recipes.len();
        elf2_cursor = (elf2_cursor + 1 + board.recipes[elf2_cursor]) % board.recipes.len();
    }

    print!("score of 10 recipes following {} attempts: ", TARGET);
    for score_digit in &board.recipes[TARGET..TARGET + 10] {
        print!("{}", score_digit);
    }
    println!();

    board = RecipeBoard::new(3, 7);
    elf1_cursor = 0;
    elf2_cursor = 1;

    const TARGET_DIGITS: [usize; 6] = [1, 1, 0, 2, 0, 1];

    let recipes_before_target = 'try_recipes: loop {
        let new = board.mix_recipes(elf1_cursor, elf2_cursor);
        let skip = (board.recipes.len() - new).checked_sub(TARGET_DIGITS.len()).unwrap_or(0);

        for (i, window) in board.recipes[skip..].windows(6).enumerate() {
            if window == &TARGET_DIGITS {
                break 'try_recipes i + skip;
            }
        }

        elf1_cursor = (elf1_cursor + 1 + board.recipes[elf1_cursor]) % board.recipes.len();
        elf2_cursor = (elf2_cursor + 1 + board.recipes[elf2_cursor]) % board.recipes.len();
    };

    println!("recipes tried before reaching target scores: {}", recipes_before_target);
}
