mod rpc_server;
mod rpc_client;

use hyper::rt;
use std::time::Duration;
use std::net::SocketAddr;

use jsonrpc_core::futures::Future;
use jsonrpc_http_server::*;
use jsonrpc_client_transports::transports::http;
use rpc_server::Rpc;
use jsonrpc_core::{Result};
use jsonrpc_core_client::transports::local;
use self::rpc_server::gen_client;

fn id<T>(t: T) -> T {
    t
}

fn example() {
    let mut io = rpc_server::rpc_handler();

    let request = r#"{"jsonrpc": "2.0", "method": "hello", "params": ["world"], "id": 1}"#;
    let response = r#"{"jsonrpc":"2.0","result":"hello world","id":1}"#;

    assert_eq!(io.handle_request(request).wait().unwrap(), Some(response.to_string()));
}

fn example2() {
    // init RPC server
    let server = rpc_server::RpcServer::serve(id);
    let (tx, rx) = std::sync::mpsc::channel();

    // create connect
    let run = http::connect(&server.uri)
        .and_then(|client: rpc_client::RpcClient| {
            client.hello("http").and_then(move |result| {
                drop(client);
                let _ = tx.send(result);
                Ok(())
            })
        })
        .map_err(|e| println!("RPC Client error: {:?}", e));

    rt::run(run);

    // get response
    let result = rx.recv_timeout(Duration::from_secs(3)).unwrap();
    assert_eq!("hello http", result);
}

struct RpcImpl;

impl Rpc for RpcImpl {
	fn add(&self, a: u64, b: u64) -> Result<u64> {
		Ok(a + b)
	}
}

fn example3() {
    let mut handler = rpc_server::rpc_handler();
    handler.extend_with(RpcImpl.to_delegate());
    
    // let server_details = "0.0.0.0:15678";
    // let server_addr: SocketAddr = server_details.parse().unwrap();
    // let new_server = rpc_server::start_http(&server_addr, handler);

    let fut = {
        let (client, server) = local::connect::<gen_client::Client, _, _>(handler);
		client.add(5, 6).map(|res| println!("5 + 6 = {}", res)).join(server)
	};
	fut.wait().unwrap();  
}

fn example1() {
    // init RPC server
    let server_details = "0.0.0.0:15678";
    let socket_addr: SocketAddr = server_details.parse().unwrap();
    let mut handler = rpc_server::rpc_handler();
    let new_server = rpc_server::start_http(&socket_addr, handler);
    let server_uri = format!("http://{}", socket_addr);

    let (tx, rx) = std::sync::mpsc::channel();

    // create connect
    let run = http::connect(&server_uri)
        .and_then(|client: rpc_client::RpcClient| {
            client.hello("http rpc").and_then(move |result| {
                drop(client);
                let _ = tx.send(result);
                Ok(())
            })
        })
        .map_err(|e| println!("RPC Client error: {:?}", e));

    rt::run(run);

    // get response
    let result = rx.recv_timeout(Duration::from_secs(3)).unwrap();
    assert_eq!("hello http rpc", result);
    println!("RPC Client example1: {:?}", result);
}

fn main() {
    // example
    example();

    example1();

    // example
    example2();

    // example
    example3();
}