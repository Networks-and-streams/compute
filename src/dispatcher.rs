use crate::{
    algorithms::minty::minty::MintyAlgorithm,
    error::ComputeError,
    graph::GraphInput,
};

pub fn dispatch(
    algo: &str,
    g: &GraphInput,
    include_steps: bool,
    target: Option<u32>,
) -> Result<(serde_json::Value, serde_json::Value), ComputeError> {
    match algo {
        "minty" => {
            let (res, steps) = MintyAlgorithm.execute_for(g, include_steps, target)?;
            Ok((
                serde_json::to_value(&res).unwrap(),
                serde_json::to_value(&steps).unwrap(),
            ))
        }
        // "ford_fulkerson" => todo!(), // future, no rewrite needed
        other => Err(ComputeError::UnsupportedAlgorithm(other.into())),
    }
}
