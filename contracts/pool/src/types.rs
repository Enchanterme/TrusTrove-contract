//! Pool contract data types.
//!
//! # Basis points (bps)
//!
//! Ratios in this module use basis-point (bps) scaling, where `10_000` bps
//! equals `100%` and `1` bp equals `0.01%`. For example, a
//! `utilization_rate_bps` value of `7500` represents a utilization of `75%`.

use soroban_sdk::{contracttype, Address, BytesN};

/// Aggregate, pool-wide accounting snapshot returned by `get_pool_stats`.
#[contracttype]
#[derive(Clone, Debug)]
pub struct PoolStats {
    /// Total USDC principal currently held by the pool, in stroops
    /// (1 USDC = 10_000_000 stroops). Grows when LPs deposit or when yield
    /// is distributed back into the pool, and shrinks on LP withdrawals and
    /// invoice funding.
    pub total_deposits: u128,
    /// Total USDC (in stroops) currently deployed to fund outstanding
    /// invoices. Increases on `fund_invoice` and decreases on repayment.
    pub total_funded: u128,
    /// USDC (in stroops) available for new invoice funding or LP
    /// withdrawals. Equal to `total_deposits - total_funded`.
    pub available_liquidity: u128,
    /// Current pool utilization, expressed in basis points
    /// (`0` = 0%, `10_000` = 100%). Computed as
    /// `total_funded * 10_000 / total_deposits`.
    pub utilization_rate_bps: u32,
    /// Cumulative USDC (in stroops) of yield that has been distributed to
    /// the pool from repaid invoices over the pool's lifetime.
    pub total_yield_distributed: u128,
    /// Cumulative USDC principal (in stroops) written off when funded
    /// invoices default. This is lifetime accounting and is not reduced by
    /// later deposits.
    pub total_loss_realised: u128,
    /// Number of invoices currently funded and awaiting repayment.
    pub active_invoice_count: u32,
    /// Total supply of LP shares outstanding. Individual LP ownership of
    /// the pool is `lp_shares / total_shares`.
    pub total_shares: u128,
    /// Maximum utilization the pool will allow before rejecting new
    /// invoice funding, in basis points (see module docs).
    pub max_utilization_bps: u32,
}

/// Per-LP position snapshot returned by `get_lp_position`.
#[contracttype]
#[derive(Clone, Debug)]
pub struct LPPosition {
    /// LP share balance owned by this liquidity provider. Ownership of the
    /// pool is `shares / PoolStats::total_shares`.
    pub shares: u128,
    /// Current redemption value of `shares` in USDC stroops, computed as
    /// `shares * total_deposits / total_shares` at query time. Includes
    /// principal plus the LP's proportional share of undistributed yield.
    pub usdc_value: u128,
    /// Cumulative USDC yield (in stroops) realised by this LP across all
    /// prior withdrawals. Only updated on withdraw, when the redeemed
    /// amount exceeds the LP's tracked principal portion; unrealised yield
    /// still sitting in `usdc_value` is not counted here.
    pub yield_earned: u128,
    /// Number of successful deposits this LP has made into the pool.
    pub deposit_count: u32,
}

/// Snapshot of a single pool-to-pool migration, returned by `get_migration_record`.
#[contracttype]
#[derive(Clone, Debug)]
pub struct MigrationRecord {
    /// LP address that initiated the migration.
    pub lp: Address,
    /// Source pool contract address.
    pub source_pool: Address,
    /// Target pool contract address.
    pub target_pool: Address,
    /// Shares burned in the source pool.
    pub shares_burned: u128,
    /// USDC withdrawn from the source pool (stroops).
    pub usdc_withdrawn: u128,
    /// Shares minted in the target pool.
    pub shares_minted: u128,
    /// Ledger timestamp of the migration.
    pub timestamp: u64,
}

/// Pre-migration estimate returned by `estimate_migration`.
#[contracttype]
#[derive(Clone, Debug)]
pub struct MigrationEstimate {
    /// USDC the LP would receive from the source pool (stroops).
    pub usdc_out: u128,
    /// Shares the LP would receive in the target pool.
    pub shares_in: u128,
    /// Share price in the source pool (total_deposits / total_shares, scaled by 1e7).
    pub source_share_price: u128,
    /// Share price in the target pool (scaled by 1e7).
    pub target_share_price: u128,
    /// Slippage in basis points: `|source_price - target_price| / source_price * 10_000`.
    pub slippage_bps: u32,
}

#[contracttype]
pub enum DataKey {
    Admin,
    InvoiceContract,
    EscrowContract,
    UsdcAsset,
    TotalShares,
    TotalDeposits,
    TotalFunded,
    TotalYieldDistributed,
    TotalLossRealised,
    ActiveInvoiceCount,
    LPShares(Address),
    LPDepositCount(Address),
    LPYieldEarned(Address),
    LPInitialDeposit(Address),
    FundedInvoice(BytesN<32>),
    MaxUtilizationBps,
    MigrationCount,
    MigrationRecord(u64),
}
