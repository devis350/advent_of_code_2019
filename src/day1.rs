#[aoc_generator(day1)]
fn generate_moduls(input: &str) -> Vec<usize> {
    let splits:Vec<&str> = input.split("/n").collect();
    splits.iter().map(|x| str::parse(x).unwrap()).collect()
}

#[aoc(day1, part1)]
fn fuel_from_modules(modules: &[usize]) -> usize {
    modules.iter().map(|i|{ (i/3)-2}).sum()

}

#[aoc(day1, part2)]
fn advanced_fuel_from_modules(modules: &[usize]) -> usize {
    modules.iter().map(|&int| {
        let mut sum = 0;
        let mut i = (int / 3) - 2;
        while i > 0 {
            sum += i;
            i = (i / 3) - 2;
        }
        sum
    }).sum()
}

