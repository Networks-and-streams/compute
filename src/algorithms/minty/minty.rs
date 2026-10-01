use super::{Algorithm, MintyResult, MintyStep};
use crate::graph::GraphInput;

pub struct MintyAlgorithm;

impl Algorithm for MintyAlgorithm {
    fn name(&self) -> &'static str {
        "minty"
    }

    fn execute(
        &self,
        g: &GraphInput,
        include_steps: bool,
    ) -> Result<(MintyResult, Vec<MintyStep>), crate::error::ComputeError> {
        g.validate()?;
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
        let mut paths: std::collections::BTreeMap<u32, Vec<Vec<u32>>> =
            std::collections::BTreeMap::new();
        for v in 1..=n {
            distances.insert(v, dist[v as usize]);
            paths.insert(v, Vec::new());
        }

        if dist[g.source as usize].is_some() {
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
            ) {
                paths.entry(u).or_default().push(current_path.clone());

                for &(v, w) in &adj[u as usize] {
                    if let (Some(du), Some(dv)) = (dist[u as usize], dist[v as usize]) {
                        if du + w as u64 == dv && !in_path[v as usize] {
                            in_path[v as usize] = true;
                            current_path.push(v);
                            collect_paths(v, adj, dist, current_path, in_path, paths);
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
            );
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
            },
            steps,
        ))
    }
}
