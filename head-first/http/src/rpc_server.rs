use std::net::SocketAddr;

use jsonrpc_core::{Error, ErrorCode, IoHandler, Params, Value};
use jsonrpc_http_server::{Server, ServerBuilder, RestApi};

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