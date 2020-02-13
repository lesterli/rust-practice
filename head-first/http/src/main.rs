mod rpc_server;

use jsonrpc_core::futures::Future;

fn main() {
    let mut io = rpc_server::rpc_handler();

    let request = r#"{"jsonrpc": "2.0", "method": "hello", "params": ["world"], "id": 1}"#;
    let response = r#"{"jsonrpc":"2.0","result":"hello world","id":1}"#;

    assert_eq!(io.handle_request(request).wait().unwrap(), Some(response.to_string()));
}