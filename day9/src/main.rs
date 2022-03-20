mod data;

fn fill_basin(origin: &(usize, usize), basins: &mut Vec<Vec<bool>>) -> usize {
    let y_max = basins.len() - 1;
    let x_max = basins[0].len() - 1;
    let mut basin_size = 0usize;
    let mut neighbors: Vec<(usize, usize)> = vec![*origin];

    basins[origin.1][origin.0] = true;
    while neighbors.len() > 0 {
        let (x, y) = neighbors.pop().unwrap();
        basin_size += 1;
        if x > 0 && !basins[y][x - 1] {
            basins[y][x - 1] = true;
            neighbors.push((x - 1, y));
        }
        if x < x_max && !basins[y][x + 1] {
            basins[y][x + 1] = true;
            neighbors.push((x + 1, y));
        }
        if y > 0 && !basins[y - 1][x] {
            basins[y - 1][x] = true;
            neighbors.push((x, y - 1));
        }
        if y < y_max && !basins[y + 1][x] {
            basins[y + 1][x] = true;
            neighbors.push((x, y + 1));
        }
    }
    basin_size
}

fn calculate_solution<T>(height_map: &[T]) -> (u32, usize)
where
    T: AsRef<[u32]>,
{
    let mut risk_level_sum = 0;
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for y in 0..height_map.len() {
        let row = height_map[y].as_ref();
        for x in 0..row.len() {
            let height = row[x];
            if (x == 0 || height < row[x - 1])
                && (x == row.len() - 1 || height < row[x + 1])
                && (y == 0 || height < height_map[y - 1].as_ref()[x])
                && (y == height_map.len() - 1 || height < height_map[y + 1].as_ref()[x])
            {
                risk_level_sum += height + 1;
                low_points.push((x, y));
            }
        }
    }

    const BORDER_HEIGHT: u32 = 9;
    let mut basins: Vec<_> = height_map
        .iter()
        .map(|row| {
            row.as_ref()
                .iter()
                .map(|height| *height == BORDER_HEIGHT)
                .collect::<Vec<_>>()
        })
        .collect();
    let mut basin_sizes: Vec<usize> = Vec::with_capacity(low_points.len());
    for origin in low_points.iter() {
        basin_sizes.push(fill_basin(&origin, &mut basins));
    }
    basin_sizes.sort_unstable();
    let top_3_basins_product: usize = basin_sizes.iter().rev().take(3).fold(1, |p, size| p * size);
    (risk_level_sum, top_3_basins_product)
}

fn main() {
    let (risk_level_sum, top_3_basins_product) = calculate_solution(&data::HEIGHT_MAP);
    println!("Solution {} {}", risk_level_sum, top_3_basins_product);
}
