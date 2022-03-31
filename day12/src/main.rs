mod data;

extern crate indexmap;
extern crate petgraph;
use indexmap::IndexSet;
use petgraph::graphmap::UnGraphMap;

fn is_big_cave(cave: &str) -> bool {
    cave.chars().last().unwrap().is_ascii_uppercase()
}

fn calculate_solution(cave_connections: &[(&str, &str)]) -> usize {
    let caves = UnGraphMap::<_, ()>::from_edges(cave_connections);
    let start = "start";
    let end = "end";
    let mut caves_to_explore = vec![caves.neighbors(start)];
    let mut visited = IndexSet::from([start]);
    let mut constructed_path = vec![start];
    let mut path_count = 0;
    while let Some(adjacent_caves) = caves_to_explore.last_mut() {
        if let Some(cave) = adjacent_caves.next() {
            if cave == end {
                path_count += 1;
                let new_path = constructed_path
                    .iter()
                    .cloned()
                    .chain(Some(end))
                    .collect::<Vec<&str>>();
                println!("path = {:?}", new_path);
            } else if is_big_cave(cave) {
                caves_to_explore.push(caves.neighbors(cave));
                constructed_path.push(cave);
            } else if !visited.contains(cave) {
                visited.insert(cave);
                caves_to_explore.push(caves.neighbors(cave));
                constructed_path.push(cave);
            }
        } else {
            if let Some(cave) = constructed_path.last() {
                if visited.contains(cave) {
                    visited.remove(cave);
                }
            }
            constructed_path.pop();
            caves_to_explore.pop();
        }
    }
    path_count
}

fn main() {
    println!("Solution {:?}", calculate_solution(&data::CAVE_CONNECTIONS));
}
