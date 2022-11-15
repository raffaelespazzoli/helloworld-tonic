
use hello_world::greeter_client::GreeterClient;
use hello_world::{HelloReply, HelloRequest};
use tokio::task::{JoinSet};
use tonic::{Request, Response, Status};
use std::time::Instant;
use clap::Parser;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

/// Run a program or echo a string
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[arg(short, long)]
    times: u32,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let now = Instant::now();
    let pem = tokio::fs::read("ca.pem").await?;
    let ca = Certificate::from_pem(pem);
    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("example.com");
    let channel = Channel::from_static("https://[::1]:50051")
        .tls_config(tls)?
        .connect()
        .await?;
    let elapsed_time = now.elapsed();
    println!("connection established in  {}", elapsed_time.as_millis());

    //let's spawn some requests in parallel
    // let responses: FuturesUnordered<JoinHandle<Result<Response<HelloReply>, Status>>> = FuturesUnordered::new();

    let mut responses: JoinSet<Result<Response<HelloReply>, Status>> = JoinSet::new();
    let mut messages: Vec<String> = Vec::with_capacity(args.times as usize);
    let now = Instant::now();
    for n in 0..args.times {
        let client = GreeterClient::new(channel.clone());
        let request = Request::new(HelloRequest { name: format!("Tonic{n}") });
        let response = {
            let mut client = client.clone();
            async move {
                client.say_hello(request).await
            }
        };
        responses.spawn(response);
    }

    
    // let responses = (0..args.times)
    //     .map(|i| {
    //         let client = GreeterClient::new(channel.clone());
    //         let request = Request::new(HelloRequest {
    //             name: format!("Tonic{i}"),
    //         });
    //         let response = {
    //             let mut client = client.clone();
    //             async move { client.say_hello(request).await }
    //         };
    //         tokio::spawn(response)
    //     })
    //     .collect::<Vec<JoinHandle<Result<Response<HelloReply>, Status>>>>();

    //let f = join_all(responses);
   
    while let Some(response) = responses.join_next().await {
        let message = response.as_ref().unwrap().as_ref().unwrap().get_ref().to_owned().message;
        messages.push(message);
    }
    let elapsed_time = now.elapsed();

    // let now = Instant::now();
    // let mut messages: Vec<String> = vec![];
    // for response in responses {
    //     let message = response.await.as_ref().unwrap().as_ref().unwrap().get_ref().to_owned().message;
    //     messages.push(message);
    // };
    // let elapsed_time = now.elapsed();

    //let's wait for the request to complete
    // let now = Instant::now();
    // let messages: Vec<String> = f
    //     .await
    //     .iter()
    //     .map(|r| r.as_ref().unwrap())
    //     .map(|r| r.as_ref().unwrap())
    //     .map(|r| r.get_ref().to_owned().message)
    //     .collect();
    // let elapsed_time = now.elapsed();

    for message in messages.iter() {
        println!("{}", message)
    }

    println!(
        "{} requests executed in {}",
        args.times,
        elapsed_time.as_millis()
    );

    Ok(())
}
