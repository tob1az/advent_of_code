mod bingo;

use std::collections::HashSet;
use std::iter::FromIterator;

fn compute_score(
    winning_numbers: &bingo::BoardNumbers,
    winning_line_number: usize,
    last_number: u32,
    called_numbers: &HashSet<u32>,
) -> u32 {
    let unmarked_sum: u32 = winning_numbers
        .iter()
        .cloned()
        .enumerate()
        .filter(|(i, _)| *i != winning_line_number)
        .map(|(_, line)| 
            line.difference(&called_numbers).sum::<u32>()
        )
        .sum();
    unmarked_sum * last_number
}

fn calculate_solution(numbers: &[u32], boards: &[bingo::Board]) -> (u32, u32) {
    if numbers.len() < bingo::SIZE {
        return (0, 0);
    }

    let mut winner_board_numbers: HashSet<usize> = HashSet::new();
    let mut first_win: Option<u32> = None;
    let mut last_win: Option<u32> = None;

    for i in bingo::SIZE..numbers.len() {
        let sequence = &numbers[0..i];
        let number_set = HashSet::from_iter(sequence.iter().cloned());
        for (board_number, board) in boards.iter().enumerate() {
            if winner_board_numbers.contains(&board_number) {
                continue;
            }
            for (row_number, row) in board.rows.iter().cloned().enumerate() {
                if row.is_subset(&number_set) {
                    let win = compute_score(&board.rows, row_number, numbers[i - 1], &number_set);
                    if first_win.is_none() {
                        first_win = Some(win);
                    } else {
                        last_win = Some(win);
                    }
                    winner_board_numbers.insert(board_number);
                }
            }
            for (column_number, column) in board.columns.iter().cloned().enumerate() {
                if column.is_subset(&number_set) {
                    let win =
                        compute_score(&board.columns, column_number, numbers[i - 1], &number_set);
                    if first_win.is_none() {
                        first_win = Some(win);
                    } else {
                        last_win = Some(win);
                    }
                    winner_board_numbers.insert(board_number);
                }
            }
        }
    }
    (first_win.unwrap_or_default(), last_win.unwrap_or_default())
}

fn main() {
    let boards = bingo::BOARD_INPUT
        .iter()
        .map(|i| bingo::parse_board(i))
        .collect::<Vec<bingo::Board>>();
    let (first_win, last_win) = calculate_solution(&bingo::NUMBERS, &boards);
    println!("Solution {} {}", first_win, last_win);
}
