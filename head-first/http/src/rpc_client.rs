use jsonrpc_core_client::{RpcChannel, TypedClient};

#[derive(Clone)]
struct RpcClient(TypedClient);

impl From<RpcChannel> for RpcClient {
    fn from(channel: RpcChannel) -> Self {
        RpcClient(channel.into())
    }
}

