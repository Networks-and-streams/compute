fn main() {
    let g = compute::graph::GraphInput {
        vertices: 4,
        edges: vec![
            compute::graph::Edge {
                from: 1,
                to: 2,
                weight: 5,
            },
            compute::graph::Edge {
                from: 1,
                to: 3,
                weight: 8,
            },
            compute::graph::Edge {
                from: 2,
                to: 4,
                weight: 7,
            },
        ],
        source: 1,
    };
    let (res, steps) = compute::dispatcher::dispatch("minty", &g, true).unwrap();
    println!("{res}");
    println!("Steps: {steps}");
}
