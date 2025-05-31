use crate::constant;
use crate::state::*;
use anchor_lang::AnchorDeserialize;
use anchor_spl::token::TokenAccount;
use anyhow::{anyhow, ensure, Context};
use solana_program::pubkey::Pubkey;
use std::convert::TryInto;
use std::str::FromStr;

use constant::METEORA_DYN_PROGRAM_ID;
use constant::METEORA_DYN_VAULT_BASE_ID;
use constant::METEORA_DYN_VAULT_PROGRAM_ID;
use constant::VAULT_WITH_NON_PDA_BASED_LP_MINT;

use crate::rpc_service::RpcService;

pub struct QuoteService {
    rpc_service: RpcService,
}

impl QuoteService {
    pub fn new(rpc_service: RpcService) -> Self {
        Self { rpc_service }
    }

    pub fn get_for_pair(
        &self,
        token_in: Pubkey,
        pool_address: Pubkey,
        amount: u64,
    ) -> Result<QuoteResult, anyhow::Error> {
        let rpc_service = &self.rpc_service;

        // Get current slot and timestamp
        let slot = rpc_service.get_slot();
        let block_time = rpc_service.get_block_time(slot);

        // Fetch the account data
        let pool_account_data = rpc_service.get_account_data(&pool_address);

        // 875
        let pool = Pool::try_from_slice(&pool_account_data[8..532]).unwrap();

        let MeteoraAccounts {
            base_vault_authority,
            quote_vault_authority,
            base_token_vault,
            quote_token_vault,
            base_vault_lp_address,
            quote_vault_lp_address,
            base_vault_lp_mint_address,
            quote_vault_lp_mint_address,
        } = get_all_accounts_for_quote(pool.token_a_mint, pool.token_b_mint, pool_address);

        println!("Pool: {:?}", pool);

        let base_vault =
            Vault::try_from_slice(&rpc_service.get_account_data(&base_vault_authority)[8..1227])
                .unwrap();
        let quote_vault =
            Vault::try_from_slice(&rpc_service.get_account_data(&quote_vault_authority)[8..1227])
                .unwrap();

        // get mint account total supply
        let base_vault_lp_mint = rpc_service.get_mint(&base_vault_lp_mint_address);
        let quote_vault_lp_mint = rpc_service.get_mint(&quote_vault_lp_mint_address);

        // Get token accounts
        let pool_vault_a_lp_token = rpc_service.get_token_account(&base_vault_lp_address);
        let pool_vault_b_lp_token = rpc_service.get_token_account(&quote_vault_lp_address);

        let vault_a_token = rpc_service.get_token_account(&base_token_vault);
        let vault_b_token = rpc_service.get_token_account(&quote_token_vault);

        let quote_data = QuoteData {
            pool,
            vault_a: base_vault,
            vault_b: quote_vault,
            pool_vault_a_lp_token,
            pool_vault_b_lp_token,
            vault_a_lp_mint_supply: base_vault_lp_mint.supply,
            vault_b_lp_mint_supply: quote_vault_lp_mint.supply,
            vault_a_token,
            vault_b_token,
            slot: slot,
            block_time: block_time,
        };

        // print every value from quote_data
        println!("Vault A: {:?}", quote_data.vault_a.total_amount);
        println!("Vault B: {:?}", quote_data.vault_b.total_amount);
        println!(
            "Pool Vault A LP Token: {:?}",
            quote_data.pool_vault_a_lp_token
        );
        println!(
            "Pool Vault B LP Token: {:?}",
            quote_data.pool_vault_b_lp_token
        );
        println!(
            "Vault A LP Mint Supply: {:?}",
            quote_data.vault_a_lp_mint_supply
        );
        println!(
            "Vault B LP Mint Supply: {:?}",
            quote_data.vault_b_lp_mint_supply
        );
        println!("Vault A Token: {:?}", quote_data.vault_a_token);
        println!("Vault B Token: {:?}", quote_data.vault_b_token);
        println!("Slot: {:?}", quote_data.slot);
        println!("Block Time: {:?}", quote_data.block_time);

        let quote_result = compute_quote(token_in, amount, quote_data)?;

        Ok(quote_result)
    }
}

#[derive(Clone)]
struct QuoteData {
    /// Pool state to swap
    pub pool: Pool,
    /// Vault state of vault A
    pub vault_a: Vault,
    /// Vault state of vault B
    pub vault_b: Vault,
    /// Pool vault A LP token
    pub pool_vault_a_lp_token: TokenAccount,
    /// Pool vault B LP token
    pub pool_vault_b_lp_token: TokenAccount,
    /// Lp supply of mint of vault A
    pub vault_a_lp_mint_supply: u64,
    /// Lp supply of mint of vault B
    pub vault_b_lp_mint_supply: u64,
    /// Token account of vault A
    pub vault_a_token: TokenAccount,
    /// Token account of vault B
    pub vault_b_token: TokenAccount,
    /// Slot
    pub slot: u64,
    /// Epoch start timestamp
    pub block_time: i64,
}

#[derive(Debug, Clone)]
pub struct QuoteResult {
    /// Swap out amount
    pub out_amount: u64,
    /// Total fee amount. Fee is charged based on in token mint.
    pub fee: u64,
}

fn compute_quote(
    in_token_mint: Pubkey,
    in_amount: u64,
    quote_data: QuoteData,
) -> anyhow::Result<QuoteResult> {
    let QuoteData {
        pool,
        vault_a,
        vault_b,
        pool_vault_a_lp_token,
        pool_vault_b_lp_token,
        vault_a_lp_mint_supply,
        vault_b_lp_mint_supply,
        vault_a_token,
        vault_b_token,
        slot,
        block_time,
    } = quote_data;

    let activation_type =
        ActivationType::try_from(pool.bootstrapping.activation_type).map_err(|e| anyhow!(e))?;

    let current_point = match activation_type {
        ActivationType::Slot => slot,
        ActivationType::Timestamp => block_time as u64,
    };

    ensure!(pool.enabled, "Pool disabled");
    ensure!(
        current_point >= pool.bootstrapping.activation_point,
        "Swap is disabled"
    );

    let current_time: u64 = block_time.try_into()?;

    ensure!(
        in_token_mint == pool.token_a_mint || in_token_mint == pool.token_b_mint,
        "In token mint not matches with pool token mints"
    );

    let token_a_amount = vault_a
        .get_amount_by_share(
            current_time,
            pool_vault_a_lp_token.amount,
            vault_a_lp_mint_supply,
        )
        .context("Fail to get token a amount")?;

    let token_b_amount = vault_b
        .get_amount_by_share(
            current_time,
            pool_vault_b_lp_token.amount,
            vault_b_lp_mint_supply,
        )
        .context("Fail to get token b amount")?;

    let trade_direction = if in_token_mint == pool.token_a_mint {
        TradeDirection::AtoB
    } else {
        TradeDirection::BtoA
    };

    let (
        mut in_vault,
        out_vault,
        in_vault_lp_amount,
        in_vault_lp_mint_supply,
        out_vault_lp_mint_supply,
        out_vault_token_account,
        in_token_total_amount,
        out_token_total_amount,
    ) = match trade_direction {
        TradeDirection::AtoB => (
            vault_a,
            vault_b,
            pool_vault_a_lp_token.amount,
            vault_a_lp_mint_supply,
            vault_b_lp_mint_supply,
            vault_b_token,
            token_a_amount,
            token_b_amount,
        ),
        TradeDirection::BtoA => (
            vault_b,
            vault_a,
            pool_vault_b_lp_token.amount,
            vault_b_lp_mint_supply,
            vault_a_lp_mint_supply,
            vault_a_token,
            token_b_amount,
            token_a_amount,
        ),
    };

    let trade_fee = pool
        .fees
        .trading_fee(in_amount.into())
        .context("Fail to calculate trading fee")?;

    let protocol_fee = pool
        .fees
        .protocol_trading_fee(trade_fee)
        .context("Fail to calculate protocol trading fee")?;

    // Protocol fee is a cut from trade fee
    let trade_fee = trade_fee
        .checked_sub(protocol_fee)
        .context("Fail to calculate trade fee")?;

    let in_amount_after_protocol_fee = in_amount
        .checked_sub(protocol_fee.try_into()?)
        .context("Fail to calculate in_amount_after_protocol_fee")?;

    println!(
        "In Amount After Protocol Fee: {:?}",
        in_amount_after_protocol_fee
    );

    let before_in_token_total_amount = in_token_total_amount;

    let in_lp = in_vault
        .get_unmint_amount(
            current_time,
            in_amount_after_protocol_fee,
            in_vault_lp_mint_supply,
        )
        .context("Fail to get in_vault_lp")?;

    println!("In LP: {:?}", in_lp);

    in_vault.total_amount = in_vault
        .total_amount
        .checked_add(in_amount_after_protocol_fee)
        .context("Fail to add in_vault.total_amount")?;

    println!("In Vault Total Amount: {:?}", in_vault.total_amount);

    let after_in_token_total_amount = in_vault
        .get_amount_by_share(
            current_time,
            in_lp
                .checked_add(in_vault_lp_amount)
                .context("Fail to get new in_vault_lp")?,
            in_vault_lp_mint_supply
                .checked_add(in_lp)
                .context("Fail to get new in_vault_lp_mint")?,
        )
        .context("Fail to get after_in_token_total_amount")?;

    println!(
        "After In Token Total Amount: {:?}",
        after_in_token_total_amount
    );
    println!(
        "Before In Token Total Amount: {:?}",
        before_in_token_total_amount
    );

    let actual_in_amount = after_in_token_total_amount
        .checked_sub(before_in_token_total_amount)
        .context("Fail to get actual_in_amount")?;

    let actual_in_amount_after_fee = actual_in_amount
        .checked_sub(trade_fee.try_into()?)
        .context("Fail to calculate in_amount_after_fee")?;

    let swap_curve = ConstantProduct {};

    let SwapResult {
        destination_amount_swapped,
        ..
    } = swap_curve
        .swap(
            actual_in_amount_after_fee,
            in_token_total_amount,
            out_token_total_amount,
            trade_direction,
        )
        .context("Fail to get swap result")?;

    let out_vault_lp = out_vault
        .get_unmint_amount(
            current_time,
            destination_amount_swapped.try_into()?,
            out_vault_lp_mint_supply,
        )
        .context("Fail to get out_vault_lp")?;

    let out_amount = out_vault
        .get_amount_by_share(current_time, out_vault_lp, out_vault_lp_mint_supply)
        .context("Fail to get out_amount")?;

    ensure!(
        out_amount < out_vault_token_account.amount,
        "Out amount > vault reserve"
    );

    Ok(QuoteResult {
        fee: trade_fee.try_into()?,
        out_amount,
    })
}

pub fn get_all_accounts_for_quote(
    token_in: Pubkey,
    token_out: Pubkey,
    pool_address: Pubkey,
) -> MeteoraAccounts {
    let (base_vault_authority, _) = Pubkey::find_program_address(
        &[
            b"vault",
            token_in.as_ref(),
            &Pubkey::from_str(METEORA_DYN_VAULT_BASE_ID)
                .unwrap()
                .as_ref(),
        ],
        &Pubkey::from_str(METEORA_DYN_VAULT_PROGRAM_ID).unwrap(),
    );

    let (quote_vault_authority, _) = Pubkey::find_program_address(
        &[
            b"vault",
            token_out.as_ref(),
            &Pubkey::from_str(METEORA_DYN_VAULT_BASE_ID)
                .unwrap()
                .as_ref(),
        ],
        &Pubkey::from_str(METEORA_DYN_VAULT_PROGRAM_ID).unwrap(),
    );

    println!("A Vault Authority: {}", base_vault_authority); //A Vault
    println!("B Vault Authority: {}", quote_vault_authority); //B Vault

    let (base_token_vault, _) = Pubkey::find_program_address(
        &[b"token_vault", &base_vault_authority.as_ref()],
        &Pubkey::from_str(METEORA_DYN_VAULT_PROGRAM_ID).unwrap(),
    );

    let (quote_token_vault, _) = Pubkey::find_program_address(
        &[b"token_vault", &quote_vault_authority.as_ref()],
        &Pubkey::from_str(METEORA_DYN_VAULT_PROGRAM_ID).unwrap(),
    );

    println!("A Token Vault: {}", base_token_vault); //A Token Vault
    println!("B Token Vault: {}", quote_token_vault); //B Token Vault

    let base_vault_lp_mint = VAULT_WITH_NON_PDA_BASED_LP_MINT
        .get(&base_vault_authority)
        .map_or_else(
            || {
                Pubkey::find_program_address(
                    &[b"lp_mint", base_vault_authority.as_ref()],
                    &Pubkey::from_str(METEORA_DYN_VAULT_PROGRAM_ID).unwrap(),
                )
                .0
            },
            |&lp_mint| lp_mint,
        );

    let quote_vault_lp_mint = VAULT_WITH_NON_PDA_BASED_LP_MINT
        .get(&quote_vault_authority)
        .map_or_else(
            || {
                Pubkey::find_program_address(
                    &[b"lp_mint", quote_vault_authority.as_ref()],
                    &Pubkey::from_str(METEORA_DYN_VAULT_PROGRAM_ID).unwrap(),
                )
                .0
            },
            |&lp_mint| lp_mint,
        );

    println!("A Vault LP Token Mint: {}", base_vault_lp_mint); //A Vault LP Mint
    println!("B Vault LP Token Mint: {}", quote_vault_lp_mint); //B Vault LP Mint

    let (base_vault_lp_address, _) = Pubkey::find_program_address(
        &[base_vault_authority.as_ref(), pool_address.as_ref()],
        &Pubkey::from_str(METEORA_DYN_PROGRAM_ID).unwrap(),
    );

    let (quote_vault_lp_address, _) = Pubkey::find_program_address(
        &[quote_vault_authority.as_ref(), pool_address.as_ref()],
        &Pubkey::from_str(METEORA_DYN_PROGRAM_ID).unwrap(),
    );

    println!("A LP Token Vault : {}", base_vault_lp_address); //A Vault LP
    println!("V LP Token Vault : {}", quote_vault_lp_address); //B Vault LP

    return MeteoraAccounts {
        base_vault_authority,
        quote_vault_authority,
        base_token_vault,
        quote_token_vault,
        base_vault_lp_address,
        quote_vault_lp_address,
        base_vault_lp_mint_address: base_vault_lp_mint,
        quote_vault_lp_mint_address: quote_vault_lp_mint,
    };
}

pub struct MeteoraAccounts {
    pub base_vault_authority: Pubkey,
    pub quote_vault_authority: Pubkey,
    pub base_token_vault: Pubkey,
    pub quote_token_vault: Pubkey,
    pub base_vault_lp_address: Pubkey,
    pub quote_vault_lp_address: Pubkey,
    pub base_vault_lp_mint_address: Pubkey,
    pub quote_vault_lp_mint_address: Pubkey,
}
