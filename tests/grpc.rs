use compute::server::proto::{
    compute_service_client::ComputeServiceClient, compute_service_server::ComputeServiceServer,
    ComputeRequest, Edge, Graph,
};
use compute::server::ComputeServer;

async fn spawn_server() -> String {
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let addr = format!("127.0.0.1:{port}");
    let socket: std::net::SocketAddr = addr.parse().unwrap();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(ComputeServiceServer::new(ComputeServer::default()))
            .serve(socket),
    );
    // Give the server a moment to bind.
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    format!("http://{addr}")
}

fn demo_graph() -> Graph {
    Graph {
        vertices: 4,
        edges: vec![
            Edge { from: 1, to: 2, weight: 5 },
            Edge { from: 1, to: 3, weight: 8 },
            Edge { from: 2, to: 4, weight: 7 },
        ],
        source: 1,
    }
}

#[tokio::test]
async fn grpc_ok_with_steps() {
    let mut client = ComputeServiceClient::connect(spawn_server().await).await.unwrap();
    let resp = client
        .execute(ComputeRequest {
            algorithm: "minty".into(),
            graph: Some(demo_graph()),
            include_steps: true,
        })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(resp.status, "ok");
    assert!(resp.result_json.contains("\"4\":12"));
    assert!(!resp.steps_json.is_empty());
    assert!(resp.error.is_empty());
}

#[tokio::test]
async fn grpc_missing_graph() {
    let mut client = ComputeServiceClient::connect(spawn_server().await).await.unwrap();
    let resp = client
        .execute(ComputeRequest {
            algorithm: "minty".into(),
            graph: None,
            include_steps: false,
        })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(resp.status, "InvalidRequest");
}

#[tokio::test]
async fn grpc_unsupported_algorithm() {
    let mut client = ComputeServiceClient::connect(spawn_server().await).await.unwrap();
    let resp = client
        .execute(ComputeRequest {
            algorithm: "nope".into(),
            graph: Some(demo_graph()),
            include_steps: false,
        })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(resp.status, "UnsupportedAlgorithm");
    assert!(!resp.error.is_empty());
}

#[tokio::test]
async fn grpc_invalid_graph() {
    let mut client = ComputeServiceClient::connect(spawn_server().await).await.unwrap();
    let resp = client
        .execute(ComputeRequest {
            algorithm: "minty".into(),
            graph: Some(Graph { vertices: 0, edges: vec![], source: 1 }),
            include_steps: false,
        })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(resp.status, "InvalidGraph");
}

#[tokio::test]
async fn grpc_multiple_shortest_paths() {
    let mut client = ComputeServiceClient::connect(spawn_server().await).await.unwrap();
    let resp = client
        .execute(ComputeRequest {
            algorithm: "minty".into(),
            graph: Some(Graph {
                vertices: 4,
                edges: vec![
                    Edge { from: 1, to: 2, weight: 5 },
                    Edge { from: 1, to: 3, weight: 5 },
                    Edge { from: 2, to: 4, weight: 7 },
                    Edge { from: 3, to: 4, weight: 7 },
                ],
                source: 1,
            }),
            include_steps: false,
        })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(resp.status, "ok");
    assert!(resp.result_json.contains("\"4\":12"));
    assert!(resp.result_json.contains("[[1,2,4],[1,3,4]]"));
}
