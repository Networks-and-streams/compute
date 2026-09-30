use proto::compute_service_server::ComputeService;
use proto::{ComputeRequest, ComputeResponse};

pub mod proto {
    tonic::include_proto!("compute");
}

#[derive(Debug, Default)]
pub struct ComputeServer;

fn grpc_status(e: &crate::error::ComputeError) -> &'static str {
    match e {
        crate::error::ComputeError::InvalidGraph(_) => "InvalidGraph",
        crate::error::ComputeError::UnsupportedAlgorithm(_) => "UnsupportedAlgorithm",
        crate::error::ComputeError::Execution(_) => "AlgorithmExecutionError",
    }
}

#[tonic::async_trait]
impl ComputeService for ComputeServer {
    async fn execute(
        &self,
        request: tonic::Request<ComputeRequest>,
    ) -> Result<tonic::Response<ComputeResponse>, tonic::Status> {
        let req = request.into_inner();
        let graph = match req.graph {
            Some(g) => crate::graph::GraphInput {
                vertices: g.vertices,
                edges: g
                    .edges
                    .into_iter()
                    .map(|e| crate::graph::Edge {
                        from: e.from,
                        to: e.to,
                        weight: e.weight,
                    })
                    .collect(),
                source: g.source,
            },
            None => {
                return Ok(tonic::Response::new(ComputeResponse {
                    status: "InvalidRequest".into(),
                    result_json: String::new(),
                    steps_json: String::new(),
                    error: "missing graph".into(),
                }));
            }
        };

        match crate::dispatcher::dispatch(&req.algorithm, &graph, req.include_steps) {
            Ok((result, steps)) => Ok(tonic::Response::new(ComputeResponse {
                status: "ok".into(),
                result_json: result.to_string(),
                steps_json: steps.to_string(),
                error: String::new(),
            })),
            Err(e) => Ok(tonic::Response::new(ComputeResponse {
                status: grpc_status(&e).into(),
                result_json: String::new(),
                steps_json: String::new(),
                error: e.to_string(),
            })),
        }
    }
}
