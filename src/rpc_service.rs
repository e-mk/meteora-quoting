use anchor_lang::AccountDeserialize;
use anchor_spl::token::{Mint, TokenAccount};
use solana_client::rpc_client::RpcClient;
use solana_program::clock::Slot;
use solana_program::pubkey::Pubkey;

pub struct RpcService {
    pub rpc_client: RpcClient,
}

impl RpcService {
    pub fn new(rpc_url: String) -> Self {
        Self {
            rpc_client: RpcClient::new(rpc_url),
        }
    }

    pub fn get_slot(&self) -> Slot {
        self.rpc_client.get_slot().unwrap()
    }

    pub fn get_block_time(&self, slot: Slot) -> i64 {
        self.rpc_client.get_block_time(slot).unwrap()
    }

    pub fn get_mint(&self, pubkey: &Pubkey) -> Mint {
        Mint::try_deserialize_unchecked(&mut &self.rpc_client.get_account_data(pubkey).unwrap()[..])
            .unwrap()
    }

    pub fn get_token_account(&self, pubkey: &Pubkey) -> TokenAccount {
        TokenAccount::try_deserialize_unchecked(
            &mut &self.rpc_client.get_account_data(pubkey).unwrap()[..],
        )
        .unwrap()
    }

    pub fn get_account_data(&self, pubkey: &Pubkey) -> Vec<u8> {
        self.rpc_client.get_account_data(pubkey).unwrap()
    }
}
