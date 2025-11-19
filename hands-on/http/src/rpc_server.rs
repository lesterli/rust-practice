use std::net::SocketAddr;
use std::io;

use jsonrpc_core::{Error, ErrorCode, IoHandler, Params, Value, Result};
use jsonrpc_http_server::{Server, ServerBuilder, RestApi};
use jsonrpc_derive::rpc;

pub struct RpcServer {
	pub uri: String,
	socket_addr: SocketAddr,
	server: Option<Server>,
}

impl RpcServer {
	pub fn serve<F: FnOnce(ServerBuilder) -> ServerBuilder>(alter: F) -> Self {
		let builder = ServerBuilder::new(rpc_handler()).rest_api(RestApi::Unsecure);

		let server = alter(builder).start_http(&"127.0.0.1:0".parse().unwrap()).unwrap();
		let socket_addr = server.address().clone();
		let uri = format!("http://{}", socket_addr);

		RpcServer {
			uri,
			socket_addr,
			server: Some(server),
		}
	}

	fn start(&mut self) {
		if self.server.is_none() {
			let server = ServerBuilder::new(rpc_handler())
				.rest_api(RestApi::Unsecure)
				.start_http(&self.socket_addr)
				.unwrap();
			self.server = Some(server);
		} else {
			panic!("Server already running")
		}
	}

	fn stop(&mut self) {
		let server = self.server.take();
		if let Some(server) = server {
			server.close();
		}
	}
}

pub fn rpc_handler() -> IoHandler {
	let mut io = IoHandler::default();
	io.add_method("hello", |params: Params| match params.parse::<(String,)>() {
		Ok((msg,)) => Ok(Value::String(format!("hello {}", msg))),
		_ => Ok(Value::String("world".into())),
	});
	io.add_method("fail", |_: Params| Err(Error::new(ErrorCode::ServerError(-34))));
	io.add_notification("notify", |params: Params| {
		let (value,) = params.parse::<(u64,)>().expect("expected one u64 as param");
		assert_eq!(value, 12);
	});

	io
}

/// Maximal payload accepted by RPC servers.
const MAX_PAYLOAD: usize = 15 * 1024 * 1024;

/// Type alias for http server
pub type HttpServer = Server;

/// The RPC IoHandler containing all requested APIs.
pub type RpcHandler = IoHandler;

/// Start HTTP server listening on given address.
///
/// **Note**: Only available if `not(target_os = "unknown")`.
pub fn start_http(
	addr: &std::net::SocketAddr,
	io: RpcHandler,
) -> io::Result<Server> {
	ServerBuilder::new(io)
		//.threads(4)
		.rest_api(RestApi::Unsecure)
		.max_request_body_size(MAX_PAYLOAD)
		.start_http(addr)
}

/// API
#[rpc]
pub trait Rpc {
	/// Adds two numbers and returns a result
	#[rpc(name = "add")]
	fn add(&self, a: u64, b: u64) -> Result<u64>;
}