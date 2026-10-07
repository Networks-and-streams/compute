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
    assert_eq!(res.paths[&1], vec![vec![1]]);
    assert_eq!(res.paths[&2], vec![vec![1, 2]]);
    assert_eq!(res.paths[&3], vec![vec![1, 3]]);
    assert_eq!(res.paths[&4], vec![vec![1, 2, 4]]);
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
    assert_eq!(res.paths[&2], vec![vec![1, 3, 2]]);
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
    assert_eq!(res.paths[&1], vec![vec![1]]);
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
    assert_eq!(res.paths[&5], vec![vec![1, 2, 3, 4, 5]]);
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
    assert_eq!(res.paths[&2], vec![vec![1, 2]]);
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
    let err = dispatcher::dispatch("unknown", &g, false, None).unwrap_err();
    assert!(err.to_string().contains("unknown"));
}

// 10. Dispatcher passes validation errors through.
#[test]
fn dispatcher_invalid_graph() {
    let g = GraphInput { vertices: 2, edges: vec![edge(1, 2, -3)], source: 1 };
    assert!(dispatcher::dispatch("minty", &g, false, None).is_err());
}

// 11. Multiple equal shortest paths (diamond topology).
#[test]
fn multiple_shortest_paths_diamond() {
    let g = GraphInput {
        vertices: 4,
        edges: vec![
            edge(1, 2, 5),
            edge(1, 3, 5),
            edge(2, 4, 7),
            edge(3, 4, 7),
        ],
        source: 1,
    };
    let (res, _) = MintyAlgorithm.execute(&g, false).unwrap();
    assert_eq!(res.distances[&4], Some(12));
    assert_eq!(
        res.paths[&4],
        vec![vec![1, 2, 4], vec![1, 3, 4]]
    );
}

// 12. Three alternative shortest paths to the same destination.
#[test]
fn multiple_shortest_paths_three_alternatives() {
    let g = GraphInput {
        vertices: 5,
        edges: vec![
            edge(1, 2, 2),
            edge(1, 3, 2),
            edge(1, 4, 2),
            edge(2, 5, 3),
            edge(3, 5, 3),
            edge(4, 5, 3),
        ],
        source: 1,
    };
    let (res, _) = MintyAlgorithm.execute(&g, false).unwrap();
    assert_eq!(res.distances[&5], Some(5));
    assert_eq!(
        res.paths[&5],
        vec![vec![1, 2, 5], vec![1, 3, 5], vec![1, 4, 5]]
    );
}

// 13. Multiple shortest paths of different hop counts, ignoring longer paths.
#[test]
fn multiple_shortest_paths_different_lengths_and_intermediates() {
    let g = GraphInput {
        vertices: 5,
        edges: vec![
            edge(1, 4, 4),        // direct path: length 4
            edge(1, 2, 2),
            edge(2, 4, 2),        // via 2: length 4
            edge(1, 3, 2),
            edge(3, 4, 2),        // via 3: length 4
            edge(1, 5, 3),
            edge(5, 4, 3),        // via 5: length 6 (longer, must NOT be included)
        ],
        source: 1,
    };
    let (res, _) = MintyAlgorithm.execute(&g, false).unwrap();
    assert_eq!(res.distances[&4], Some(4));
    assert_eq!(
        res.paths[&4],
        vec![vec![1, 2, 4], vec![1, 3, 4], vec![1, 4]]
    );
}

// 14. Combinatorial multi-stage shortest paths (2 x 2 = 4 paths).
#[test]
fn multiple_shortest_paths_multi_stage_combinatorial() {
    let g = GraphInput {
        vertices: 7,
        edges: vec![
            edge(1, 2, 1),
            edge(1, 3, 1),
            edge(2, 4, 1),
            edge(3, 4, 1),
            edge(4, 5, 1),
            edge(4, 6, 1),
            edge(5, 7, 1),
            edge(6, 7, 1),
        ],
        source: 1,
    };
    let (res, _) = MintyAlgorithm.execute(&g, false).unwrap();
    assert_eq!(res.distances[&7], Some(4));
    assert_eq!(
        res.paths[&7],
        vec![
            vec![1, 2, 4, 5, 7],
            vec![1, 2, 4, 6, 7],
            vec![1, 3, 4, 5, 7],
            vec![1, 3, 4, 6, 7],
        ]
    );
}

// 15. Multiple shortest paths with zero-weight edge.
#[test]
fn multiple_shortest_paths_with_zero_weight_edge() {
    let g = GraphInput {
        vertices: 3,
        edges: vec![
            edge(1, 2, 2),
            edge(1, 3, 2),
            edge(2, 3, 0),
        ],
        source: 1,
    };
    let (res, _) = MintyAlgorithm.execute(&g, false).unwrap();
    assert_eq!(res.distances[&3], Some(2));
    assert_eq!(
        res.paths[&3],
        vec![vec![1, 2, 3], vec![1, 3]]
    );
}

// 16. Graph with zero-weight cycle terminates cleanly and produces simple shortest paths.
#[test]
fn zero_weight_cycle_terminates_and_finds_simple_paths() {
    let g = GraphInput {
        vertices: 3,
        edges: vec![
            edge(1, 2, 1),
            edge(1, 3, 1),
            edge(2, 3, 0),
            edge(3, 2, 0),
        ],
        source: 1,
    };
    let (res, _) = MintyAlgorithm.execute(&g, false).unwrap();
    assert_eq!(res.distances[&2], Some(1));
    assert_eq!(res.distances[&3], Some(1));
    assert_eq!(res.paths[&2], vec![vec![1, 2], vec![1, 3, 2]]);
    assert_eq!(res.paths[&3], vec![vec![1, 2, 3], vec![1, 3]]);
}

// 17. Parallel edges with identical weight do not cause duplicate paths.
#[test]
fn parallel_edges_identical_weight() {
    let g = GraphInput {
        vertices: 2,
        edges: vec![
            edge(1, 2, 3),
            edge(1, 2, 3),
        ],
        source: 1,
    };
    let (res, _) = MintyAlgorithm.execute(&g, false).unwrap();
    assert_eq!(res.distances[&2], Some(3));
    assert_eq!(res.paths[&2], vec![vec![1, 2]]);
}

// 18. Target mode returns exactly the routes the all-vertices mode finds for
// that vertex, and only that vertex.
#[test]
fn target_mode_matches_all_mode_for_every_vertex() {
    let graphs = vec![
        // diamond ties + unreachable vertex 5
        GraphInput {
            vertices: 5,
            edges: vec![edge(1, 2, 5), edge(1, 3, 5), edge(2, 4, 7), edge(3, 4, 7)],
            source: 1,
        },
        // zero-weight cycle
        GraphInput {
            vertices: 3,
            edges: vec![edge(1, 2, 1), edge(1, 3, 1), edge(2, 3, 0), edge(3, 2, 0)],
            source: 1,
        },
        // parallel edges, multi-stage ties, source not 1
        GraphInput {
            vertices: 6,
            edges: vec![
                edge(2, 1, 1), edge(2, 1, 1), edge(2, 3, 1), edge(1, 4, 1),
                edge(3, 4, 1), edge(4, 5, 2), edge(4, 6, 1), edge(6, 5, 1), edge(5, 2, 0),
            ],
            source: 2,
        },
    ];

    for g in &graphs {
        let (all, _) = MintyAlgorithm.execute(g, false).unwrap();
        for t in 1..=g.vertices {
            let (one, _) = MintyAlgorithm.execute_for(g, false, Some(t)).unwrap();
            assert_eq!(one.paths.len(), 1, "only the target is returned");
            assert_eq!(one.paths[&t], all.paths[&t], "routes to {t}");
            assert_eq!(one.distances, all.distances);
        }
    }
}

// 19. Target outside 1..vertices is rejected.
#[test]
fn target_out_of_range_is_invalid() {
    let g = GraphInput { vertices: 2, edges: vec![edge(1, 2, 1)], source: 1 };
    assert!(MintyAlgorithm.execute_for(&g, false, Some(0)).is_err());
    assert!(MintyAlgorithm.execute_for(&g, false, Some(3)).is_err());
    assert!(dispatcher::dispatch("minty", &g, false, Some(3)).is_err());
}

/// A chain of `layers` diamonds: 2^layers tied shortest routes to the end.
fn diamond_chain(layers: u32) -> GraphInput {
    let mut edges = Vec::new();
    let mut v = 1;
    for _ in 0..layers {
        edges.extend([edge(v, v + 1, 1), edge(v, v + 2, 1), edge(v + 1, v + 3, 1), edge(v + 2, v + 3, 1)]);
        v += 3;
    }
    GraphInput { vertices: v, edges, source: 1 }
}

// 20. Exponentially many tied routes are capped; every reachable vertex keeps a route.
#[test]
fn route_explosion_is_capped() {
    use compute::algorithms::minty::minty::MAX_ROUTES;
    let g = diamond_chain(16); // 65 536 routes to the last vertex

    let (all, _) = MintyAlgorithm.execute(&g, false).unwrap();
    assert!(all.truncated);
    for v in 1..=g.vertices {
        assert!(!all.paths[&v].is_empty(), "vertex {v} keeps a route");
    }

    let (one, _) = MintyAlgorithm.execute_for(&g, false, Some(g.vertices)).unwrap();
    assert!(one.truncated);
    assert_eq!(one.paths[&g.vertices].len(), MAX_ROUTES);

    let (early, _) = MintyAlgorithm.execute_for(&g, false, Some(7)).unwrap();
    assert!(!early.truncated);
    assert_eq!(early.paths[&7].len(), 4);
}
