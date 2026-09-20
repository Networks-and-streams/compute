use crate::{
    algorithms::minty::{minty::MintyAlgorithm, Algorithm},
    error::ComputeError,
    graph::GraphInput,
};

pub fn dispatch(
    algo: &str,
    g: &GraphInput,
    include_steps: bool,
) -> Result<(serde_json::Value, serde_json::Value), ComputeError> {
    match algo {
        "minty" => {
            let (res, steps) = MintyAlgorithm.execute(g, include_steps)?;
            Ok((
                serde_json::to_value(&res).unwrap(),
                serde_json::to_value(&steps).unwrap(),
            ))
        }
        // "ford_fulkerson" => todo!(), // future, no rewrite needed
        other => Err(ComputeError::UnsupportedAlgorithm(other.into())),
    }
}
