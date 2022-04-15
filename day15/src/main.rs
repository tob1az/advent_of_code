extern crate ndarray;

#[macro_use(lazy_static)]
extern crate lazy_static;

mod data;

use data::RiskMap;
use petgraph::algo::astar;
use petgraph::prelude::*;

fn build_graph(risk_map: &RiskMap) -> UnGraph<usize, ()> {
    let mut graph = Graph::new_undirected();

    let mut upper_row = Vec::new();
    for row in risk_map.rows() {
        let mut previous: Option<NodeIndex> = None;
        let mut current_row = Vec::new();
        for (i, risk) in row.indexed_iter() {
            let point = graph.add_node(risk.clone());
            if let Some(previous_point) = previous {
                graph.add_edge(previous_point, point, ());
            }
            if upper_row.len() > 0 {
                let upper_point = upper_row[i];
                graph.add_edge(upper_point, point, ());
            }
            current_row.push(point.clone());
            previous = Some(point);
        }
        upper_row = current_row;
    }
    graph
}

fn find_safest_path(risk_map: &RiskMap) -> usize {
    let graph = build_graph(risk_map);
    let start = graph.node_indices().next().unwrap();
    let end = graph.node_indices().last().unwrap();
    let safest_path = astar(
        &graph,
        start,
        |e| e == end,
        |edge| *graph.node_weight(edge.target()).unwrap(),
        |_| 0,
    )
    .unwrap();

    safest_path.0
}

fn expand_map(risk_map: &RiskMap) -> RiskMap {
    const NUM_TILES: usize = 5;
    let max_risk = 9;
    let shape = risk_map.shape();
    let rows = shape[0];
    let columns = shape[1];
    let new_shape = (rows * NUM_TILES, columns * NUM_TILES);
    RiskMap::from_shape_fn(new_shape, |(i, j)| {
        let tile_i = i / rows;
        let tile_j = j / columns;
        let risk = risk_map[(i % rows, j % columns)] + tile_i + tile_j;
        if risk > max_risk {
            risk - max_risk
        } else {
            risk
        }
    })
}

fn calculate_solution(risk_map: &RiskMap) -> (usize, usize) {
    (
        find_safest_path(risk_map),
        find_safest_path(&expand_map(risk_map)),
    )
}

fn main() {
    println!("Solution {:?}", calculate_solution(&data::RISK_MAP));
}
