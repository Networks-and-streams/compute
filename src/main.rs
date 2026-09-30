use compute::server::{proto::compute_service_server::ComputeServiceServer, ComputeServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::var("COMPUTE_ADDR").unwrap_or_else(|_| "0.0.0.0:50051".into());
    let addr = addr.parse()?;
    println!("compute listening on {addr}");
    tonic::transport::Server::builder()
        .add_service(ComputeServiceServer::new(ComputeServer::default()))
        .serve(addr)
        .await?;
    Ok(())
}
