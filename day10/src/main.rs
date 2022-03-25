mod data;

fn evaluate_corrupted_line(line: &str) -> u32 {
    let mut braces: Vec<char> = Vec::with_capacity(line.len());
    for c in line.chars() {
        match c {
            '<' | '(' | '[' | '{' => braces.push(c),
            _ => {
                let last = braces.pop().unwrap_or('x');
                if c == '>' && last != '<' {
                    return 25137;
                } else if c == '}' && last != '{' {
                    return 1197;
                } else if c == ']' && last != '[' {
                    return 57;
                } else if c == ')' && last != '(' {
                    return 3;
                }
            }
        }
    }
    // valid or incomplete
    0
}

fn calculate_solution(lines: &str) -> u32 {
    lines.lines().map(|l| evaluate_corrupted_line(l)).sum()
}

fn main() {
    println!("Solution {}", calculate_solution(data::LINES));
}
