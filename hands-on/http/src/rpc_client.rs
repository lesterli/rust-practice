use jsonrpc_core_client::{RpcChannel, TypedClient, RpcError};

use futures::Future;

#[derive(Clone)]
pub struct RpcClient(TypedClient);

impl From<RpcChannel> for RpcClient {
    fn from(channel: RpcChannel) -> Self {
        RpcClient(channel.into())
    }
}

impl RpcClient {
    pub fn hello(&self, msg: &'static str) -> impl Future<Item = String, Error = RpcError> {
        self.0.call_method("hello", "String", (msg,))
    }

    pub fn fail(&self) -> impl Future<Item = (), Error = RpcError> {
        self.0.call_method("fail", "()", ())
    }

    pub fn notify(&self, value: u64) -> impl Future<Item = (), Error = RpcError> {
        self.0.notify("notify", (value,))
    }
}