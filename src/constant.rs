use solana_program::pubkey::pubkey;
use solana_program::pubkey::Pubkey;
use std::collections::HashMap;

pub const RPC_URL: &str = "https://api.mainnet-beta.solana.com";

pub const METEORA_DYN_PROGRAM_ID: &str = "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB";
pub const METEORA_DYN_VAULT_PROGRAM_ID: &str = "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi";
pub const METEORA_DYN_VAULT_BASE_ID: &str = "HWzXGcGHy4tcpYfaRDCyLNzXqBTv3E6BttpCH2vJxArv";

lazy_static! {
  pub static ref VAULT_WITH_NON_PDA_BASED_LP_MINT: HashMap<Pubkey, Pubkey> = HashMap::from_iter([
      (
          // ACUSD
          pubkey!("BFJP6RYDxJa4FmFtBpPDYcrPozjC98CELrXqVL7rGMVW"),
          pubkey!("5CuhvouXVx6t5XPiyhRkrfgK5omAf8XnqY1ef6CLjw7o"),
      ),
      (
          // USH
          pubkey!("AzrUPWWyT9ZoAuMTgGHxYCnnWD2veh98FsCcMknVjg3Q"),
          pubkey!("9MSsSzDKq8VzokicRom6ciYPLhhZf65bCCBQLjnC7jUH")
      ),
      (
          // afUSDC
          pubkey!("GGQfASSnFaqPu83jWrL1DMJBJEzG3rdwsDARDGt6Gxmj"),
          pubkey!("4da9saTYgDs37wRSuS8mnFoiWzSYeRtvSWaFRe8rtkFc"),
      ),
      (
          // Bridged USD Coin (Wormhole Ethereum)
          pubkey!("GofttAxULhp5NE9faWNngsnDM1iJiL75AJ2AkSaaC2CC"),
          pubkey!("Bma9RZx1AjNGcojNJpstGe9Wcytxz17YA6rd2Lq1UirT"),
      ),
      (
          // PAI
          pubkey!("671JaLe2zDgBeXK3UtFHBiid7WFCHAKZTmLqAaQxx7cL"),
          pubkey!("9NywobBSCyntrPSZxEZpUbJXLfgUzKbUF2ZqBBkJLEgB"),
      ),
      (
          // UXD
          pubkey!("2dH3aSpt5aEwhoeSaThKRNtNppEpg2DhGKGa1C5Wecc1"),
          pubkey!("Afe5fiLmbKw7aBi1VgWZb9hEY8nRYtib6LNr5RGUJibP"),
      ),
      (
          // WAVAX
          pubkey!("BVJACEffKRHvKbQT9VfEqoxrUWJN2UVdonTKYB2c4MgK"),
          pubkey!("FFmYsMk5xQq3zQf1r4A6Yyf3kaKd3LUQokeVa776rKWH"),
      ),
      (
          // USDT
          pubkey!("5XCP3oD3JAuQyDpfBFFVUxsBxNjPQojpKuL4aVhHsDok"),
          pubkey!("EZun6G5514FeqYtUv26cBHWLqXjAEdjGuoX6ThBpBtKj"),
      ),
      (
          // WBTC
          pubkey!("mPWBpKzzchEjitz7x4Q2d7cbQ3fHibF2BHWbWk8YGnH"),
          pubkey!("4nCGSVN8ZGuewX36TznzisceaNYzURWPesxyGtDvA2iP"),
      ),
      (
          // mSOL
          pubkey!("8p1VKP45hhqq5iZG5fNGoi7ucme8nFLeChoDWNy7rWFm"),
          pubkey!("21bR3D4QR4GzopVco44PVMBXwHFpSYrbrdeNwdKk7umb"),
      ),
      (
          // stSOL
          pubkey!("CGY4XQq8U4VAJpbkaFPHZeXpW3o4KQ5LowVsn6hnMwKe"),
          pubkey!("28KR3goEditLnzBZShRk2H7xvgzc176EoFwMogjdfSkn"),
      ),
      (
          // wSOL
          pubkey!("FERjPVNEa7Udq8CEv68h6tPL46Tq7ieE49HrE2wea3XT"),
          pubkey!("FZN7QZ8ZUUAxMPfxYEYkH3cXUASzH8EqA6B4tyCL8f1j"),
      ),
      (
          // USDC
          pubkey!("3ESUFCnRNgZ7Mn2mPPUMmXYaKU8jpnV9VtA17M7t2mHQ"),
          pubkey!("3RpEekjLE5cdcG15YcXJUpxSepemvq2FpmMcgo342BwC"),
      ),
  ]);
}
