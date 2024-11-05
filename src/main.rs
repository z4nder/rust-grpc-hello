use core::option::Option::Some;
use std::env;

use helloworld::hello_service_client::HelloServiceClient;
use helloworld::hello_service_server::{HelloService, HelloServiceServer};
use helloworld::{HelloRequest, HelloResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyHelloService {}

#[tonic::async_trait]
impl HelloService for MyHelloService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!(
            "Received message from client: {:?}",
            request.get_ref().message
        );
        let message = HelloResponse {
            message: format!("Hello back from server!"),
        };
        println!("Responding to client with: {:?}", message);
        Ok(Response::new(message))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        //"cargo run server <porta>"
        Some("server") => {
            let port = args
                .get(2)
                .expect("Please provide a port for the server")
                .parse::<u16>()
                .expect("Invalid port number");
            server_mode(port).await?;
        }

        // "cargo run client <porta_origem> <porta_destino> <mensagem>"
        Some("client") => {
            let origin_port = args
                .get(2)
                .expect("Please provide an origin port")
                .parse::<u16>()
                .expect("Invalid origin port number");
            let target_port = args
                .get(3)
                .expect("Please provide a target port")
                .parse::<u16>()
                .expect("Invalid target port number");
            let message = args.get(4).expect("Please provide a message").as_str();

            println!(
                "Client on port {} sending message to port {}: {}",
                origin_port, target_port, message
            );

            send_message_to_peer(target_port, message).await?;
        }

        _ => {
            eprintln!("Usage:");
            eprintln!("  cargo run server <port>               # Start server on specified port");
            eprintln!("  cargo run client <origin_port> <target_port> <message>  # Send message from client to target server");
        }
    }

    Ok(())
}

async fn server_mode(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("[::1]:{}", port).parse()?;
    let hello_service = MyHelloService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(HelloServiceServer::new(hello_service))
        .serve(addr)
        .await?;

    Ok(())
}

async fn send_message_to_peer(
    target_port: u16,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let peer_addr = format!("http://[::1]:{}", target_port);
    let mut client = HelloServiceClient::connect(peer_addr).await?;
    let request = tonic::Request::new(HelloRequest {
        message: message.to_string(),
    });

    let response = client.say_hello(request).await?;
    println!("Response from peer: {:?}", response.into_inner().message);

    Ok(())
}
