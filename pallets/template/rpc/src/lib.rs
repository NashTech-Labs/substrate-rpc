//! RPC interface for the transaction payment module.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use something_runtime_api::SomethingApi as SomethingRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

#[rpc]
pub trait SomethingApi<BlockHash> {
    #[rpc(name = "template_getSomething")]
    fn get_something(&self, at: Option<BlockHash>) -> Result<u32>;
}

/// A struct that implements the `SomethingApi` trait.
pub struct Getter<C, M> {
    // If you have more generics, no need to SumStorage<C, M, N, P, ...>
    // just use a tuple like SumStorage<C, (M, N, P, ...)>
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> Getter<C, M> {
    /// new method create new `Getter` instance.
    ///
    /// #Arguments
    ///
    /// * `client`-  A reference to the client.
    ///
    /// #Return
    ///
    /// returns an instance of Getter type.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block> SomethingApi<<Block as BlockT>::Hash> for Getter<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: SomethingRuntimeApi<Block>,
{
    /// get_something method queries the storage by calling a runtime api.
    ///
    /// #Arguments
    ///
    /// * `at`-  An Option containing block hash at which the runtime api is to be called.
    ///
    /// #Return
    ///
    /// returns an u32 value stored in the storage.
    fn get_something(&self, at: Option<<Block as BlockT>::Hash>) -> Result<u32> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_something(&at);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
}
