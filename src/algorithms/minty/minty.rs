use super::{Algorithm, MintyResult, MintyStep};
use crate::graph::GraphInput;

pub struct MintyAlgorithm;

/// Upper bound on routes returned by one run. Tied shortest routes can grow
/// exponentially (each "diamond" doubles them), so enumeration stops here and
/// the result is flagged `truncated`.
pub const MAX_ROUTES: usize = 1000;

impl Algorithm for MintyAlgorithm {
    fn name(&self) -> &'static str {
        "minty"
    }

    fn execute(
        &self,
        g: &GraphInput,
        include_steps: bool,
    ) -> Result<(MintyResult, Vec<MintyStep>), crate::error::ComputeError> {
        self.execute_for(g, include_steps, None)
    }
}

impl MintyAlgorithm {
    /// Like [`Algorithm::execute`], with an optional `target` vertex.
    ///
    /// - `None`: all shortest routes to every vertex.
    /// - `Some(t)`: all shortest routes to `t` only. `paths` then holds just `t`,
    ///   and only vertices lying on a shortest route to `t` are explored
    ///   (backwards from `t`), instead of enumerating routes to every vertex.
    ///
    /// Distances are always computed for every vertex (one Dijkstra pass).
    pub fn execute_for(
        &self,
        g: &GraphInput,
        include_steps: bool,
        target: Option<u32>,
    ) -> Result<(MintyResult, Vec<MintyStep>), crate::error::ComputeError> {
        g.validate()?;
        if let Some(t) = target {
            if t < 1 || t > g.vertices {
                return Err(crate::error::ComputeError::InvalidGraph("bad target".into()));
            }
        }
        let n = g.vertices;
        let adj = g.adj_list();
        let mut dist: Vec<Option<u64>> = vec![None; n as usize + 1];
        let mut visited = vec![false; n as usize + 1];
        dist[g.source as usize] = Some(0);
        let mut steps = Vec::new();

        for _ in 0..n {
            let u = (1..=n)
                .filter(|&v| !visited[v as usize] && dist[v as usize].is_some())
                .min_by_key(|&v| dist[v as usize])
                .unwrap_or(0);
            if u == 0 {
                break; // rest unreachable
            }
            visited[u as usize] = true;
            for &(v, w) in &adj[u as usize] {
                let nd = dist[u as usize].unwrap() + w as u64;
                let better = match dist[v as usize] {
                    None => true,
                    Some(d) => nd < d,
                };
                if better {
                    dist[v as usize] = Some(nd);
                }
            }
            if include_steps {
                steps.push(MintyStep {
                    current: Some(u),
                    visited: (1..=n).filter(|&v| visited[v as usize]).collect(),
                    distances: (1..=n).map(|v| (v, dist[v as usize])).collect(),
                });
            }
        }

        let mut distances = std::collections::BTreeMap::new();
        for v in 1..=n {
            distances.insert(v, dist[v as usize]);
        }
        let mut paths: std::collections::BTreeMap<u32, Vec<Vec<u32>>> =
            std::collections::BTreeMap::new();

        let mut truncated = false;
        if let Some(t) = target {
            paths.insert(t, routes_to(g, &dist, t, MAX_ROUTES, &mut truncated));
        } else {
            for v in 1..=n {
                paths.insert(v, Vec::new());
            }
        }

        if target.is_none() && dist[g.source as usize].is_some() {
            let mut budget = MAX_ROUTES;
            let mut current_path = vec![g.source];
            let mut in_path = vec![false; n as usize + 1];
            in_path[g.source as usize] = true;

            fn collect_paths(
                u: u32,
                adj: &[Vec<(u32, i64)>],
                dist: &[Option<u64>],
                current_path: &mut Vec<u32>,
                in_path: &mut Vec<bool>,
                paths: &mut std::collections::BTreeMap<u32, Vec<Vec<u32>>>,
                budget: &mut usize,
                truncated: &mut bool,
            ) {
                if *budget == 0 {
                    *truncated = true;
                    return;
                }
                *budget -= 1;
                paths.entry(u).or_default().push(current_path.clone());

                for &(v, w) in &adj[u as usize] {
                    if *truncated {
                        return;
                    }
                    if let (Some(du), Some(dv)) = (dist[u as usize], dist[v as usize]) {
                        if du + w as u64 == dv && !in_path[v as usize] {
                            in_path[v as usize] = true;
                            current_path.push(v);
                            collect_paths(v, adj, dist, current_path, in_path, paths, budget, truncated);
                            current_path.pop();
                            in_path[v as usize] = false;
                        }
                    }
                }
            }

            collect_paths(
                g.source,
                &adj,
                &dist,
                &mut current_path,
                &mut in_path,
                &mut paths,
                &mut budget,
                &mut truncated,
            );

            // The cap may have cut the search before some reachable vertex got a
            // route; give each of those one route so none looks unreachable.
            if truncated {
                for v in 1..=n {
                    if dist[v as usize].is_some() && paths[&v].is_empty() {
                        paths.insert(v, routes_to(g, &dist, v, 1, &mut false));
                    }
                }
            }
        }

        for path_list in paths.values_mut() {
            path_list.sort();
            path_list.dedup();
        }

        Ok((
            MintyResult {
                source: g.source,
                distances,
                paths,
                truncated,
            },
            steps,
        ))
    }
}

/// All shortest routes from the source to `target`, found by walking tight
/// edges (`dist[u] + w == dist[v]`) backwards from `target`. Only vertices on
/// some shortest route to `target` are visited. Routes are simple paths (zero-
/// weight cycles cannot loop), sorted and de-duplicated (parallel edges).
/// Stops at `limit` routes and sets `truncated` if more exist.
fn routes_to(
    g: &GraphInput,
    dist: &[Option<u64>],
    target: u32,
    limit: usize,
    truncated: &mut bool,
) -> Vec<Vec<u32>> {
    if dist[target as usize].is_none() {
        return Vec::new(); // unreachable
    }

    let mut reverse_adj = vec![Vec::new(); g.vertices as usize + 1];
    for e in &g.edges {
        reverse_adj[e.to as usize].push((e.from, e.weight));
    }

    fn walk_back(
        v: u32,
        source: u32,
        reverse_adj: &[Vec<(u32, i64)>],
        dist: &[Option<u64>],
        reversed_path: &mut Vec<u32>,
        in_path: &mut Vec<bool>,
        routes: &mut Vec<Vec<u32>>,
        limit: usize,
        truncated: &mut bool,
    ) {
        if v == source {
            if routes.len() >= limit {
                *truncated = true;
            } else {
                routes.push(reversed_path.iter().rev().copied().collect());
            }
            return;
        }
        for &(u, w) in &reverse_adj[v as usize] {
            if *truncated {
                return;
            }
            if let (Some(du), Some(dv)) = (dist[u as usize], dist[v as usize]) {
                if du + w as u64 == dv && !in_path[u as usize] {
                    in_path[u as usize] = true;
                    reversed_path.push(u);
                    walk_back(u, source, reverse_adj, dist, reversed_path, in_path, routes, limit, truncated);
                    reversed_path.pop();
                    in_path[u as usize] = false;
                }
            }
        }
    }

    let mut routes = Vec::new();
    let mut reversed_path = vec![target];
    let mut in_path = vec![false; g.vertices as usize + 1];
    in_path[target as usize] = true;
    walk_back(
        target,
        g.source,
        &reverse_adj,
        dist,
        &mut reversed_path,
        &mut in_path,
        &mut routes,
        limit,
        truncated,
    );
    routes.sort();
    routes.dedup();
    routes
}
