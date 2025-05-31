pub struct ConstantProduct {}

impl SwapCurve for ConstantProduct {
    fn swap(
        &self,
        source_amount: u64,
        swap_source_amount: u64,
        swap_destination_amount: u64,
        _trade_direction: TradeDirection,
    ) -> Option<SwapResult> {
        // Convert to u128 to prevent overflow during calculations
        let source_amount = source_amount as u128;
        let swap_source_amount = swap_source_amount as u128;
        let swap_destination_amount = swap_destination_amount as u128;

        // Calculate new source amount after adding the input
        let new_swap_source_amount = swap_source_amount.checked_add(source_amount)?;

        // Calculate new destination amount using constant product formula:
        // (x + dx) * (y - dy) = x * y
        // where x = swap_source_amount, y = swap_destination_amount, dx = source_amount
        let new_swap_destination_amount = (swap_source_amount
            .checked_mul(swap_destination_amount)?)
        .checked_div(new_swap_source_amount)?;

        // Calculate how much destination token we get
        let destination_amount_swapped =
            swap_destination_amount.checked_sub(new_swap_destination_amount)?;

        Some(SwapResult {
            new_swap_source_amount,
            new_swap_destination_amount,
            source_amount_swapped: source_amount,
            destination_amount_swapped,
        })
    }
}

pub trait SwapCurve {
    fn swap(
        &self,
        source_amount: u64,
        swap_source_amount: u64,
        swap_destination_amount: u64,
        trade_direction: TradeDirection,
    ) -> Option<SwapResult>;
}

/// Encodes all results of swapping
#[derive(Debug, PartialEq)]
pub struct SwapResult {
    /// New amount of source token
    pub new_swap_source_amount: u128,
    /// New amount of destination token
    pub new_swap_destination_amount: u128,
    /// Amount of source token swapped (includes fees)
    pub source_amount_swapped: u128,
    /// Amount of destination token swapped
    pub destination_amount_swapped: u128,
}

pub enum TradeDirection {
    AtoB,
    BtoA,
}
