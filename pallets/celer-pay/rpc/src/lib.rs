//! RPC interface for the transaction payment module.

use codec::Codec;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

pub use self::gen_client::Client as CelerPayModuleClient;
pub use celer_pay_module_rpc_runtime_api::CelerPayModuleApi as CelerPayModuleRuntimeApi;

/// Celer Pay Module RPC methods
#[rpc]
pub trait CelerPayModuleApi<BlockHash, AccountId, Hash, Balance, BlockNumber> {
    #[rpc(name = "celerPayModule_getCelerLedgerId")]
    fn get_celer_ledger_id(&self, at: Option<BlockHash>) -> Result<AccountId>;

    #[rpc(name = "celerPaymodule_getSettleFinalizedTime")]
    fn get_settle_finalized_time(&self, channel_id: Hash, at: Option<BlockHash>) -> Result<Option<BlockNumber>>;

    #[rpc(name = "celerPayModule_getChannelStatus")]
    fn get_channel_status(&self, channel_id: Hash, at: Option<BlockHash>) -> Result<u8>;
    
    #[rpc(name = "celerPayModule_getCooperativeWithdrawSeqNum")]
    fn get_cooperative_withdraw_seq_num(&self, channel_id: Hash, at: Option<BlockHash>)  -> Result<Option<u128>>;

    #[rpc(name = "celerPayModule_getBalanceMap")]
    fn get_balance_map(&self, channel_id: Hash, at: Option<BlockHash>) -> Result<(Vec<AccountId>, Vec<Balance>, Vec<Balance>)>;

    #[rpc(name = "celerPayModule_getStateSeqNumMap")]
    fn get_state_seq_num_map(&self, channel_id: Hash, at: Option<BlockHash>) -> Result<Option<(Vec<AccountId>, Vec<u128>)>>;

    #[rpc(name = "celerPayModule_getTransferOutMap")]
    fn get_transfer_out_map(&self, channel_id: Hash, at: Option<BlockHash>) -> Result<Option<(Vec<AccountId>, Vec<Balance>)>>;

    #[rpc(name = "celerPayModule_getNextPayIdListHashMap")]
    fn get_next_pay_id_list_hash_map(&self, channel_id: Hash, at: Option<BlockHash>) -> Result<Option<(Vec<AccountId>, Vec<Hash>)>>;

    #[rpc(name = "celerPayModule_getLastPayResolveDeadlineMap")]
    fn get_last_pay_resolve_deadline_map(&self, channel_id: Hash, at: Option<BlockHash>) -> Result<Option<(Vec<AccountId>, Vec<BlockNumber>)>>;

    #[rpc(name = "celerPayModule_getPendingPayOutMap")]
    fn get_pending_pay_out_map(&self, channel_id: Hash, at: Option<BlockHash>) -> Result<Option<(Vec<AccountId>, Vec<Balance>)>>;

    #[rpc(name = "celerPayModule_getWithdrawIntent")]
    fn get_withdraw_intent(&self, channel_id: Hash, at: Option<BlockHash>) -> Result<Option<(AccountId, Balance, BlockNumber, Hash)>>;

    #[rpc(name = "celerPayModule_getChannelStatusNum")]
    fn get_channel_status_num(&self, channel_status: u8, at: Option<BlockHash>) -> Result<Option<u8>>;

    #[rpc(name = "celerPayModule_getBalanceLimit")]
    fn get_balance_limit(&self, channel_id: Hash, at: Option<BlockHash>) -> Result<Option<Balance>>;

    #[rpc(name = "celerPayModule_getBalanceLimitsEnabled")]
    fn get_balance_limits_enabled(&self, channel_id: Hash, at: Option<BlockHash>) -> Result<Option<bool>>;

    #[rpc(name = "celerPayModule_getPeersMigrationInfo")]
    fn get_peers_migration_info(&self, channel_id: Hash, at: Option<BlockHash>)-> Result<Option<(Vec<AccountId>, Vec<Balance>, Vec<Balance>, Vec<u128>, Vec<Balance>, Vec<Balance>)>>;

    #[rpc(name = "celerPayModule_getCelerWalletId")]
    fn get_celer_wallet_id(&self, at: Option<BlockHash>) -> Result<AccountId>;

    #[rpc(name = "celerPayModule_getWalletOwners")]
    fn get_wallet_owners(&self, wallet_id: Hash, at: Option<BlockHash>) -> Result<Option<Vec<AccountId>>>;

    #[rpc(name = "celerPayModule_getBalance")]
    fn get_wallet_balance(&self, wallet_id: Hash, at: Option<BlockHash>) -> Result<Option<Balance>>;

    #[rpc(name = "celerPayModule_getPoolId")]
    fn get_pool_id(&self, at: Option<BlockHash>) -> Result<AccountId>;

    #[rpc(name = "celerPayModule_balanceOf")]
    fn get_pool_balance(&self, owner: AccountId, at: Option<BlockHash>) -> Result<Option<Balance>>;

    #[rpc(name = "celerPayModule_allowance")]
    fn get_allowance(&self, owner: AccountId, spender: AccountId, at: Option<BlockHash>) -> Result<Option<Balance>>;

    #[rpc(name = "celerPayModule_getPayResolverId")]
    fn get_pay_resolver_id(&self, at: Option<BlockHash>) -> Result<AccountId>;

    #[rpc(name = "celerPayModule_calculatePayId")]
    fn calculate_pay_id(&self, pay_hash: Hash, at: Option<BlockHash>) -> Result<Hash>;
}

/// A struct that implements the `CelerPayModuleApi'
pub struct CelerPayModule<C, P> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<P>,
}

impl<C, P> CelerPayModule<C, P> {
    pub fn new (client: Arc<C>) -> Self {
        CelerPayModule {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId, Hash, Balance, BlockNumber> 
    CelerPayModuleApi<
        <Block as BlockT>::Hash, 
        AccountId,
        Hash,
        Balance,
        BlockNumber,
    > for CelerPayModule<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: CelerPayModuleRuntimeApi<Block, AccountId, Hash, Balance, BlockNumber>,
    AccountId: Codec,
    Hash: Codec,
    Balance: Codec,
    BlockNumber: Codec,
{
    fn get_celer_ledger_id(&self, at: Option<<Block as BlockT>::Hash>) -> Result<AccountId> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_celer_ledger_id(&at);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get celer ledger id".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_settle_finalized_time(&self, channel_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Option<BlockNumber>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_settle_finalized_time(&at, channel_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get settle finalized time".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_channel_status(&self, channel_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<u8> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_channel_status(&at, channel_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get Channel Status".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_cooperative_withdraw_seq_num(&self, channel_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Option<u128>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_cooperative_withdraw_seq_num(&at, channel_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get cooperative wihtdraw sequence number".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    
    fn get_balance_map(&self, channel_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<(Vec<AccountId>, Vec<Balance>, Vec<Balance>)> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_balance_map(&at, channel_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get balance map".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_state_seq_num_map(&self, channel_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Option<(Vec<AccountId>, Vec<u128>)>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_state_seq_num_map(&at, channel_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get state sequence number map".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_transfer_out_map(&self, channel_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Option<(Vec<AccountId>, Vec<Balance>)>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_transfer_out_map(&at, channel_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get transfer out map".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_next_pay_id_list_hash_map(&self, channel_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Option<(Vec<AccountId>, Vec<Hash>)>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_next_pay_id_list_hash_map(&at, channel_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get next pay id list hash map".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_last_pay_resolve_deadline_map(&self, channel_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Option<(Vec<AccountId>, Vec<BlockNumber>)>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_last_pay_resolve_deadline_map(&at, channel_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get next pay resolve deadline map".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }    

    fn get_pending_pay_out_map(&self, channel_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Option<(Vec<AccountId>, Vec<Balance>)>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_pending_pay_out_map(&at, channel_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get pending pay out map".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }   

    fn get_withdraw_intent(&self, channel_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Option<(AccountId, Balance, BlockNumber, Hash)>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_withdraw_intent(&at, channel_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get withdraw intent".into(),
            data: Some(format!("{:?}", e).into()),
        })
    } 

    fn get_channel_status_num(&self, channel_status: u8, at: Option<<Block as BlockT>::Hash>) -> Result<Option<u8>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_channel_status_num(&at, channel_status);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get channel status num".into(),
            data: Some(format!("{:?}", e).into()),
        })
    } 

    fn get_balance_limit(&self, channel_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Option<Balance>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_balance_limit(&at, channel_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get balance limit".into(),
            data: Some(format!("{:?}", e).into()),
        })
    } 

    fn get_balance_limits_enabled(&self, channel_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Option<bool>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_balance_limits_enabled(&at, channel_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get balance limits enabled".into(),
            data: Some(format!("{:?}", e).into()),
        })
    } 

    fn get_peers_migration_info(&self, channel_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Option<(Vec<AccountId>, Vec<Balance>, Vec<Balance>, Vec<u128>, Vec<Balance>, Vec<Balance>)>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_peers_migration_info(&at, channel_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get peers migration info".into(),
            data: Some(format!("{:?}", e).into()),
        })
    } 

    fn get_celer_wallet_id(&self, at: Option<<Block as BlockT>::Hash>) -> Result<AccountId> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_celer_wallet_id(&at);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get celer wallet id".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_wallet_owners(&self, wallet_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Option<Vec<AccountId>>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_wallet_owners(&at, wallet_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get wallet owners".into(),
            data: Some(format!("{:?}", e).into()),
        })
    } 

    fn get_wallet_balance(&self, wallet_id: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Option<Balance>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_wallet_balance(&at, wallet_id);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get wallet owners".into(),
            data: Some(format!("{:?}", e).into()),
        })
    } 

    fn get_pool_id(&self, at: Option<<Block as BlockT>::Hash>) -> Result<AccountId> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_pool_id(&at);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get pool id".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_pool_balance(&self, owner: AccountId, at: Option<<Block as BlockT>::Hash>) -> Result<Option<Balance>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_pool_balance(&at, owner);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get pool balance".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_allowance(&self, owner: AccountId, spender: AccountId, at: Option<<Block as BlockT>::Hash>) -> Result<Option<Balance>> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_allowance(&at, owner, spender);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get allowed balance of spender".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_pay_resolver_id(&self, at: Option<<Block as BlockT>::Hash>) -> Result<AccountId> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.get_pay_resolver_id(&at);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't get pay resolver id".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn calculate_pay_id(&self, pay_hash: Hash, at: Option<<Block as BlockT>::Hash>) -> Result<Hash> {
        let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.calculate_pay_id(&at, pay_hash);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Can't calculate pay id".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
}
