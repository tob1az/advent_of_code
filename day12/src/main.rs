mod data;

extern crate indexmap;
extern crate petgraph;
use indexmap::IndexSet;
use petgraph::graphmap::UnGraphMap;

fn is_big_cave(cave: &str) -> bool {
    cave.chars().last().unwrap().is_ascii_uppercase()
}

#[derive(PartialEq)]
enum SmallCaveEntering {
    Once,
    TwiceForOneCave,
}

fn count_paths(cave_connections: &[(&str, &str)], small_cave_entering: SmallCaveEntering) -> usize {
    let caves = UnGraphMap::<_, ()>::from_edges(cave_connections);
    let start = "start";
    let end = "end";
    let mut caves_to_explore = vec![caves.neighbors(start)];
    let mut visited_small_caves = IndexSet::from([start]);
    let mut constructed_path = vec![start];
    let mut path_count = 0;
    let mut double_visit_cave: Option<&str> = Option::None;
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
            } else if !visited_small_caves.contains(cave) {
                visited_small_caves.insert(cave);
                caves_to_explore.push(caves.neighbors(cave));
                constructed_path.push(cave);
            } else if small_cave_entering == SmallCaveEntering::TwiceForOneCave
                && cave != start
                && cave != end
                && double_visit_cave.is_none()
            {
                double_visit_cave = Some(cave);
                caves_to_explore.push(caves.neighbors(cave));
                constructed_path.push(cave);
            }
        } else {
            if let Some(cave_to_remove) = constructed_path.last() {
                if double_visit_cave == Some(cave_to_remove) {
                    double_visit_cave = None;
                } else if let Some(last_visited) = visited_small_caves.last() {
                    if last_visited == cave_to_remove {
                        visited_small_caves.pop();
                    }
                }
            }
            constructed_path.pop();
            caves_to_explore.pop();
        }
    }
    path_count
}

fn calculate_solution(cave_connections: &[(&str, &str)]) -> (usize, usize) {
    (
        count_paths(cave_connections, SmallCaveEntering::Once),
        count_paths(cave_connections, SmallCaveEntering::TwiceForOneCave),
    )
}

fn main() {
    println!("Solution {:?}", calculate_solution(&data::CAVE_CONNECTIONS));
}
