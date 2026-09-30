use compute::algorithms::minty::{minty::MintyAlgorithm, Algorithm};
use compute::dispatcher;
use compute::graph::{Edge, GraphInput};

fn edge(from: u32, to: u32, weight: i64) -> Edge {
    Edge { from, to, weight }
}

// 1. Simple graph (README example).
#[test]
fn simple_graph() {
    let g = GraphInput {
        vertices: 4,
        edges: vec![edge(1, 2, 5), edge(1, 3, 8), edge(2, 4, 7)],
        source: 1,
    };
    let (res, _) = MintyAlgorithm.execute(&g, false).unwrap();
    assert_eq!(res.distances[&1], Some(0));
    assert_eq!(res.distances[&2], Some(5));
    assert_eq!(res.distances[&3], Some(8));
    assert_eq!(res.distances[&4], Some(12));
    assert_eq!(res.paths[&4], vec![1, 2, 4]);
}

// 2. Several possible paths -> picks the shortest.
#[test]
fn multiple_paths_picks_shortest() {
    let g = GraphInput {
        vertices: 3,
        edges: vec![edge(1, 2, 10), edge(1, 3, 2), edge(3, 2, 3)],
        source: 1,
    };
    let (res, _) = MintyAlgorithm.execute(&g, false).unwrap();
    assert_eq!(res.distances[&2], Some(5));
    assert_eq!(res.paths[&2], vec![1, 3, 2]);
}

// 3. Unreachable vertices.
#[test]
fn unreachable_vertices() {
    let g = GraphInput {
        vertices: 3,
        edges: vec![edge(1, 2, 5)],
        source: 1,
    };
    let (res, _) = MintyAlgorithm.execute(&g, false).unwrap();
    assert_eq!(res.distances[&3], None);
    assert!(res.paths[&3].is_empty());
}

// 4. Single vertex.
#[test]
fn single_vertex() {
    let g = GraphInput {
        vertices: 1,
        edges: vec![],
        source: 1,
    };
    let (res, steps) = MintyAlgorithm.execute(&g, true).unwrap();
    assert_eq!(res.distances[&1], Some(0));
    assert_eq!(res.paths[&1], vec![1]);
    assert_eq!(steps.len(), 1);
}

// 5. Execution trace modes (§11): false -> no steps, true -> one step per reached vertex.
#[test]
fn trace_modes() {
    let g = GraphInput {
        vertices: 4,
        edges: vec![edge(1, 2, 5), edge(1, 3, 8), edge(2, 4, 7)],
        source: 1,
    };
    let (_, no_steps) = MintyAlgorithm.execute(&g, false).unwrap();
    assert!(no_steps.is_empty());
    let (_, steps) = MintyAlgorithm.execute(&g, true).unwrap();
    assert_eq!(steps.len(), 4);
    assert_eq!(steps[0].current, Some(1));
}

// 6. Path through several intermediate vertices.
#[test]
fn path_through_intermediates() {
    let g = GraphInput {
        vertices: 5,
        edges: vec![
            edge(1, 2, 1),
            edge(2, 3, 1),
            edge(3, 4, 1),
            edge(4, 5, 1),
            edge(1, 5, 100),
        ],
        source: 1,
    };
    let (res, _) = MintyAlgorithm.execute(&g, false).unwrap();
    assert_eq!(res.distances[&5], Some(4));
    assert_eq!(res.paths[&5], vec![1, 2, 3, 4, 5]);
}

// 7. Boundary case: zero-weight edge.
#[test]
fn zero_weight_edge() {
    let g = GraphInput {
        vertices: 2,
        edges: vec![edge(1, 2, 0)],
        source: 1,
    };
    let (res, _) = MintyAlgorithm.execute(&g, false).unwrap();
    assert_eq!(res.distances[&2], Some(0));
}

// 8. Invalid inputs.
#[test]
fn invalid_inputs() {
    // zero vertices
    let g = GraphInput { vertices: 0, edges: vec![], source: 1 };
    assert!(MintyAlgorithm.execute(&g, false).is_err());
    // bad source
    let g = GraphInput { vertices: 2, edges: vec![], source: 5 };
    assert!(MintyAlgorithm.execute(&g, false).is_err());
    // edge points outside the graph
    let g = GraphInput { vertices: 2, edges: vec![edge(1, 9, 1)], source: 1 };
    assert!(MintyAlgorithm.execute(&g, false).is_err());
    // negative weight
    let g = GraphInput { vertices: 2, edges: vec![edge(1, 2, -1)], source: 1 };
    assert!(MintyAlgorithm.execute(&g, false).is_err());
}

// 9. Dispatcher rejects unknown algorithms.
#[test]
fn dispatcher_unsupported_algorithm() {
    let g = GraphInput { vertices: 1, edges: vec![], source: 1 };
    let err = dispatcher::dispatch("unknown", &g, false).unwrap_err();
    assert!(err.to_string().contains("unknown"));
}

// 10. Dispatcher passes validation errors through.
#[test]
fn dispatcher_invalid_graph() {
    let g = GraphInput { vertices: 2, edges: vec![edge(1, 2, -3)], source: 1 };
    assert!(dispatcher::dispatch("minty", &g, false).is_err());
}
