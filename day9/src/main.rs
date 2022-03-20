mod data;

fn calculate_solution<T>(height_map: &[T]) -> u32
where
    T: AsRef<[u32]>,
{
    let mut risk_level_sum = 0;
    for y in 0..height_map.len() {
        let row = height_map[y].as_ref();
        for x in 0..row.len() {
            let height = row[x];
            if (x == 0 || height < row[x - 1])
                && (x == row.len() - 1 || height < row[x + 1])
                && (y == 0 || height < height_map[y - 1].as_ref()[x])
                && (y == height_map.len() - 1 || height < height_map[y + 1].as_ref()[x])
            {
                print!("({})", row[x]);
                risk_level_sum += height + 1;
            } else {
                print!("{} ", row[x]);
            }
        }
        println!("\n");
    }
    risk_level_sum
}

fn main() {
    println!("Solution {}", calculate_solution(&data::HEIGHT_MAP));
}
