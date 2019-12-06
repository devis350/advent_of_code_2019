#[aoc_generator(day2)]
fn generate_intcode(input: &str) -> Vec<usize> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn simple_intcode(intcode: &[usize]) -> usize {
    run_intcode(intcode, 12, 2)[0]
}

#[aoc(day2, part2)]
fn guess_input(intcode: &[usize]) -> usize {
    let output = 19_690_720;
    let mut ret = None;
    for x in 1..99 {
        for y in 1..99 {
            if run_intcode(intcode.clone(), x, y)[0] == output {
                ret = Some((x, y));
                break;
            }
            if ret.is_some() { break; }
        };
    }
    let (x,y)=ret.unwrap();
    100 * x+ y
}


fn run_intcode(intcode: &[usize], noun: usize, verb: usize) -> Vec<usize> {
    let mut intcode_vec = intcode.to_vec();

    intcode_vec[1] = noun;
    intcode_vec[2] = verb;
    let mut index = 0;

    while index <= intcode_vec.len() {
        match intcode_vec[index] {
            1 => {
                let point_1 = intcode_vec[index + 1];
                let point_2 = intcode_vec[index + 2];
                let new_pos = intcode_vec[index + 3];
                intcode_vec[new_pos] = intcode_vec[point_1] + intcode_vec[point_2];
                index += 4;
            }
            2 => {
                let point_1 = intcode_vec[index + 1];
                let point_2 = intcode_vec[index + 2];
                let new_pos = intcode_vec[index + 3];
                intcode_vec[new_pos] = intcode_vec[point_1] * intcode_vec[point_2];
                index += 4;
            }
            99 => {
                break;
            }
            _ => break
        }
    }

    intcode_vec
}
