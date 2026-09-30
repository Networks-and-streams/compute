// Backend-style usage example: talks to a running compute server over gRPC.
use compute::server::proto::{
    compute_service_client::ComputeServiceClient, ComputeRequest, Edge, Graph,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::var("COMPUTE_ADDR").unwrap_or_else(|_| "http://127.0.0.1:50051".into());
    let mut client = ComputeServiceClient::connect(addr).await?;
    let resp = client
        .execute(ComputeRequest {
            algorithm: "minty".into(),
            graph: Some(Graph {
                vertices: 4,
                edges: vec![
                    Edge { from: 1, to: 2, weight: 5 },
                    Edge { from: 1, to: 3, weight: 8 },
                    Edge { from: 2, to: 4, weight: 7 },
                ],
                source: 1,
            }),
            include_steps: true,
        })
        .await?
        .into_inner();
    println!("status: {}", resp.status);
    println!("result: {}", resp.result_json);
    println!("steps: {}", resp.steps_json);
    Ok(())
}
