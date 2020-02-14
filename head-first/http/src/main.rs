mod rpc_server;
mod rpc_client;

use jsonrpc_core::futures::Future;
use hyper::rt;
use jsonrpc_http_server::*;
use jsonrpc_client_transports::transports::http;
use std::time::Duration;

fn id<T>(t: T) -> T {
    t
}

fn main() {
    let mut io = rpc_server::rpc_handler();

    let request = r#"{"jsonrpc": "2.0", "method": "hello", "params": ["world"], "id": 1}"#;
    let response = r#"{"jsonrpc":"2.0","result":"hello world","id":1}"#;

    assert_eq!(io.handle_request(request).wait().unwrap(), Some(response.to_string()));

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