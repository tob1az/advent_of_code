mod data;

fn calculate_solution1(instructions: &str) -> i64 {
    let mut depth = 0;
    let mut advance = 0;
    for command in instructions.lines() {
        match command.split_once(' ') {
            Some(("forward", x)) => advance += x.parse::<i64>().unwrap(),
            Some(("up", y)) => depth -= y.parse::<i64>().unwrap(),
            Some(("down", y)) => depth += y.parse::<i64>().unwrap(),
            _ => ()
        }
    }
    depth * advance
}

fn calculate_solution2(instructions: &str) -> i64 {
    let mut advance = 0i64;
    let mut aim = 0i64;
    let mut depth = 0i64;
    for command in instructions.lines() {
        match command.split_once(' ') {
            Some(("forward", x)) => {
                let forward = x.parse::<i64>().unwrap(); 
                advance += forward;
                depth += aim * forward;
            },
            Some(("up", y)) => {
                let up = y.parse::<i64>().unwrap();
                aim -= up;
            },
            Some(("down", y)) => {
                let down = y.parse::<i64>().unwrap();
                aim += down;
            },
            _ => ()
        }
    }
    depth * advance
}

fn main() {
    println!("Part 1 solution {} \n", calculate_solution1(data::INSTRUCTIONS));
    println!("Part 2 solution {} \n", calculate_solution2(data::INSTRUCTIONS));
}
