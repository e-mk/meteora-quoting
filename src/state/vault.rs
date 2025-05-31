use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;
use std::fmt::Debug;

/// Max strategy number that a vault can support
pub const MAX_STRATEGY: usize = 30;
/// DENOMINATOR of degradation
pub const LOCKED_PROFIT_DEGRADATION_DENOMINATOR: u128 = 1_000_000_000_000;

/// Vault struct
#[derive(Debug, AnchorDeserialize, AnchorSerialize, Clone, Copy)]
pub struct Vault {
    /// The flag, if admin set enable = false, then the user can only withdraw and cannot deposit in the vault.
    pub enabled: u8,
    /// Vault nonce, to create vault seeds
    pub bumps: VaultBumps,
    /// The total liquidity of the vault, including remaining tokens in token_vault and the liquidity in all strategies.
    pub total_amount: u64,
    /// Token account, hold liquidity in vault reserve
    pub token_vault: Pubkey,
    /// Hold lp token of vault, each time rebalance crank is called, vault calculate performance fee and mint corresponding lp token amount to fee_vault. fee_vault is owned by treasury address
    pub fee_vault: Pubkey,
    /// Token mint that vault supports
    pub token_mint: Pubkey,
    /// Lp mint of vault
    pub lp_mint: Pubkey,
    /// The list of strategy addresses that vault supports, vault can support up to MAX_STRATEGY strategies at the same time.
    pub strategies: [Pubkey; MAX_STRATEGY],
    /// The base address to create vault seeds
    pub base: Pubkey,
    /// Admin of vault
    pub admin: Pubkey,
    /// Person who can send the crank. Operator can only send liquidity to strategies that admin defined, and claim reward to account of treasury address
    pub operator: Pubkey,
    /// Stores information for locked profit.
    pub locked_profit_tracker: LockedProfitTracker,
}

impl Vault {
    /// Get amount by share
    pub fn get_amount_by_share(
        &self,
        current_time: u64,
        share: u64,
        total_supply: u64,
    ) -> Option<u64> {
        let total_amount = self.get_unlocked_amount(current_time)?;
        u64::try_from(
            u128::from(share)
                .checked_mul(u128::from(total_amount))?
                .checked_div(u128::from(total_supply))?,
        )
        .ok()
    }
    /// Get unlocked amount of vault
    pub fn get_unlocked_amount(&self, current_time: u64) -> Option<u64> {
        self.total_amount.checked_sub(
            self.locked_profit_tracker
                .calculate_locked_profit(current_time)?,
        )
    }

    /// Get unmint amount by token amount
    pub fn get_unmint_amount(
        &self,
        current_time: u64,
        out_token: u64,
        total_supply: u64,
    ) -> Option<u64> {
        let total_amount = self.get_unlocked_amount(current_time)?;
        u64::try_from(
            u128::from(out_token)
                .checked_mul(u128::from(total_supply))?
                .checked_div(u128::from(total_amount))?,
        )
        .ok()
    }
}

/// Vault bumps struct
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Default, Debug, Copy)]
pub struct VaultBumps {
    /// vault_bump
    pub vault_bump: u8,
    /// token_vault_bump
    pub token_vault_bump: u8,
}

/// LockedProfitTracker struct
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug)]
pub struct LockedProfitTracker {
    /// The total locked profit from the last report
    pub last_updated_locked_profit: u64,
    /// The last timestamp (in seconds) rebalancing
    pub last_report: u64,
    /// Rate per second of degradation
    pub locked_profit_degradation: u64,
}

impl LockedProfitTracker {
    /// Calculate locked profit, based from Yearn `https://github.com/yearn/yearn-vaults/blob/main/contracts/Vault.vy#L825`
    pub fn calculate_locked_profit(&self, current_time: u64) -> Option<u64> {
        let duration = u128::from(current_time.checked_sub(self.last_report)?);
        let locked_profit_degradation = u128::from(self.locked_profit_degradation);
        let locked_fund_ratio = duration.checked_mul(locked_profit_degradation)?;

        if locked_fund_ratio > LOCKED_PROFIT_DEGRADATION_DENOMINATOR {
            return Some(0);
        }
        let locked_profit = u128::from(self.last_updated_locked_profit);

        let locked_profit = (locked_profit
            .checked_mul(LOCKED_PROFIT_DEGRADATION_DENOMINATOR - locked_fund_ratio)?)
        .checked_div(LOCKED_PROFIT_DEGRADATION_DENOMINATOR)?;
        let locked_profit = u64::try_from(locked_profit).ok()?;
        Some(locked_profit)
    }
}
