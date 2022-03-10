mod data;

fn compute_bit_histogram(numbers: &Vec<Vec<u32>>) -> Vec<u32> {
    numbers
        .clone()
        .into_iter()
        .reduce(|total_bits, bits| {
            total_bits
                .iter()
                .zip(bits.iter())
                .map(|(x, y)| x + y)
                .collect::<Vec<u32>>()
        })
        .unwrap()
}

fn find_most_common_bits(histogram: &Vec<u32>, numbers: &Vec<Vec<u32>>) -> Vec<u32> {
    histogram
        .iter()
        .map(|&bit_count| {
            if is_majority_of(&numbers, bit_count) {
                1
            } else {
                0
            }
        })
        .collect::<Vec<u32>>()
}

fn as_number(bits: &Vec<u32>) -> u32 {
    bits.iter().fold(0, |result, bit| (result << 1) | bit)
}

fn is_majority_of(numbers: &Vec<Vec<u32>>, bit_count: u32) -> bool {
    let number_count = numbers.len() as u32;
    let is_even = number_count & 1 == 0;
    if is_even {
        bit_count >= number_count / 2
    } else {
        bit_count >= number_count / 2 + 1
    }
}

fn count_bits(numbers: &Vec<Vec<u32>>, bit: usize) -> u32 {
    numbers
        .iter()
        .map(|bits| bits[bit])
        .fold(0, |total_bits, bits| total_bits + bits)
}

fn calculate_solution(report: &str) -> (u32, u32) {
    let numbers = report
        .lines()
        .map(|number| {
            number
                .chars()
                .map(|c| if c == '1' { 1 } else { 0 })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let histogram = compute_bit_histogram(&numbers);
    let most_common_bits = find_most_common_bits(&histogram, &numbers);
    let bit_width = histogram.len();
    let gamma_rate = as_number(&most_common_bits);
    let epsilon_rate = (!gamma_rate) & ((1u32 << bit_width) - 1);
    println!("g {} e {} h {:?}", gamma_rate, epsilon_rate, histogram);

    let mut oxygen: Option<Vec<u32>> = None;
    let mut filtered_o2 = numbers.clone();
    for bit in 0..bit_width {
        let number_count = filtered_o2.len() as u32;
        let most_common_bit = if is_majority_of(&filtered_o2, count_bits(&filtered_o2, bit)) {
            1
        } else {
            0
        };
        println!(
            "bit: {} number_count: {} most_common_bit o2: {}",
            bit, number_count, most_common_bit
        );
        filtered_o2 = filtered_o2
            .into_iter()
            .filter(|n| n[bit] == most_common_bit)
            .collect::<Vec<Vec<u32>>>();
        println!("filtered o2: {:?}", filtered_o2);
        if filtered_o2.len() == 1 {
            oxygen = Some(filtered_o2[0].clone());
            break;
        }
    }
    let oxygen_rate = as_number(&oxygen.unwrap());

    let mut co2: Option<Vec<u32>> = None;
    let mut filtered_co2 = numbers;
    for bit in 0..bit_width {
        let number_count = filtered_co2.len() as u32;
        let least_common_bit = if is_majority_of(&filtered_co2, count_bits(&filtered_co2, bit)) {
            0
        } else {
            1
        };

        println!(
            "bit: {} number_count: {} least_common_bit co2: {}",
            bit, number_count, least_common_bit
        );
        filtered_co2 = filtered_co2
            .into_iter()
            .filter(|n| n[bit] == least_common_bit)
            .collect::<Vec<Vec<u32>>>();
        println!("filtered co2: {:?}", filtered_co2);
        if filtered_co2.len() == 1 {
            co2 = Some(filtered_co2[0].clone());
            break;
        }
    }
    let co2_rate = as_number(&co2.unwrap());
    println!("co2_rate {} oxygen_rate {}", co2_rate, oxygen_rate);

    (gamma_rate * epsilon_rate, oxygen_rate * co2_rate)
}

fn main() {
    let (power_consumption, life_support_rating) = calculate_solution(data::REPORT);
    println!("Solution {}, {}", power_consumption, life_support_rating);
}
