pub mod minty;

use crate::{error::ComputeError, graph::GraphInput};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintyResult {
    pub source: u32,
    pub distances: BTreeMap<u32, Option<u64>>, // None = unreachable
    pub paths: BTreeMap<u32, Vec<Vec<u32>>>,
    /// True when more tied shortest routes exist than were returned (see `MAX_ROUTES`).
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintyStep {
    pub current: Option<u32>,
    pub visited: Vec<u32>,
    pub distances: BTreeMap<u32, Option<u64>>,
}

pub trait Algorithm {
    fn name(&self) -> &'static str;
    fn execute(
        &self,
        g: &GraphInput,
        include_steps: bool,
    ) -> Result<(MintyResult, Vec<MintyStep>), ComputeError>;
}
