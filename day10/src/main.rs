mod data;

enum LineKind {
    Ok,
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn classify_line(line: &str) -> LineKind {
    let mut braces: Vec<char> = Vec::with_capacity(line.len());
    for c in line.chars() {
        match c {
            '<' | '(' | '[' | '{' => braces.push(c),
            _ => {
                let last = braces.pop().unwrap_or('x');
                if c == '>' && last != '<' {
                    return LineKind::Corrupted('>');
                } else if c == '}' && last != '{' {
                    return LineKind::Corrupted('}');
                } else if c == ']' && last != '[' {
                    return LineKind::Corrupted(']');
                } else if c == ')' && last != '(' {
                    return LineKind::Corrupted(')');
                }
            }
        }
    }
    if braces.len() > 0 {
        LineKind::Incomplete(braces)
    } else {
        LineKind::Ok
    }
}

fn evaluate_corrupted_line(line: &str) -> u32 {
    match classify_line(line) {
        LineKind::Corrupted(c) => match c {
            '>' => 25137,
            '}' => 1197,
            ']' => 57,
            ')' => 3,
            _ => 0,
        },
        _ => 0,
    }
}

fn complete_line(line: &str) -> Vec<char> {
    if let LineKind::Incomplete(open_braces) = classify_line(line) {
        open_braces
            .into_iter()
            .rev()
            .map(|c| match c {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => c,
            })
            .collect()
    } else {
        Vec::new()
    }
}

fn score_line_complement(complement: Vec<char>) -> u64 {
    complement
        .into_iter()
        .filter_map(|c| match c {
            ')' => Some(1),
            ']' => Some(2),
            '}' => Some(3),
            '>' => Some(4),
            _ => None,
        })
        .fold(0, |total, score| total * 5 + score)
}

fn calculate_solution(lines: &str) -> (u32, u64) {
    let corrupted_line_score = lines.lines().map(|l| evaluate_corrupted_line(l)).sum();
    let incomplete_line_scores = lines
        .lines()
        .map(|l| score_line_complement(complete_line(l)))
        .collect::<Vec<u64>>();
    (
        corrupted_line_score,
        incomplete_line_scores[incomplete_line_scores.len() / 2 + 1],
    )
}

fn main() {
    let (corrupted_line_score, mean_incomplete_line_score) = calculate_solution(data::LINES);
    println!(
        "Solution {} {}",
        corrupted_line_score, mean_incomplete_line_score
    );
}
