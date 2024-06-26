use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

use crate::Discriminator;

/// Instructions

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum DriftV2Instruction {
    InitializeUser(InitializeUser),
    InitializeUserStats,
    InitializeReferrerName(InitializeReferrerName),
    Deposit(Deposit),
    Withdraw(Withdraw),
    TransferDeposit(TransferDeposit),
    PlacePerpOrder(PlacePerpOrder),
    CancelOrder(CancelOrder),
    CancelOrderByUserId(CancelOrderByUserId),
    CancelOrders(CancelOrders),
    CancelOrdersByIds(CancelOrdersByIds),
    ModifyOrder(ModifyOrder),
    ModifyOrderByUserId {
        user_order_id: u8,
        modify_order_params: ModifyOrderParams,
    },
    PlaceAndTakePerpOrder(PlaceAndTakePerpOrder),
    PlaceAndMakePerpOrder(PlaceAndMakePerpOrder),
    PlaceSpotOrder(PlaceSpotOrder),
    PlaceAndTakeSpotOrder {
        params: OrderParams,
        fulfillment_type: Option<SpotFulfillmentType>,
        maker_order_id: Option<u32>,
    },
    PlaceAndMakeSpotOrder {
        params: OrderParams,
        taker_order_id: u32,
        fulfillment_type: Option<SpotFulfillmentType>,
    },
    PlaceOrders(PlaceOrders),
    BeginSwap(BeginSwap),
    EndSwap(EndSwap),
    AddPerpLpShares(AddPerpLpShares),
    RemovePerpLpShares(RemovePerpLpShares),
    RemovePerpLpSharesInExpiringMarket {
        shares_to_burn: u64,
        market_index: u16,
    },
    UpdateUserName {
        sub_account_id: u16,
        name: [u8; 32],
    },
    UpdateUserCustomMarginRatio(UpdateUserCustomMarginRatio),
    UpdateUserMarginTradingEnabled(UpdateUserMarginTradingEnabled),
    UpdateUserDelegate {
        sub_account_id: u16,
        delegate: Pubkey,
    },
    UpdateUserReduceOnly {
        sub_account_id: u16,
        reduce_only: bool,
    },
    UpdateUserAdvancedLp {
        sub_account_id: u16,
        advanced_lp: bool,
    },
    DeleteUser,
    ReclaimRent,
    FillPerpOrder(FillPerpOrder),
    RevertFill,
    FillSpotOrder(FillSpotOrder),
    TriggerOrder(TriggerOrder),
    ForceCancelOrders,
    UpdateUserIdle,
    UpdateUserOpenOrdersCount,
    AdminDisableUpdatePerpBidAskTwap {
        disable: bool,
    },
    SettlePnl(SettlePnl),
    SettleMultiplePnls(SettleMultiplePnls),
    SettleFundingPayment,
    SettleLp(SettleLp),
    SettleExpiredMarket {
        market_index: u16,
    },
    LiquidatePerp(LiquidatePerp),
    LiquidateSpot {
        asset_market_index: u16,
        liability_market_index: u16,
        liquidator_max_liability_transfer: u128,
        limit_price: Option<u64>,
    },
    LiquidateBorrowForPerpPnl {
        perp_market_index: u16,
        spot_market_index: u16,
        liquidator_max_liability_transfer: u128,
        limit_price: Option<u64>,
    },
    LiquidatePerpPnlForDeposit {
        perp_market_index: u16,
        spot_market_index: u16,
        liquidator_max_pnl_transfer: u128,
        limit_price: Option<u64>,
    },
    ResolvePerpPnlDeficit {
        spot_market_index: u16,
        perp_market_index: u16,
    },
    ResolvePerpBankruptcy {
        quote_spot_market_index: u16,
        market_index: u16,
    },
    ResolveSpotBankruptcy {
        market_index: u16,
    },
    SettleRevenueToInsuranceFund(SettleRevenueToInsuranceFund),
    UpdateFundingRate(UpdateFundingRate),
    UpdatePrelaunchOracle,
    UpdatePerpBidAskTwap,
    UpdateSpotMarketCumulativeInterest,
    UpdateAmms(UpdateAmms),
    UpdateSpotMarketExpiry {
        expiry_ts: i64,
    },
    UpdateUserQuoteAssetInsuranceStake,
    InitializeInsuranceFundStake(InitializeInsuranceFundStake),
    AddInsuranceFundStake(AddInsuranceFundStake),
    RequestRemoveInsuranceFundStake(RequestRemoveInsuranceFundStake),
    CancelRequestRemoveInsuranceFundStake(CancelRequestRemoveInsuranceFundStake),
    RemoveInsuranceFundStake(RemoveInsuranceFundStake),
    TransferProtocolIfShares {
        market_index: u16,
        shares: u128,
    },
    Initialize,
    InitializeSpotMarket {
        optimal_utilization: u32,
        optimal_borrow_rate: u32,
        max_borrow_rate: u32,
        oracle_source: OracleSource,
        initial_asset_weight: u32,
        maintenance_asset_weight: u32,
        initial_liability_weight: u32,
        maintenance_liability_weight: u32,
        imf_factor: u32,
        liquidator_fee: u32,
        if_liquidation_fee: u32,
        active_status: bool,
        asset_tier: AssetTier,
        scale_initial_asset_weight_start: u64,
        withdraw_guard_threshold: u64,
        order_tick_size: u64,
        order_step_size: u64,
        if_total_factor: u32,
        name: [u8; 32],
    },
    DeleteInitializedSpotMarket {
        market_index: u16,
    },
    InitializeSerumFulfillmentConfig {
        market_index: u16,
    },
    UpdateSerumFulfillmentConfigStatus {
        status: SpotFulfillmentConfigStatus,
    },
    InitializePhoenixFulfillmentConfig {
        market_index: u16,
    },
    PhoenixFulfillmentConfigStatus {
        status: SpotFulfillmentConfigStatus,
    },
    UpdateSerumVault,
    InitializePerpMarket {
        market_index: u16,
        amm_base_asset_reserve: u128,
        amm_quote_asset_reserve: u128,
        amm_periodicity: i64,
        amm_peg_multiplier: u128,
        oracle_source: OracleSource,
        contract_tier: ContractTier,
        margin_ratio_initial: u32,
        margin_ratio_maintenance: u32,
        liquidator_fee: u32,
        if_liquidation_fee: u32,
        imf_factor: u32,
        active_status: bool,
        base_spread: u32,
        max_spread: u32,
        max_open_interest: u128,
        max_revenue_withdraw_per_period: u64,
        quote_max_insurance: u64,
        order_step_size: u64,
        order_tick_size: u64,
        min_order_size: u64,
        concentration_coef_scale: u128,
        curve_update_intensity: u8,
        amm_jit_intensity: u8,
        name: [u8; 32],
    },
    DeleteInitializedPerpMarket {
        market_index: u16,
    },
    MoveAmmPrice {
        base_asset_reserve: u128,
        quote_asset_reserve: u128,
        sqrt_k: u128,
    },
    RecenterPerpMarketAmm {
        peg_multiplier: u128,
        sqrt_k: u128,
    },
    UpdatePerpMarketAmmSummaryStats {
        params: UpdatePerpMarketSummaryStatsParams,
    },
    UpdatePerpMarketExpiry {
        expiry_ts: i64,
    },
    SettleExpiredMarketPoolsToRevenuePool,
    DepositIntoPerpMarketFeePool {
        amount: u64,
    },
    DepositIntoSpotMarketRevenuePool(DepositIntoSpotMarketRevenuePool),
    RepegAmmCurve {
        new_peg_candidate: u128,
    },
    UpdatePerpMarketAmmOracleTwap,
    ResetPerpMarketAmmOracleTwap,
    UpdateK {
        sqrt_k: u128,
    },
    UpdatePerpMarketMarginRatio {
        margin_ratio_initial: u32,
        margin_ratio_maintenance: u32,
    },
    UpdatePerpMarketFundingPeriod {
        funding_period: i64,
    },
    UpdatePerpMarketMaxImbalances {
        unrealized_max_imbalance: u64,
        max_revenue_withdraw_per_period: u64,
        quote_max_insurance: u64,
    },
    UpdatePerpMarketLiquidationFee {
        liquidator_fee: u32,
        if_liquidation_fee: u32,
    },
    UpdateInsuranceFundUnstakingPeriod {
        insurance_fund_unstaking_period: i64,
    },
    UpdateSpotMarketLiquidationFee {
        liquidator_fee: u32,
        if_liquidation_fee: u32,
    },
    UpdateWithdrawGuardThreshold {
        withdraw_guard_threshold: u64,
    },
    UpdateSpotMarketIfFactor {
        spot_market_index: u16,
        user_if_factor: u32,
        total_if_factor: u32,
    },
    UpdateSpotMarketRevenueSettlePeriod {
        revenue_settle_period: i64,
    },
    UpdateSpotMarketStatus {
        status: MarketStatus,
    },
    UpdateSpotMarketPausedOperations {
        paused_operations: u8,
    },
    UpdateSpotMarketAssetTier {
        asset_tier: AssetTier,
    },
    UpdateSpotMarketMarginWeights {
        initial_asset_weight: u32,
        maintenance_asset_weight: u32,
        initial_liability_weight: u32,
        maintenance_liability_weight: u32,
        imf_factor: u32,
    },
    UpdateSpotMarketBorrowRate {
        optimal_utilization: u32,
        optimal_borrow_rate: u32,
        max_borrow_rate: u32,
    },
    UpdateSpotMarketMaxTokenDeposits {
        max_token_deposits: u64,
    },
    UpdateSpotMarketScaleInitialAssetWeightStart {
        scale_initial_asset_weight_start: u64,
    },
    UpdateSpotMarketOracle {
        oracle: Pubkey,
        oracle_source: OracleSource,
    },
    UpdateSpotMarketStepSizeAndTickSize {
        step_size: u64,
        tick_size: u64,
    },
    UpdateSpotMarketMinOrderSize {
        order_size: u64,
    },
    UpdateSpotMarketOrdersEnabled {
        orders_enabled: bool,
    },
    UpdateSpotMarketIfPausedOperations {
        paused_operations: u8,
    },
    UpdateSpotMarketName {
        name: [u8; 32],
    },
    UpdatePerpMarketStatus {
        status: MarketStatus,
    },
    UpdatePerpMarketPausedOperations {
        paused_operations: u8,
    },
    UpdatePerpMarketContractTier {
        contract_tier: ContractTier,
    },
    UpdatePerpMarketImfFactor {
        imf_factor: u32,
        unrealized_pnl_imf_factor: u32,
    },
    UpdatePerpMarketUnrealizedAssetWeight {
        unrealized_initial_asset_weight: u32,
        unrealized_maintenance_asset_weight: u32,
    },
    UpdatePerpMarketConcentrationCoef {
        concentration_scale: u128,
    },
    UpdatePerpMarketCurveUpdateIntensity {
        curve_update_intensity: u8,
    },
    UpdatePerpMarketTargetBaseAssetAmountPerLp {
        target_base_asset_amount_per_lp: i32,
    },
    UpdatePerpMarketPerLpBase {
        per_lp_base: i8,
    },
    UpdateLpCooldownTime {
        lp_cooldown_time: u64,
    },
    UpdatePerpFeeStructure {
        fee_structure: FeeStructure,
    },
    UpdateSpotFeeStructure {
        fee_structure: FeeStructure,
    },
    UpdateInitialPctToLiquidate {
        initial_pct_to_liquidate: u16,
    },
    UpdateLiquidationDuration {
        liquidation_duration: u8,
    },
    UpdateLiquidationMarginBufferRatio {
        liquidation_margin_buffer_ratio: u32,
    },
    UpdateOracleGuardRails {
        oracle_guard_rails: OracleGuardRails,
    },
    UpdateStateSettlementDuration {
        settlement_duration: u16,
    },
    UpdateStateMaxNumberOfSubAccounts {
        max_number_of_sub_accounts: u16,
    },
    UpdateStateMaxInitializeUserFee {
        max_initialize_user_fee: u16,
    },
    UpdatePerpMarketOracle {
        oracle: Pubkey,
        oracle_source: OracleSource,
    },
    UpdatePerpMarketBaseSpread {
        base_spread: u32,
    },
    UpdateAmmJitIntensity {
        amm_jit_intensity: u8,
    },
    UpdatePerpMarketMaxSpread {
        max_spread: u32,
    },
    UpdatePerpMarketStepSizeAndTickSize {
        step_size: u64,
        tick_size: u64,
    },
    UpdatePerpMarketName {
        name: [u8; 32],
    },
    UpdatePerpMarketMinOrderSize {
        order_size: u64,
    },
    UpdatePerpMarketMaxSlippageRatio {
        max_slippage_ratio: u16,
    },
    UpdatePerpMarketMaxFillReserveFraction {
        max_fill_reserve_fraction: u16,
    },
    UpdatePerpMarketMaxOpenInterest {
        max_open_interest: u128,
    },
    UpdatePerpMarketNumberOfUsers {
        number_of_users: Option<u32>,
        number_of_users_with_base: Option<u32>,
    },
    UpdatePerpMarketFeeAdjustment {
        fee_adjustment: i16,
    },
    UpdateSpotMarketFeeAdjustment {
        fee_adjustment: i16,
    },
    UpdateAdmin {
        admin: Pubkey,
    },
    UpdateWhitelistMint {
        whitelist_mint: Pubkey,
    },
    UpdateDiscountMint {
        discount_mint: Pubkey,
    },
    UpdateExchangeStatus {
        exchange_status: u8,
    },
    UpdatePerpAuctionDuration {
        min_perp_auction_duration: u8,
    },
    UpdateSpotAuctionDuration {
        default_spot_auction_duration: u8,
    },
    InitializeProtocolIfSharesTransferConfig,
    UpdateProtocolIfSharesTransferConfig {
        whitelisted_signers: Option<[Pubkey; 4]>,
        max_transfer_per_epoch: Option<u128>,
    },
    InitializePrelaunchOracle {
        params: PrelaunchOracleParams,
    },
    UpdatePrelaunchOracleParams {
        params: PrelaunchOracleParams,
    },
    DeletePrelaunchOracle {
        perp_market_index: u16,
    },
}

impl DriftV2Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < 8 {
            return Err(ProgramError::InvalidInstructionData);
        }

        let (discriminator, rest) = input.split_at(8);
        if let Ok(discriminator) = discriminator.try_into() {
            return Ok(match discriminator {
                InitializeUser::DISCRIMINATOR => Self::InitializeUser(borsh::from_slice(rest)?),
                InitializeUserStats::DISCRIMINATOR => Self::InitializeUserStats,
                InitializeReferrerName::DISCRIMINATOR => {
                    Self::InitializeReferrerName(borsh::from_slice(rest)?)
                }
                Deposit::DISCRIMINATOR => Self::Deposit(borsh::from_slice(rest)?),
                Withdraw::DISCRIMINATOR => Self::Withdraw(borsh::from_slice(rest)?),
                TransferDeposit::DISCRIMINATOR => Self::TransferDeposit(borsh::from_slice(rest)?),
                PlacePerpOrder::DISCRIMINATOR => Self::PlacePerpOrder(borsh::from_slice(rest)?),
                CancelOrder::DISCRIMINATOR => Self::CancelOrder(borsh::from_slice(rest)?),
                CancelOrderByUserId::DISCRIMINATOR => {
                    Self::CancelOrderByUserId(borsh::from_slice(rest)?)
                }
                CancelOrders::DISCRIMINATOR => Self::CancelOrders(borsh::from_slice(rest)?),
                CancelOrdersByIds::DISCRIMINATOR => {
                    Self::CancelOrdersByIds(borsh::from_slice(rest)?)
                }
                ModifyOrder::DISCRIMINATOR => Self::ModifyOrder(borsh::from_slice(rest)?),
                PlaceAndTakePerpOrder::DISCRIMINATOR => {
                    Self::PlaceAndTakePerpOrder(borsh::from_slice(rest)?)
                }
                PlaceAndMakePerpOrder::DISCRIMINATOR => {
                    Self::PlaceAndMakePerpOrder(borsh::from_slice(rest)?)
                }
                PlaceSpotOrder::DISCRIMINATOR => Self::PlaceSpotOrder(borsh::from_slice(rest)?),
                PlaceOrders::DISCRIMINATOR => Self::PlaceOrders(borsh::from_slice(rest)?),
                BeginSwap::DISCRIMINATOR => Self::BeginSwap(borsh::from_slice(rest)?),
                EndSwap::DISCRIMINATOR => Self::EndSwap(borsh::from_slice(rest)?),
                AddPerpLpShares::DISCRIMINATOR => Self::AddPerpLpShares(borsh::from_slice(rest)?),
                RemovePerpLpShares::DISCRIMINATOR => {
                    Self::RemovePerpLpShares(borsh::from_slice(rest)?)
                }
                UpdateUserCustomMarginRatio::DISCRIMINATOR => {
                    Self::UpdateUserCustomMarginRatio(borsh::from_slice(rest)?)
                }
                UpdateUserMarginTradingEnabled::DISCRIMINATOR => {
                    Self::UpdateUserMarginTradingEnabled(borsh::from_slice(rest)?)
                }
                DeleteUser::DISCRIMINATOR => Self::DeleteUser,
                ReclaimRent::DISCRIMINATOR => Self::ReclaimRent,
                FillPerpOrder::DISCRIMINATOR => Self::FillPerpOrder(borsh::from_slice(rest)?),
                RevertFill::DISCRIMINATOR => Self::RevertFill,
                FillSpotOrder::DISCRIMINATOR => Self::FillSpotOrder(borsh::from_slice(rest)?),
                TriggerOrder::DISCRIMINATOR => Self::TriggerOrder(borsh::from_slice(rest)?),
                UpdateUserIdle::DISCRIMINATOR => Self::UpdateUserIdle,
                SettlePnl::DISCRIMINATOR => Self::SettlePnl(borsh::from_slice(rest)?),
                SettleMultiplePnls::DISCRIMINATOR => {
                    Self::SettleMultiplePnls(borsh::from_slice(rest)?)
                }
                SettleFundingPayment::DISCRIMINATOR => Self::SettleFundingPayment,
                SettleLp::DISCRIMINATOR => Self::SettleLp(borsh::from_slice(rest)?),
                LiquidatePerp::DISCRIMINATOR => Self::LiquidatePerp(borsh::from_slice(rest)?),
                SettleRevenueToInsuranceFund::DISCRIMINATOR => {
                    Self::SettleRevenueToInsuranceFund(borsh::from_slice(rest)?)
                }
                UpdateFundingRate::DISCRIMINATOR => {
                    Self::UpdateFundingRate(borsh::from_slice(rest)?)
                }
                UpdatePrelaunchOracle::DISCRIMINATOR => Self::UpdatePrelaunchOracle,
                UpdatePerpBidAskTwap::DISCRIMINATOR => Self::UpdatePerpBidAskTwap,
                UpdateAmms::DISCRIMINATOR => Self::UpdateAmms(borsh::from_slice(rest)?),
                InitializeInsuranceFundStake::DISCRIMINATOR => {
                    Self::InitializeInsuranceFundStake(borsh::from_slice(rest)?)
                }
                AddInsuranceFundStake::DISCRIMINATOR => {
                    Self::AddInsuranceFundStake(borsh::from_slice(rest)?)
                }
                RequestRemoveInsuranceFundStake::DISCRIMINATOR => {
                    Self::RequestRemoveInsuranceFundStake(borsh::from_slice(rest)?)
                }
                CancelRequestRemoveInsuranceFundStake::DISCRIMINATOR => {
                    Self::CancelRequestRemoveInsuranceFundStake(borsh::from_slice(rest)?)
                }
                RemoveInsuranceFundStake::DISCRIMINATOR => {
                    Self::RemoveInsuranceFundStake(borsh::from_slice(rest)?)
                }
                DepositIntoSpotMarketRevenuePool::DISCRIMINATOR => {
                    Self::DepositIntoSpotMarketRevenuePool(borsh::from_slice(rest)?)
                }
                _ => return Err(ProgramError::InvalidInstructionData),
            });
        }

        Err(ProgramError::InvalidInstructionData)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializeUser {
    pub sub_account_id: u16,
    pub name: [u8; 32],
}

impl Discriminator for InitializeUser {
    const DISCRIMINATOR: [u8; 8] = [111, 17, 185, 250, 60, 122, 38, 254];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializeUserStats {}

impl Discriminator for InitializeUserStats {
    const DISCRIMINATOR: [u8; 8] = [254, 243, 72, 98, 251, 130, 168, 213];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializeReferrerName {
    pub name: [u8; 32],
}

impl Discriminator for InitializeReferrerName {
    const DISCRIMINATOR: [u8; 8] = [235, 126, 231, 10, 42, 164, 26, 61];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct Deposit {
    pub market_index: u16,
    pub amount: u64,
    pub reduce_only: bool,
}

impl Discriminator for Deposit {
    const DISCRIMINATOR: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct Withdraw {
    pub market_index: u16,
    pub amount: u64,
    pub reduce_only: bool,
}

impl Discriminator for Withdraw {
    const DISCRIMINATOR: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct TransferDeposit {
    pub market_index: u16,
    pub amount: u64,
}

impl Discriminator for TransferDeposit {
    const DISCRIMINATOR: [u8; 8] = [20, 20, 147, 223, 41, 63, 204, 111];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PlacePerpOrder {
    pub params: OrderParams,
}

impl Discriminator for PlacePerpOrder {
    const DISCRIMINATOR: [u8; 8] = [69, 161, 93, 202, 120, 126, 76, 185];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct CancelOrder {
    pub order_id: Option<u32>,
}

impl Discriminator for CancelOrder {
    const DISCRIMINATOR: [u8; 8] = [95, 129, 237, 240, 8, 49, 223, 132];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct CancelOrderByUserId {
    pub user_order_id: u8,
}

impl Discriminator for CancelOrderByUserId {
    const DISCRIMINATOR: [u8; 8] = [107, 211, 250, 133, 18, 37, 57, 100];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct CancelOrders {
    pub market_type: Option<MarketType>,
    pub market_index: Option<u16>,
    pub direction: Option<PositionDirection>,
}

impl Discriminator for CancelOrders {
    const DISCRIMINATOR: [u8; 8] = [238, 225, 95, 158, 227, 103, 8, 194];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct CancelOrdersByIds {
    pub order_ids: Vec<u32>,
}

impl Discriminator for CancelOrdersByIds {
    const DISCRIMINATOR: [u8; 8] = [134, 19, 144, 165, 94, 240, 210, 94];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ModifyOrder {
    pub order_id: Option<u32>,
    pub modify_order_params: ModifyOrderParams,
}

impl Discriminator for ModifyOrder {
    const DISCRIMINATOR: [u8; 8] = [47, 124, 117, 255, 201, 197, 130, 94];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PlaceAndTakePerpOrder {
    pub params: OrderParams,
    pub maker_order_id: Option<u32>,
}

impl Discriminator for PlaceAndTakePerpOrder {
    const DISCRIMINATOR: [u8; 8] = [213, 51, 1, 187, 108, 220, 230, 224];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PlaceAndMakePerpOrder {
    params: OrderParams,
    taker_order_id: u32,
}

impl Discriminator for PlaceAndMakePerpOrder {
    const DISCRIMINATOR: [u8; 8] = [149, 117, 11, 237, 47, 95, 89, 237];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PlaceSpotOrder {
    params: OrderParams,
}

impl Discriminator for PlaceSpotOrder {
    const DISCRIMINATOR: [u8; 8] = [45, 79, 81, 160, 248, 90, 91, 220];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PlaceOrders {
    params: Vec<OrderParams>,
}

impl Discriminator for PlaceOrders {
    const DISCRIMINATOR: [u8; 8] = [60, 63, 50, 123, 12, 197, 60, 190];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct BeginSwap {
    in_market_index: u16,
    out_market_index: u16,
    amount_in: u64,
}

impl Discriminator for BeginSwap {
    const DISCRIMINATOR: [u8; 8] = [174, 109, 228, 1, 242, 105, 232, 105];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct EndSwap {
    in_market_index: u16,
    out_market_index: u16,
    limit_price: Option<u64>,
    reduce_only: Option<SwapReduceOnly>,
}

impl Discriminator for EndSwap {
    const DISCRIMINATOR: [u8; 8] = [177, 184, 27, 193, 34, 13, 210, 145];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct AddPerpLpShares {
    n_shares: u64,
    market_index: u16,
}

impl Discriminator for AddPerpLpShares {
    const DISCRIMINATOR: [u8; 8] = [56, 209, 56, 197, 119, 254, 188, 117];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct RemovePerpLpShares {
    shares_to_burn: u64,
    market_index: u16,
}

impl Discriminator for RemovePerpLpShares {
    const DISCRIMINATOR: [u8; 8] = [213, 89, 217, 18, 160, 55, 53, 141];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateUserCustomMarginRatio {
    sub_account_id: u16,
    margin_ratio: u32,
}

impl Discriminator for UpdateUserCustomMarginRatio {
    const DISCRIMINATOR: [u8; 8] = [21, 221, 140, 187, 32, 129, 11, 123];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateUserMarginTradingEnabled {
    sub_account_id: u16,
    margin_trading_enabled: bool,
}

impl Discriminator for UpdateUserMarginTradingEnabled {
    const DISCRIMINATOR: [u8; 8] = [194, 92, 204, 223, 246, 188, 31, 203];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct DeleteUser {}

impl Discriminator for DeleteUser {
    const DISCRIMINATOR: [u8; 8] = [186, 85, 17, 249, 219, 231, 98, 251];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ReclaimRent {}

impl Discriminator for ReclaimRent {
    const DISCRIMINATOR: [u8; 8] = [218, 200, 19, 197, 227, 89, 192, 22];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct FillPerpOrder {
    order_id: Option<u32>,
    maker_order_id: Option<u32>,
}

impl Discriminator for FillPerpOrder {
    const DISCRIMINATOR: [u8; 8] = [13, 188, 248, 103, 134, 217, 106, 240];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct RevertFill {}

impl Discriminator for RevertFill {
    const DISCRIMINATOR: [u8; 8] = [236, 238, 176, 69, 239, 10, 181, 193];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct FillSpotOrder {
    order_id: Option<u32>,
    fulfillment_type: Option<SpotFulfillmentType>,
    maker_order_id: Option<u32>,
}

impl Discriminator for FillSpotOrder {
    const DISCRIMINATOR: [u8; 8] = [212, 206, 130, 173, 21, 34, 199, 40];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct TriggerOrder {
    order_id: u32,
}

impl Discriminator for TriggerOrder {
    const DISCRIMINATOR: [u8; 8] = [63, 112, 51, 233, 232, 47, 240, 199];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateUserIdle {}

impl Discriminator for UpdateUserIdle {
    const DISCRIMINATOR: [u8; 8] = [253, 133, 67, 22, 103, 161, 20, 100];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SettlePnl {
    pub market_index: u16,
}

impl Discriminator for SettlePnl {
    const DISCRIMINATOR: [u8; 8] = [43, 61, 234, 45, 15, 95, 152, 153];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SettleMultiplePnls {
    market_indexes: Vec<u16>,
    mode: SettlePnlMode,
}

impl Discriminator for SettleMultiplePnls {
    const DISCRIMINATOR: [u8; 8] = [127, 66, 117, 57, 40, 50, 152, 127];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SettleFundingPayment {}

impl Discriminator for SettleFundingPayment {
    const DISCRIMINATOR: [u8; 8] = [222, 90, 202, 94, 28, 45, 115, 183];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SettleLp {
    market_index: u16,
}

impl Discriminator for SettleLp {
    const DISCRIMINATOR: [u8; 8] = [155, 231, 116, 113, 97, 229, 139, 141];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct LiquidatePerp {
    market_index: u16,
    liquidator_max_base_asset_amount: u64,
    limit_price: Option<u64>,
}

impl Discriminator for LiquidatePerp {
    const DISCRIMINATOR: [u8; 8] = [75, 35, 119, 247, 191, 18, 139, 2];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SettleRevenueToInsuranceFund {
    spot_market_index: u16,
}

impl Discriminator for SettleRevenueToInsuranceFund {
    const DISCRIMINATOR: [u8; 8] = [200, 120, 93, 136, 69, 38, 199, 159];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateFundingRate {
    market_index: u16,
}

impl Discriminator for UpdateFundingRate {
    const DISCRIMINATOR: [u8; 8] = [201, 178, 116, 212, 166, 144, 72, 238];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePrelaunchOracle {}

impl Discriminator for UpdatePrelaunchOracle {
    const DISCRIMINATOR: [u8; 8] = [220, 132, 27, 27, 233, 220, 61, 219];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpBidAskTwap {}

impl Discriminator for UpdatePerpBidAskTwap {
    const DISCRIMINATOR: [u8; 8] = [247, 23, 255, 65, 212, 90, 221, 194];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateAmms {
    market_indexes: [u16; 5],
}

impl Discriminator for UpdateAmms {
    const DISCRIMINATOR: [u8; 8] = [201, 106, 217, 253, 4, 175, 228, 97];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializeInsuranceFundStake {
    market_index: u16,
}

impl Discriminator for InitializeInsuranceFundStake {
    const DISCRIMINATOR: [u8; 8] = [187, 179, 243, 70, 248, 90, 92, 147];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct AddInsuranceFundStake {
    market_index: u16,
    amount: u64,
}

impl Discriminator for AddInsuranceFundStake {
    const DISCRIMINATOR: [u8; 8] = [251, 144, 115, 11, 222, 47, 62, 236];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct RequestRemoveInsuranceFundStake {
    market_index: u16,
    amount: u64,
}

impl Discriminator for RequestRemoveInsuranceFundStake {
    const DISCRIMINATOR: [u8; 8] = [142, 70, 204, 92, 73, 106, 180, 52];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct CancelRequestRemoveInsuranceFundStake {
    market_index: u16,
}

impl Discriminator for CancelRequestRemoveInsuranceFundStake {
    const DISCRIMINATOR: [u8; 8] = [97, 235, 78, 62, 212, 42, 241, 127];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct RemoveInsuranceFundStake {
    market_index: u16,
}

impl Discriminator for RemoveInsuranceFundStake {
    const DISCRIMINATOR: [u8; 8] = [128, 166, 142, 9, 254, 187, 143, 174];
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct DepositIntoSpotMarketRevenuePool {
    amount: u64,
}

impl Discriminator for DepositIntoSpotMarketRevenuePool {
    const DISCRIMINATOR: [u8; 8] = [92, 40, 151, 42, 122, 254, 139, 246];
}

/// Instruction accounts

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializeUserAccounts {
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub state: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializeUserStatsAccounts {
    pub user_stats: Pubkey,
    pub state: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializeReferrerNameAccounts {
    pub referrer_name: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct DepositAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct WithdrawAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct TransferDepositAccounts {
    pub from_user: Pubkey,
    pub to_user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub state: Pubkey,
    pub spot_market_vault: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PlacePerpOrderAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct CancelOrderAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct CancelOrderByUserIdAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct CancelOrdersAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct CancelOrdersByIdsAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ModifyOrderAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ModifyOrderByUserIdAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PlaceAndTakePerpOrderAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PlaceAndMakePerpOrderAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub taker: Pubkey,
    pub taker_stats: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PlaceSpotOrderAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PlaceAndTakeSpotOrderAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PlaceAndMakeSpotOrderAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub taker: Pubkey,
    pub taker_stats: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PlaceOrdersAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct BeginSwapAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub out_spot_market_vault: Pubkey,
    pub in_spot_market_vault: Pubkey,
    pub out_token_account: Pubkey,
    pub in_token_account: Pubkey,
    pub token_program: Pubkey,
    pub drift_signer: Pubkey,
    /// Instructions Sysvar for instruction introspection
    pub instructions: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct EndSwapAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub out_spot_market_vault: Pubkey,
    pub in_spot_market_vault: Pubkey,
    pub out_token_account: Pubkey,
    pub in_token_account: Pubkey,
    pub token_program: Pubkey,
    pub drift_signer: Pubkey,
    /// Instructions Sysvar for instruction introspection
    pub instructions: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct AddPerpLpSharesAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct RemovePerpLpSharesAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct RemovePerpLpSharesInExpiringMarketAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateUserNameAccounts {
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateUserCustomMarginRatioAccounts {
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateUserMarginTradingEnabledAccounts {
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateUserDelegateAccounts {
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateUserReduceOnlyAccounts {
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateUserAdvancedLpAccounts {
    pub user: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct DeleteUserAccounts {
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub state: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ReclaimRentAccounts {
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub state: Pubkey,
    pub authority: Pubkey,
    pub rent: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct FillPerpOrderAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub filler_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct RevertFillAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub filler_stats: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct FillSpotOrderAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub filler_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct TriggerOrderAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub user: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ForceCancelOrdersAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub user: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateUserIdleAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub user: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateUserOpenOrdersCountAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub user: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct AdminDisableUpdatePerpBidAskTwapAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub user_stats: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SettlePnlAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SettleMultiplePnlsAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SettleFundingPaymentAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SettleLpAccounts {
    pub state: Pubkey,
    pub user: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SettleExpiredMarketAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct LiquidatePerpAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct LiquidateSpotAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct LiquidateBorrowForPerpPnlAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct LiquidatePerpPnlForDepositAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ResolvePerpPnlDeficitAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub token_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ResolvePerpBankruptcyAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub token_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ResolveSpotBankruptcyAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub token_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SettleRevenueToInsuranceFundAccounts {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub spot_market_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub token_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateFundingRateAccounts {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePrelaunchOracleAccounts {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpBidAskTwapAccounts {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub keeper_stats: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketCumulativeInterestAccounts {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub oracle: Pubkey,
    pub spot_market_vault: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateAmmsAccounts {
    pub state: Pubkey,
    pub authority: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketExpiryAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateUserQuoteAssetInsuranceStakeAccounts {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub insurance_fund_vault: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializeInsuranceFundStakeAccounts {
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub state: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct AddInsuranceFundStakeAccounts {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct RequestRemoveInsuranceFundStakeAccounts {
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub insurance_fund_vault: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct CancelRequestRemoveInsuranceFundStakeAccounts {
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub insurance_fund_vault: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct RemoveInsuranceFundStakeAccounts {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct TransferProtocolIfSharesAccounts {
    pub signer: Pubkey,
    pub transfer_config: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub insurance_fund_vault: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializeAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub quote_asset_mint: Pubkey,
    pub drift_signer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializeSpotMarketAccounts {
    pub spot_market: Pubkey,
    pub spot_market_mint: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub state: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct DeleteInitializedSpotMarketAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub token_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializeSerumFulfillmentConfigAccounts {
    pub base_spot_market: Pubkey,
    pub quote_spot_market: Pubkey,
    pub state: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_open_orders: Pubkey,
    pub drift_signer: Pubkey,
    pub serum_fulfillment_config: Pubkey,
    pub admin: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSerumFulfillmentConfigStatusAccounts {
    pub state: Pubkey,
    pub serum_fulfillment_config: Pubkey,
    pub admin: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializePhoenixFulfillmentConfigAccounts {
    pub base_spot_market: Pubkey,
    pub quote_spot_market: Pubkey,
    pub state: Pubkey,
    pub phoenix_program: Pubkey,
    pub phoenix_market: Pubkey,
    pub drift_signer: Pubkey,
    pub phoenix_fulfillment_config: Pubkey,
    pub admin: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PhoenixFulfillmentConfigStatusAccounts {
    pub state: Pubkey,
    pub phoenix_fulfillment_config: Pubkey,
    pub admin: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSerumVaultAccounts {
    pub state: Pubkey,
    pub admin: Pubkey,
    pub srm_vault: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializePerpMarketAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct DeleteInitializedPerpMarketAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct MoveAmmPriceAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct RecenterPerpMarketAmmAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketAmmSummaryStatsAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub spot_market: Pubkey,
    pub oracle: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketExpiryAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SettleExpiredMarketPoolsToRevenuePoolAccounts {
    pub state: Pubkey,
    pub admin: Pubkey,
    pub spot_market: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct DepositIntoPerpMarketFeePoolAccounts {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub admin: Pubkey,
    pub source_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub quote_spot_market: Pubkey,
    pub spot_market_vault: Pubkey,
    pub token_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct DepositIntoSpotMarketRevenuePoolAccounts {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct RepegAmmCurveAccounts {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketAmmOracleTwapAccounts {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ResetPerpMarketAmmOracleTwapAccounts {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateKAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMarginRatioAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketFundingPeriodAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxImbalancesAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketLiquidationFeeAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateInsuranceFundUnstakingPeriodAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketLiquidationFeeAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateWithdrawGuardThresholdAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketIfFactorAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketRevenueSettlePeriodAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketStatusAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketPausedOperationsAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketAssetTierAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketMarginWeightsAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketBorrowRateAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketMaxTokenDepositsAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketScaleInitialAssetWeightStartAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketOracleAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub oracle: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketStepSizeAndTickSizeAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketMinOrderSizeAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketOrdersEnabledAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketIfPausedOperationsAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketNameAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketStatusAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketPausedOperationsAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketContractTierAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketUnrealizedAssetWeightAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketConcentrationCoefAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketCurveUpdateIntensityAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketPerLpBaseAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateLpCooldownTimeAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpFeeStructureAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotFeeStructureAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateInitialPctToLiquidateAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateLiquidationDurationAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateLiquidationMarginBufferRatioAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateOracleGuardRailsAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateStateSettlementDurationAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateStateMaxNumberOfSubAccountsAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateStateMaxInitializeUserFeeAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketOracleAccounts {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketBaseSpreadAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateAmmJitIntensityAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxSpreadAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketStepSizeAndTickSizeAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketNameAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMinOrderSizeAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxSlippageRatioAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxFillReserveFractionAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxOpenInterestAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketNumberOfUsersAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketFeeAdjustmentAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketFeeAdjustmentAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateAdminAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateWhitelistMintAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateDiscountMintAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateExchangeStatusAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpAuctionDurationAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateSpotAuctionDurationAccounts {
    pub admin: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializeProtocolIfSharesTransferConfigAccounts {
    pub admin: Pubkey,
    pub protocol_if_shares_transfer_config: Pubkey,
    pub state: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdateProtocolIfSharesTransferConfigAccounts {
    pub admin: Pubkey,
    pub protocol_if_shares_transfer_config: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InitializePrelaunchOracleAccounts {
    pub admin: Pubkey,
    pub prelaunch_oracle: Pubkey,
    pub state: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePrelaunchOracleParamsAccounts {
    pub admin: Pubkey,
    pub prelaunch_oracle: Pubkey,
    pub perp_market: Pubkey,
    pub state: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct DeletePrelaunchOracleAccounts {
    pub admin: Pubkey,
    pub prelaunch_oracle: Pubkey,
    pub perp_market: Pubkey,
    pub state: Pubkey,
}

/// Accounts

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PhoenixV1FulfillmentConfig {
    pub pubkey: Pubkey,
    pub phoenix_program_id: Pubkey,
    pub phoenix_log_authority: Pubkey,
    pub phoenix_market: Pubkey,
    pub phoenix_base_vault: Pubkey,
    pub phoenix_quote_vault: Pubkey,
    pub market_index: u16,
    pub fulfillment_type: SpotFulfillmentType,
    pub status: SpotFulfillmentConfigStatus,
    pub padding: [u8; 4],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SerumV3FulfillmentConfig {
    pub pubkey: Pubkey,
    pub serum_program_id: Pubkey,
    pub serum_market: Pubkey,
    pub serum_request_queue: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub serum_base_vault: Pubkey,
    pub serum_quote_vault: Pubkey,
    pub serum_open_orders: Pubkey,
    pub serum_signer_nonce: u64,
    pub market_index: u16,
    pub fulfillment_type: SpotFulfillmentType,
    pub status: SpotFulfillmentConfigStatus,
    pub padding: [u8; 4],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InsuranceFundStake {
    pub authority: Pubkey,
    pub if_shares: u128,
    pub last_withdraw_request_shares: u128,
    pub if_base: u128,
    pub last_valid_ts: i64,
    pub last_withdraw_request_value: u64,
    pub last_withdraw_request_ts: i64,
    pub cost_basis: i64,
    pub market_index: u16,
    pub padding: [u8; 14],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ProtocolIfSharesTransferConfig {
    pub whitelisted_signers: [Pubkey; 4],
    pub max_transfer_per_epoch: u128,
    pub current_epoch_transfer: u128,
    pub next_epoch_ts: i64,
    pub padding: [u128; 8],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PrelaunchOracle {
    pub price: i64,
    pub max_price: i64,
    pub confidence: u64,
    pub last_update_slot: u64,
    pub amm_last_update_slot: u64,
    pub perp_market_index: u16,
    pub padding: [u8; 70],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PerpMarket {
    /// The perp market's address. It is a pda of the market index
    pub pubkey: Pubkey,
    /// The automated market maker
    pub amm: AMM,
    /// The market's pnl pool. When users settle negative pnl, the balance increases.
    /// When users settle positive pnl, the balance decreases. Can not go negative.
    pub pnl_pool: PoolBalance,
    /// Encoded display name for the perp market e.g. SOL-PERP
    pub name: [u8; 32],
    /// The perp market's claim on the insurance fund
    pub insurance_claim: InsuranceClaim,
    /// The max pnl imbalance before positive pnl asset weight is discounted
    /// pnl imbalance is the difference between long and short pnl. When it's greater than 0,
    /// the amm has negative pnl and the initial asset weight for positive pnl is discounted
    /// precision = QUOTE_PRECISION
    pub unrealized_pnl_max_imbalance: u64,
    /// The ts when the market will be expired. Only set if market is in reduce only mode
    pub expiry_ts: i64,
    /// The price at which positions will be settled. Only set if market is expired
    /// precision = PRICE_PRECISION
    pub expiry_price: i64,
    /// Every trade has a fill record id. This is the next id to be used
    pub next_fill_record_id: u64,
    /// Every funding rate update has a record id. This is the next id to be used
    pub next_funding_rate_record_id: u64,
    /// Every amm k updated has a record id. This is the next id to be used
    pub next_curve_record_id: u64,
    /// The initial margin fraction factor. Used to increase margin ratio for large positions
    /// precision: MARGIN_PRECISION
    pub imf_factor: u32,
    /// The imf factor for unrealized pnl. Used to discount asset weight for large positive pnl
    /// precision: MARGIN_PRECISION
    pub unrealized_pnl_imf_factor: u32,
    /// The fee the liquidator is paid for taking over perp position
    /// precision: LIQUIDATOR_FEE_PRECISION
    pub liquidator_fee: u32,
    /// The fee the insurance fund receives from liquidation
    /// precision: LIQUIDATOR_FEE_PRECISION
    pub if_liquidation_fee: u32,
    /// The margin ratio which determines how much collateral is required to open a position
    /// e.g. margin ratio of .1 means a user must have $100 of total collateral to open a $1000 position
    /// precision: MARGIN_PRECISION
    pub margin_ratio_initial: u32,
    /// The margin ratio which determines when a user will be liquidated
    /// e.g. margin ratio of .05 means a user must have $50 of total collateral to maintain a $1000 position
    /// else they will be liquidated
    /// precision: MARGIN_PRECISION
    pub margin_ratio_maintenance: u32,
    /// The initial asset weight for positive pnl. Negative pnl always has an asset weight of 1
    /// precision: SPOT_WEIGHT_PRECISION
    pub unrealized_pnl_initial_asset_weight: u32,
    /// The maintenance asset weight for positive pnl. Negative pnl always has an asset weight of 1
    /// precision: SPOT_WEIGHT_PRECISION
    pub unrealized_pnl_maintenance_asset_weight: u32,
    /// number of users in a position (base)
    pub number_of_users_with_base: u32,
    /// number of users in a position (pnl) or pnl (quote)
    pub number_of_users: u32,
    pub market_index: u16,
    /// Whether a market is active, reduce only, expired, etc
    /// Affects whether users can open/close positions
    pub status: MarketStatus,
    /// Currently only Perpetual markets are supported
    pub contract_type: ContractType,
    /// The contract tier determines how much insurance a market can receive, with more speculative markets receiving less insurance
    /// It also influences the order perp markets can be liquidated, with less speculative markets being liquidated first
    pub contract_tier: ContractTier,
    pub paused_operations: u8,
    /// The spot market that pnl is settled in
    pub quote_spot_market_index: u16,
    /// Between -100 and 100, represents what % to increase/decrease the fee by
    /// E.g. if this is -50 and the fee is 5bps, the new fee will be 2.5bps
    /// if this is 50 and the fee is 5bps, the new fee will be 7.5bps
    pub fee_adjustment: i16,
    pub padding: [u8; 46],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SpotMarket {
    /// The address of the spot market. It is a pda of the market index
    pub pubkey: Pubkey,
    /// The oracle used to price the markets deposits/borrows
    pub oracle: Pubkey,
    /// The token mint of the market
    pub mint: Pubkey,
    /// The vault used to store the market's deposits
    /// The amount in the vault should be equal to or greater than deposits - borrows
    pub vault: Pubkey,
    /// The encoded display name for the market e.g. SOL
    pub name: [u8; 32],
    pub historical_oracle_data: HistoricalOracleData,
    pub historical_index_data: HistoricalIndexData,
    /// Revenue the protocol has collected in this markets token
    /// e.g. for SOL-PERP, funds can be settled in usdc and will flow into the USDC revenue pool
    pub revenue_pool: PoolBalance,
    /// The fees collected from swaps between this market and the quote market
    /// Is settled to the quote markets revenue pool
    pub spot_fee_pool: PoolBalance,
    /// Details on the insurance fund covering bankruptcies in this markets token
    /// Covers bankruptcies for borrows with this markets token and perps settling in this markets token
    pub insurance_fund: InsuranceFund,
    /// The total spot fees collected for this market
    /// precision: QUOTE_PRECISION
    pub total_spot_fee: u128,
    /// The sum of the scaled balances for deposits across users and pool balances
    /// To convert to the deposit token amount, multiply by the cumulative deposit interest
    /// precision: SPOT_BALANCE_PRECISION
    pub deposit_balance: u128,
    /// The sum of the scaled balances for borrows across users and pool balances
    /// To convert to the borrow token amount, multiply by the cumulative borrow interest
    /// precision: SPOT_BALANCE_PRECISION
    pub borrow_balance: u128,
    /// The cumulative interest earned by depositors
    /// Used to calculate the deposit token amount from the deposit balance
    /// precision: SPOT_CUMULATIVE_INTEREST_PRECISION
    pub cumulative_deposit_interest: u128,
    /// The cumulative interest earned by borrowers
    /// Used to calculate the borrow token amount from the borrow balance
    /// precision: SPOT_CUMULATIVE_INTEREST_PRECISION
    pub cumulative_borrow_interest: u128,
    /// The total socialized loss from borrows, in the mint's token
    /// precision: token mint precision
    pub total_social_loss: u128,
    /// The total socialized loss from borrows, in the quote market's token
    /// preicision: QUOTE_PRECISION
    pub total_quote_social_loss: u128,
    /// no withdraw limits/guards when deposits below this threshold
    /// precision: token mint precision
    pub withdraw_guard_threshold: u64,
    /// The max amount of token deposits in this market
    /// 0 if there is no limit
    /// precision: token mint precision
    pub max_token_deposits: u64,
    /// 24hr average of deposit token amount
    /// precision: token mint precision
    pub deposit_token_twap: u64,
    /// 24hr average of borrow token amount
    /// precision: token mint precision
    pub borrow_token_twap: u64,
    /// 24hr average of utilization
    /// which is borrow amount over token amount
    /// precision: SPOT_UTILIZATION_PRECISION
    pub utilization_twap: u64,
    /// Last time the cumulative deposit and borrow interest was updated
    pub last_interest_ts: u64,
    /// Last time the deposit/borrow/utilization averages were updated
    pub last_twap_ts: u64,
    /// The time the market is set to expire. Only set if market is in reduce only mode
    pub expiry_ts: i64,
    /// Spot orders must be a multiple of the step size
    /// precision: token mint precision
    pub order_step_size: u64,
    /// Spot orders must be a multiple of the tick size
    /// precision: PRICE_PRECISION
    pub order_tick_size: u64,
    /// The minimum order size
    /// precision: token mint precision
    pub min_order_size: u64,
    /// The maximum spot position size
    /// if the limit is 0, there is no limit
    /// precision: token mint precision
    pub max_position_size: u64,
    /// Every spot trade has a fill record id. This is the next id to use
    pub next_fill_record_id: u64,
    /// Every deposit has a deposit record id. This is the next id to use
    pub next_deposit_record_id: u64,
    /// The initial asset weight used to calculate a deposits contribution to a users initial total collateral
    /// e.g. if the asset weight is .8, $100 of deposits contributes $80 to the users initial total collateral
    /// precision: SPOT_WEIGHT_PRECISION
    pub initial_asset_weight: u32,
    /// The maintenance asset weight used to calculate a deposits contribution to a users maintenance total collateral
    /// e.g. if the asset weight is .9, $100 of deposits contributes $90 to the users maintenance total collateral
    /// precision: SPOT_WEIGHT_PRECISION
    pub maintenance_asset_weight: u32,
    /// The initial liability weight used to calculate a borrows contribution to a users initial margin requirement
    /// e.g. if the liability weight is .9, $100 of borrows contributes $90 to the users initial margin requirement
    /// precision: SPOT_WEIGHT_PRECISION
    pub initial_liability_weight: u32,
    /// The maintenance liability weight used to calculate a borrows contribution to a users maintenance margin requirement
    /// e.g. if the liability weight is .8, $100 of borrows contributes $80 to the users maintenance margin requirement
    /// precision: SPOT_WEIGHT_PRECISION
    pub maintenance_liability_weight: u32,
    /// The initial margin fraction factor. Used to increase liability weight/decrease asset weight for large positions
    /// precision: MARGIN_PRECISION
    pub imf_factor: u32,
    /// The fee the liquidator is paid for taking over borrow/deposit
    /// precision: LIQUIDATOR_FEE_PRECISION
    pub liquidator_fee: u32,
    /// The fee the insurance fund receives from liquidation
    /// precision: LIQUIDATOR_FEE_PRECISION
    pub if_liquidation_fee: u32,
    /// The optimal utilization rate for this market.
    /// Used to determine the markets borrow rate
    /// precision: SPOT_UTILIZATION_PRECISION
    pub optimal_utilization: u32,
    /// The borrow rate for this market when the market has optimal utilization
    /// precision: SPOT_RATE_PRECISION
    pub optimal_borrow_rate: u32,
    /// The borrow rate for this market when the market has 1000 utilization
    /// precision: SPOT_RATE_PRECISION
    pub max_borrow_rate: u32,
    /// The market's token mint's decimals. To from decimals to a precision, 10^decimals
    pub decimals: u32,
    pub market_index: u16,
    /// Whether or not spot trading is enabled
    pub orders_enabled: bool,
    pub oracle_source: OracleSource,
    pub status: MarketStatus,
    /// The asset tier affects how a deposit can be used as collateral and the priority for a borrow being liquidated
    pub asset_tier: AssetTier,
    pub paused_operations: u8,
    pub if_paused_operations: u8,
    pub fee_adjustment: i16,
    pub padding1: [u8; 2],
    /// For swaps, the amount of token loaned out in the begin_swap ix
    /// precision: token mint precision
    pub flash_loan_amount: u64,
    /// For swaps, the amount in the users token account in the begin_swap ix
    /// Used to calculate how much of the token left the system in end_swap ix
    /// precision: token mint precision
    pub flash_loan_initial_token_amount: u64,
    /// The total fees received from swaps
    /// precision: token mint precision
    pub total_swap_fee: u64,
    /// When to begin scaling down the initial asset weight
    /// disabled when 0
    /// precision: QUOTE_PRECISION
    pub scale_initial_asset_weight_start: u64,
    pub padding: [u8; 48],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct State {
    pub admin: Pubkey,
    pub whitelist_mint: Pubkey,
    pub discount_mint: Pubkey,
    pub signer: Pubkey,
    pub srm_vault: Pubkey,
    pub perp_fee_structure: FeeStructure,
    pub spot_fee_structure: FeeStructure,
    pub oracle_guard_rails: OracleGuardRails,
    pub number_of_authorities: u64,
    pub number_of_sub_accounts: u64,
    pub lp_cooldown_time: u64,
    pub liquidation_margin_buffer_ratio: u32,
    pub settlement_duration: u16,
    pub number_of_markets: u16,
    pub number_of_spot_markets: u16,
    pub signer_nonce: u8,
    pub min_perp_auction_duration: u8,
    pub default_market_order_time_in_force: u8,
    pub default_spot_auction_duration: u8,
    pub exchange_status: u8,
    pub liquidation_duration: u8,
    pub initial_pct_to_liquidate: u16,
    pub max_number_of_sub_accounts: u16,
    pub max_initialize_user_fee: u16,
    pub padding: [u8; 10],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct User {
    /// The owner/authority of the account
    pub authority: Pubkey,
    /// An addresses that can control the account on the authority's behalf. Has limited power, cant withdraw
    pub delegate: Pubkey,
    /// Encoded display name e.g. "toly"
    pub name: [u8; 32],
    /// The user's spot positions
    pub spot_positions: [SpotPosition; 8],
    /// The user's perp positions
    pub perp_positions: [PerpPosition; 8],
    /// The user's orders
    pub orders: [Order; 32],
    /// The last time the user added perp lp positions
    pub last_add_perp_lp_shares_ts: i64,
    /// The total values of deposits the user has made
    /// precision: QUOTE_PRECISION
    pub total_deposits: u64,
    /// The total values of withdrawals the user has made
    /// precision: QUOTE_PRECISION
    pub total_withdraws: u64,
    /// The total socialized loss the users has incurred upon the protocol
    /// precision: QUOTE_PRECISION
    pub total_social_loss: u64,
    /// Fees (taker fees, maker rebate, referrer reward, filler reward) and pnl for perps
    /// precision: QUOTE_PRECISION
    pub settled_perp_pnl: i64,
    /// Fees (taker fees, maker rebate, filler reward) for spot
    /// precision: QUOTE_PRECISION
    pub cumulative_spot_fees: i64,
    /// Cumulative funding paid/received for perps
    /// precision: QUOTE_PRECISION
    pub cumulative_perp_funding: i64,
    /// The amount of margin freed during liquidation. Used to force the liquidation to occur over a period of time
    /// Defaults to zero when not being liquidated
    /// precision: QUOTE_PRECISION
    pub liquidation_margin_freed: u64,
    /// The last slot a user was active. Used to determine if a user is idle
    pub last_active_slot: u64,
    /// Every user order has an order id. This is the next order id to be used
    pub next_order_id: u32,
    /// Custom max initial margin ratio for the user
    pub max_margin_ratio: u32,
    /// The next liquidation id to be used for user
    pub next_liquidation_id: u16,
    /// The sub account id for this user
    pub sub_account_id: u16,
    /// Whether the user is active, being liquidated or bankrupt
    pub status: u8,
    /// Whether the user has enabled margin trading
    pub is_margin_trading_enabled: bool,
    /// User is idle if they haven't interacted with the protocol in 1 week and they have no orders, perp positions or borrows
    /// Off-chain keeper bots can ignore users that are idle
    pub idle: bool,
    /// number of open orders
    pub open_orders: u8,
    /// Whether or not user has open order
    pub has_open_order: bool,
    /// number of open orders with auction
    pub open_auctions: u8,
    /// Whether or not user has open order with auction
    pub has_open_auction: bool,
    pub padding: [u8; 21],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UserStats {
    /// The authority for all of a users sub accounts
    pub authority: Pubkey,
    /// The address that referred this user
    pub referrer: Pubkey,
    /// Stats on the fees paid by the user
    pub fees: UserFees,
    /// The timestamp of the next epoch
    /// Epoch is used to limit referrer rewards earned in single epoch
    pub next_epoch_ts: i64,
    /// Rolling 30day maker volume for user
    /// precision: QUOTE_PRECISION
    pub maker_volume_30d: u64,
    /// Rolling 30day taker volume for user
    /// precision: QUOTE_PRECISION
    pub taker_volume_30d: u64,
    /// Rolling 30day filler volume for user
    /// precision: QUOTE_PRECISION
    pub filler_volume_30d: u64,
    /// last time the maker volume was updated
    pub last_maker_volume_30d_ts: i64,
    /// last time the taker volume was updated
    pub last_taker_volume_30d_ts: i64,
    /// last time the filler volume was updated
    pub last_filler_volume_30d_ts: i64,
    /// The amount of tokens staked in the quote spot markets if
    pub if_staked_quote_asset_amount: u64,
    /// The current number of sub accounts
    pub number_of_sub_accounts: u16,
    /// The number of sub accounts created. Can be greater than the number of sub accounts if user
    /// has deleted sub accounts
    pub number_of_sub_accounts_created: u16,
    /// Whether the user is a referrer. Sub account 0 can not be deleted if user is a referrer
    pub is_referrer: bool,
    pub disable_update_perp_bid_ask_twap: bool,
    pub padding: [u8; 50],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ReferrerName {
    pub authority: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub name: [u8; 32],
}

/// Types definition

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketSummaryStatsParams {
    pub quote_asset_amount_with_unsettled_lp: Option<i64>,
    pub net_unsettled_funding_pnl: Option<i64>,
    pub update_amm_summary_stats: Option<bool>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct LiquidatePerpRecord {
    pub market_index: u16,
    pub oracle_price: i64,
    pub base_asset_amount: i64,
    pub quote_asset_amount: i64,
    /// precision: AMM_RESERVE_PRECISION
    pub lp_shares: u64,
    pub fill_record_id: u64,
    pub user_order_id: u32,
    pub liquidator_order_id: u32,
    /// precision: QUOTE_PRECISION
    pub liquidator_fee: u64,
    /// precision: QUOTE_PRECISION
    pub if_fee: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct LiquidateSpotRecord {
    pub asset_market_index: u16,
    pub asset_price: i64,
    pub asset_transfer: u128,
    pub liability_market_index: u16,
    pub liability_price: i64,
    /// precision: token mint precision
    pub liability_transfer: u128,
    /// precision: token mint precision
    pub if_fee: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct LiquidateBorrowForPerpPnlRecord {
    pub perp_market_index: u16,
    pub market_oracle_price: i64,
    pub pnl_transfer: u128,
    pub liability_market_index: u16,
    pub liability_price: i64,
    pub liability_transfer: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct LiquidatePerpPnlForDepositRecord {
    pub perp_market_index: u16,
    pub market_oracle_price: i64,
    pub pnl_transfer: u128,
    pub asset_market_index: u16,
    pub asset_price: i64,
    pub asset_transfer: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PerpBankruptcyRecord {
    pub market_index: u16,
    pub pnl: i128,
    pub if_payment: u128,
    pub clawback_user: Option<Pubkey>,
    pub clawback_user_payment: Option<i128>,
    pub cumulative_funding_rate_delta: i128,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SpotBankruptcyRecord {
    pub market_index: u16,
    pub borrow_amount: u128,
    pub if_payment: u128,
    pub cumulative_deposit_interest_delta: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct MarketIdentifier {
    pub market_type: MarketType,
    pub market_index: u16,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct HistoricalOracleData {
    /// precision: PRICE_PRECISION
    pub last_oracle_price: i64,
    /// precision: PRICE_PRECISION
    pub last_oracle_conf: u64,
    /// number of slots since last update
    pub last_oracle_delay: i64,
    /// precision: PRICE_PRECISION
    pub last_oracle_price_twap: i64,
    /// precision: PRICE_PRECISION
    pub last_oracle_price_twap_5min: i64,
    /// unix_timestamp of last snapshot
    pub last_oracle_price_twap_ts: i64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct HistoricalIndexData {
    /// precision: PRICE_PRECISION
    pub last_index_bid_price: u64,
    /// precision: PRICE_PRECISION
    pub last_index_ask_price: u64,
    /// precision: PRICE_PRECISION
    pub last_index_price_twap: u64,
    /// precision: PRICE_PRECISION
    pub last_index_price_twap_5min: u64,
    /// unix_timestamp of last snapshot
    pub last_index_price_twap_ts: i64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PrelaunchOracleParams {
    pub perp_market_index: u16,
    pub price: Option<i64>,
    pub max_price: Option<i64>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct OrderParams {
    pub order_type: OrderType,
    pub market_type: MarketType,
    pub direction: PositionDirection,
    pub user_order_id: u8,
    pub base_asset_amount: u64,
    pub price: u64,
    pub market_index: u16,
    pub reduce_only: bool,
    pub post_only: PostOnlyParam,
    pub immediate_or_cancel: bool,
    pub max_ts: Option<i64>,
    pub trigger_price: Option<u64>,
    pub trigger_condition: OrderTriggerCondition,
    pub oracle_price_offset: Option<i32>,
    pub auction_duration: Option<u8>,
    pub auction_start_price: Option<i64>,
    pub auction_end_price: Option<i64>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ModifyOrderParams {
    pub direction: Option<PositionDirection>,
    pub base_asset_amount: Option<u64>,
    pub price: Option<u64>,
    pub reduce_only: Option<bool>,
    pub post_only: Option<PostOnlyParam>,
    pub immediate_or_cancel: Option<bool>,
    pub max_ts: Option<i64>,
    pub trigger_price: Option<u64>,
    pub trigger_condition: Option<OrderTriggerCondition>,
    pub oracle_price_offset: Option<i32>,
    pub auction_duration: Option<u8>,
    pub auction_start_price: Option<i64>,
    pub auction_end_price: Option<i64>,
    pub policy: Option<ModifyOrderPolicy>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InsuranceClaim {
    /// The amount of revenue last settled
    /// Positive if funds left the perp market,
    /// negative if funds were pulled into the perp market
    /// precision: QUOTE_PRECISION
    pub revenue_withdraw_since_last_settle: i64,
    /// The max amount of revenue that can be withdrawn per period
    /// precision: QUOTE_PRECISION
    pub max_revenue_withdraw_per_period: u64,
    /// The max amount of insurance that perp market can use to resolve bankruptcy and pnl deficits
    /// precision: QUOTE_PRECISION
    pub quote_max_insurance: u64,
    /// The amount of insurance that has been used to resolve bankruptcy and pnl deficits
    /// precision: QUOTE_PRECISION
    pub quote_settled_insurance: u64,
    /// The last time revenue was settled in/out of market
    pub last_revenue_withdraw_ts: i64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PoolBalance {
    /// To get the pool's token amount, you must multiply the scaled balance by the market's cumulative
    /// deposit interest
    /// precision: SPOT_BALANCE_PRECISION
    pub scaled_balance: i64,
    /// The spot market the pool is for
    pub market_index: u16,
    pub padding: [u8; 6],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct AMM {
    /// oracle price data public key
    pub oracle: Pubkey,
    /// stores historically witnessed oracle data
    pub historical_oracle_data: HistoricalOracleData,
    /// accumulated base asset amount since inception per lp share
    /// precision: QUOTE_PRECISION
    pub base_asset_amount_per_lp: i128,
    /// accumulated quote asset amount since inception per lp share
    /// precision: QUOTE_PRECISION
    pub quote_asset_amount_per_lp: i128,
    /// partition of fees from perp market trading moved from pnl settlements
    pub fee_pool: PoolBalance,
    /// `x` reserves for constant product mm formula (x * y = k)
    /// precision: AMM_RESERVE_PRECISION
    pub base_asset_reserve: u128,
    /// `y` reserves for constant product mm formula (x * y = k)
    /// precision: AMM_RESERVE_PRECISION
    pub quote_asset_reserve: u128,
    /// determines how close the min/max base asset reserve sit vs base reserves
    /// allow for decreasing slippage without increasing liquidity and v.v.
    /// precision: PERCENTAGE_PRECISION
    pub concentration_coef: u128,
    /// minimum base_asset_reserve allowed before AMM is unavailable
    /// precision: AMM_RESERVE_PRECISION
    pub min_base_asset_reserve: u128,
    /// maximum base_asset_reserve allowed before AMM is unavailable
    /// precision: AMM_RESERVE_PRECISION
    pub max_base_asset_reserve: u128,
    /// `sqrt(k)` in constant product mm formula (x * y = k). stored to avoid drift caused by integer math issues
    /// precision: AMM_RESERVE_PRECISION
    pub sqrt_k: u128,
    /// normalizing numerical factor for y, its use offers lowest slippage in cp-curve when market is balanced
    /// precision: PEG_PRECISION
    pub peg_multiplier: u128,
    /// y when market is balanced. stored to save computation
    /// precision: AMM_RESERVE_PRECISION
    pub terminal_quote_asset_reserve: u128,
    /// always non-negative. tracks number of total longs in market (regardless of counterparty)
    /// precision: BASE_PRECISION
    pub base_asset_amount_long: i128,
    /// always non-positive. tracks number of total shorts in market (regardless of counterparty)
    /// precision: BASE_PRECISION
    pub base_asset_amount_short: i128,
    /// tracks net position (longs-shorts) in market with AMM as counterparty
    /// precision: BASE_PRECISION
    pub base_asset_amount_with_amm: i128,
    /// tracks net position (longs-shorts) in market with LPs as counterparty
    /// precision: BASE_PRECISION
    pub base_asset_amount_with_unsettled_lp: i128,
    /// max allowed open interest, blocks trades that breach this value
    /// precision: BASE_PRECISION
    pub max_open_interest: u128,
    /// sum of all user's perp quote_asset_amount in market
    /// precision: QUOTE_PRECISION
    pub quote_asset_amount: i128,
    /// sum of all long user's quote_entry_amount in market
    /// precision: QUOTE_PRECISION
    pub quote_entry_amount_long: i128,
    /// sum of all short user's quote_entry_amount in market
    /// precision: QUOTE_PRECISION
    pub quote_entry_amount_short: i128,
    /// sum of all long user's quote_break_even_amount in market
    /// precision: QUOTE_PRECISION
    pub quote_break_even_amount_long: i128,
    /// sum of all short user's quote_break_even_amount in market
    /// precision: QUOTE_PRECISION
    pub quote_break_even_amount_short: i128,
    /// total user lp shares of sqrt_k (protocol owned liquidity = sqrt_k - last_funding_rate)
    /// precision: AMM_RESERVE_PRECISION
    pub user_lp_shares: u128,
    /// last funding rate in this perp market (unit is quote per base)
    /// precision: QUOTE_PRECISION
    pub last_funding_rate: i64,
    /// last funding rate for longs in this perp market (unit is quote per base)
    /// precision: QUOTE_PRECISION
    pub last_funding_rate_long: i64,
    /// last funding rate for shorts in this perp market (unit is quote per base)
    /// precision: QUOTE_PRECISION
    pub last_funding_rate_short: i64,
    /// estimate of last 24h of funding rate perp market (unit is quote per base)
    /// precision: QUOTE_PRECISION
    pub last_24h_avg_funding_rate: i64,
    /// total fees collected by this perp market
    /// precision: QUOTE_PRECISION
    pub total_fee: i128,
    /// total fees collected by the vAMM's bid/ask spread
    /// precision: QUOTE_PRECISION
    pub total_mm_fee: i128,
    /// total fees collected by exchange fee schedule
    /// precision: QUOTE_PRECISION
    pub total_exchange_fee: u128,
    /// total fees minus any recognized upnl and pool withdraws
    /// precision: QUOTE_PRECISION
    pub total_fee_minus_distributions: i128,
    /// sum of all fees from fee pool withdrawn to revenue pool
    /// precision: QUOTE_PRECISION
    pub total_fee_withdrawn: u128,
    /// all fees collected by market for liquidations
    /// precision: QUOTE_PRECISION
    pub total_liquidation_fee: u128,
    /// accumulated funding rate for longs since inception in market
    pub cumulative_funding_rate_long: i128,
    /// accumulated funding rate for shorts since inception in market
    pub cumulative_funding_rate_short: i128,
    /// accumulated social loss paid by users since inception in market
    pub total_social_loss: u128,
    /// transformed base_asset_reserve for users going long
    /// precision: AMM_RESERVE_PRECISION
    pub ask_base_asset_reserve: u128,
    /// transformed quote_asset_reserve for users going long
    /// precision: AMM_RESERVE_PRECISION
    pub ask_quote_asset_reserve: u128,
    /// transformed quote_asset_reserve for users going short
    /// precision: AMM_RESERVE_PRECISION
    pub bid_quote_asset_reserve: u128,
    /// the last seen oracle price partially shrunk toward the amm reserve price
    /// precision: PRICE_PRECISION
    pub last_oracle_normalised_price: i64,
    /// the gap between the oracle price and the reserve price = y * peg_multiplier / x
    pub last_oracle_reserve_price_spread_pct: i64,
    /// average estimate of bid price over funding_period
    /// precision: PRICE_PRECISION
    pub last_bid_price_twap: u64,
    /// average estimate of ask price over funding_period
    /// precision: PRICE_PRECISION
    pub last_ask_price_twap: u64,
    /// average estimate of (bid+ask)/2 price over funding_period
    /// precision: PRICE_PRECISION
    pub last_mark_price_twap: u64,
    /// average estimate of (bid+ask)/2 price over FIVE_MINUTES
    /// precision: PRICE_PRECISION
    pub last_mark_price_twap_5min: u64,
    /// the last blockchain slot the amm was updated
    pub last_update_slot: u64,
    /// the pct size of the oracle confidence interval
    /// precision: PERCENTAGE_PRECISION
    pub last_oracle_conf_pct: u64,
    /// the total_fee_minus_distribution change since the last funding update
    /// precision: QUOTE_PRECISION
    pub net_revenue_since_last_funding: i64,
    /// the last funding rate update unix_timestamp
    pub last_funding_rate_ts: i64,
    /// the peridocity of the funding rate updates
    pub funding_period: i64,
    /// the base step size (increment) of orders
    /// precision: BASE_PRECISION
    pub order_step_size: u64,
    /// the price tick size of orders
    /// precision: PRICE_PRECISION
    pub order_tick_size: u64,
    /// the minimum base size of an order
    /// precision: BASE_PRECISION
    pub min_order_size: u64,
    /// the max base size a single user can have
    /// precision: BASE_PRECISION
    pub max_position_size: u64,
    /// estimated total of volume in market
    /// QUOTE_PRECISION
    pub volume_24h: u64,
    /// the volume intensity of long fills against AMM
    pub long_intensity_volume: u64,
    /// the volume intensity of short fills against AMM
    pub short_intensity_volume: u64,
    /// the blockchain unix timestamp at the time of the last trade
    pub last_trade_ts: i64,
    /// estimate of standard deviation of the fill (mark) prices
    /// precision: PRICE_PRECISION
    pub mark_std: u64,
    /// estimate of standard deviation of the oracle price at each update
    /// precision: PRICE_PRECISION
    pub oracle_std: u64,
    /// the last unix_timestamp the mark twap was updated
    pub last_mark_price_twap_ts: i64,
    /// the minimum spread the AMM can quote. also used as step size for some spread logic increases.
    pub base_spread: u32,
    /// the maximum spread the AMM can quote
    pub max_spread: u32,
    /// the spread for asks vs the reserve price
    pub long_spread: u32,
    /// the spread for bids vs the reserve price
    pub short_spread: u32,
    /// the count intensity of long fills against AMM
    pub long_intensity_count: u32,
    /// the count intensity of short fills against AMM
    pub short_intensity_count: u32,
    /// the fraction of total available liquidity a single fill on the AMM can consume
    pub max_fill_reserve_fraction: u16,
    /// the maximum slippage a single fill on the AMM can push
    pub max_slippage_ratio: u16,
    /// the update intensity of AMM formulaic updates (adjusting k). 0-100
    pub curve_update_intensity: u8,
    /// the jit intensity of AMM. larger intensity means larger participation in jit. 0 means no jit participation.
    /// (0, 100] is intensity for protocol-owned AMM. (100, 200] is intensity for user LP-owned AMM.
    pub amm_jit_intensity: u8,
    /// the oracle provider information. used to decode/scale the oracle public key
    pub oracle_source: OracleSource,
    /// tracks whether the oracle was considered valid at the last AMM update
    pub last_oracle_valid: bool,
    /// the target value for `base_asset_amount_per_lp`, used during AMM JIT with LP split
    /// precision: BASE_PRECISION
    pub target_base_asset_amount_per_lp: i32,
    /// expo for unit of per_lp, base 10 (if per_lp_base=X, then per_lp unit is 10^X)
    pub per_lp_base: i8,
    pub padding1: u8,
    pub padding2: u16,
    pub total_fee_earned_per_lp: u64,
    pub net_unsettled_funding_pnl: i64,
    pub quote_asset_amount_with_unsettled_lp: i64,
    pub reference_price_offset: i32,
    pub padding: [u8; 12],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct InsuranceFund {
    pub vault: Pubkey,
    pub total_shares: u128,
    pub user_shares: u128,
    pub shares_base: u128,
    pub unstaking_period: i64,
    pub last_revenue_settle_ts: i64,
    pub revenue_settle_period: i64,
    pub total_factor: u32,
    pub user_factor: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct OracleGuardRails {
    pub price_divergence: PriceDivergenceGuardRails,
    pub validity: ValidityGuardRails,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PriceDivergenceGuardRails {
    pub mark_oracle_percent_divergence: u64,
    pub oracle_twap_5min_percent_divergence: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ValidityGuardRails {
    pub slots_before_stale_for_amm: i64,
    pub slots_before_stale_for_margin: i64,
    pub confidence_interval_max_size: u64,
    pub too_volatile_ratio: i64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct FeeStructure {
    pub fee_tiers: [FeeTier; 10],
    pub filler_reward_structure: OrderFillerRewardStructure,
    pub referrer_reward_epoch_upper_bound: u64,
    pub flat_filler_fee: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct FeeTier {
    pub fee_numerator: u32,
    pub fee_denominator: u32,
    pub maker_rebate_numerator: u32,
    pub maker_rebate_denominator: u32,
    pub referrer_reward_numerator: u32,
    pub referrer_reward_denominator: u32,
    pub referee_fee_numerator: u32,
    pub referee_fee_denominator: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct OrderFillerRewardStructure {
    pub reward_numerator: u32,
    pub reward_denominator: u32,
    pub time_based_reward_lower_bound: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct UserFees {
    /// Total taker fee paid
    /// precision: QUOTE_PRECISION
    pub total_fee_paid: u64,
    /// Total maker fee rebate
    /// precision: QUOTE_PRECISION
    pub total_fee_rebate: u64,
    /// Total discount from holding token
    /// precision: QUOTE_PRECISION
    pub total_token_discount: u64,
    /// Total discount from being referred
    /// precision: QUOTE_PRECISION
    pub total_referee_discount: u64,
    /// Total reward to referrer
    /// precision: QUOTE_PRECISION
    pub total_referrer_reward: u64,
    /// Total reward to referrer this epoch
    /// precision: QUOTE_PRECISION
    pub current_epoch_referrer_reward: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct SpotPosition {
    /// The scaled balance of the position. To get the token amount, multiply by the cumulative deposit/borrow
    /// interest of corresponding market.
    /// precision: SPOT_BALANCE_PRECISION
    pub scaled_balance: u64,
    /// How many spot bids the user has open
    /// precision: token mint precision
    pub open_bids: i64,
    /// How many spot asks the user has open
    /// precision: token mint precision
    pub open_asks: i64,
    /// The cumulative deposits/borrows a user has made into a market
    /// precision: token mint precision
    pub cumulative_deposits: i64,
    /// The market index of the corresponding spot market
    pub market_index: u16,
    /// Whether the position is deposit or borrow
    pub balance_type: SpotBalanceType,
    /// Number of open orders
    pub open_orders: u8,
    pub padding: [u8; 4],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct PerpPosition {
    /// The perp market's last cumulative funding rate. Used to calculate the funding payment owed to user
    /// precision: FUNDING_RATE_PRECISION
    pub last_cumulative_funding_rate: i64,
    /// the size of the users perp position
    /// precision: BASE_PRECISION
    pub base_asset_amount: i64,
    /// Used to calculate the users pnl. Upon entry, is equal to base_asset_amount * avg entry price - fees
    /// Updated when the user open/closes position or settles pnl. Includes fees/funding
    /// precision: QUOTE_PRECISION
    pub quote_asset_amount: i64,
    /// The amount of quote the user would need to exit their position at to break even
    /// Updated when the user open/closes position or settles pnl. Includes fees/funding
    /// precision: QUOTE_PRECISION
    pub quote_break_even_amount: i64,
    /// The amount quote the user entered the position with. Equal to base asset amount * avg entry price
    /// Updated when the user open/closes position. Excludes fees/funding
    /// precision: QUOTE_PRECISION
    pub quote_entry_amount: i64,
    /// The amount of open bids the user has in this perp market
    /// precision: BASE_PRECISION
    pub open_bids: i64,
    /// The amount of open asks the user has in this perp market
    /// precision: BASE_PRECISION
    pub open_asks: i64,
    /// The amount of pnl settled in this market since opening the position
    /// precision: QUOTE_PRECISION
    pub settled_pnl: i64,
    /// The number of lp (liquidity provider) shares the user has in this perp market
    /// LP shares allow users to provide liquidity via the AMM
    /// precision: BASE_PRECISION
    pub lp_shares: u64,
    /// The last base asset amount per lp the amm had
    /// Used to settle the users lp position
    /// precision: BASE_PRECISION
    pub last_base_asset_amount_per_lp: i64,
    /// The last quote asset amount per lp the amm had
    /// Used to settle the users lp position
    /// precision: QUOTE_PRECISION
    pub last_quote_asset_amount_per_lp: i64,
    /// Settling LP position can lead to a small amount of base asset being left over smaller than step size
    /// This records that remainder so it can be settled later on
    /// precision: BASE_PRECISION
    pub remainder_base_asset_amount: i32,
    /// The market index for the perp market
    pub market_index: u16,
    /// The number of open orders
    pub open_orders: u8,
    pub per_lp_base: i8,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct Order {
    /// The slot the order was placed
    pub slot: u64,
    /// The limit price for the order (can be 0 for market orders)
    /// For orders with an auction, this price isn't used until the auction is complete
    /// precision: PRICE_PRECISION
    pub price: u64,
    /// The size of the order
    /// precision for perps: BASE_PRECISION
    /// precision for spot: token mint precision
    pub base_asset_amount: u64,
    /// The amount of the order filled
    /// precision for perps: BASE_PRECISION
    /// precision for spot: token mint precision
    pub base_asset_amount_filled: u64,
    /// The amount of quote filled for the order
    /// precision: QUOTE_PRECISION
    pub quote_asset_amount_filled: u64,
    /// At what price the order will be triggered. Only relevant for trigger orders
    /// precision: PRICE_PRECISION
    pub trigger_price: u64,
    /// The start price for the auction. Only relevant for market/oracle orders
    /// precision: PRICE_PRECISION
    pub auction_start_price: i64,
    /// The end price for the auction. Only relevant for market/oracle orders
    /// precision: PRICE_PRECISION
    pub auction_end_price: i64,
    /// The time when the order will expire
    pub max_ts: i64,
    /// If set, the order limit price is the oracle price + this offset
    /// precision: PRICE_PRECISION
    pub oracle_price_offset: i32,
    /// The id for the order. Each users has their own order id space
    pub order_id: u32,
    /// The perp/spot market index
    pub market_index: u16,
    /// Whether the order is open or unused
    pub status: OrderStatus,
    /// The type of order
    pub order_type: OrderType,
    /// Whether market is spot or perp
    pub market_type: MarketType,
    /// User generated order id. Can make it easier to place/cancel orders
    pub user_order_id: u8,
    /// What the users position was when the order was placed
    pub existing_position_direction: PositionDirection,
    /// Whether the user is going long or short. LONG = bid, SHORT = ask
    pub direction: PositionDirection,
    /// Whether the order is allowed to only reduce position size
    pub reduce_only: bool,
    /// Whether the order must be a maker
    pub post_only: bool,
    /// Whether the order must be canceled the same slot it is placed
    pub immediate_or_cancel: bool,
    /// Whether the order is triggered above or below the trigger price. Only relevant for trigger orders
    pub trigger_condition: OrderTriggerCondition,
    /// How many slots the auction lasts
    pub auction_duration: u8,
    pub padding: [u8; 3],
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum SwapDirection {
    Add,
    Remove,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum ModifyOrderId {
    UserOrderId(u8),
    OrderId(u32),
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum PositionDirection {
    Long,
    Short,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum SpotFulfillmentType {
    SerumV3,
    Match,
    PhoenixV1,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum SwapReduceOnly {
    In,
    Out,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum TwapPeriod {
    FundingPeriod,
    FiveMin,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum LiquidationMultiplierType {
    Discount,
    Premium,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum MarginRequirementType {
    Initial,
    Fill,
    Maintenance,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum OracleValidity {
    NonPositive,
    TooVolatile,
    TooUncertain,
    StaleForMargin,
    InsufficientDataPoints,
    StaleForAMM,
    Valid,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum DriftAction {
    UpdateFunding,
    SettlePnl,
    TriggerOrder,
    FillOrderMatch,
    FillOrderAmm,
    Liquidate,
    MarginCalc,
    UpdateTwap,
    UpdateAMMCurve,
    OracleOrderPrice,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum PositionUpdateType {
    Open,
    Increase,
    Reduce,
    Close,
    Flip,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum DepositExplanation {
    None,
    Transfer,
    Borrow,
    RepayBorrow,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum DepositDirection {
    Deposit,
    Withdraw,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum OrderAction {
    Place,
    Cancel,
    Fill,
    Trigger,
    Expire,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum OrderActionExplanation {
    None,
    InsufficientFreeCollateral,
    OraclePriceBreachedLimitPrice,
    MarketOrderFilledToLimitPrice,
    OrderExpired,
    Liquidation,
    OrderFilledWithAMM,
    OrderFilledWithAMMJit,
    OrderFilledWithMatch,
    OrderFilledWithMatchJit,
    MarketExpired,
    RiskingIncreasingOrder,
    ReduceOnlyOrderIncreasedPosition,
    OrderFillWithSerum,
    NoBorrowLiquidity,
    OrderFillWithPhoenix,
    OrderFilledWithAMMJitLPSplit,
    OrderFilledWithLPJit,
    DeriskLp,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum LPAction {
    AddLiquidity,
    RemoveLiquidity,
    SettleLiquidity,
    RemoveLiquidityDerisk,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum LiquidationType {
    LiquidatePerp,
    LiquidateSpot,
    LiquidateBorrowForPerpPnl,
    LiquidatePerpPnlForDeposit,
    PerpBankruptcy,
    SpotBankruptcy,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum SettlePnlExplanation {
    None,
    ExpiredPosition,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum StakeAction {
    Stake,
    UnstakeRequest,
    UnstakeCancelRequest,
    Unstake,
    UnstakeTransfer,
    StakeTransfer,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum FillMode {
    Fill,
    PlaceAndMake,
    PlaceAndTake,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum PerpFulfillmentMethod {
    AMM(Option<u64>),
    Match(Pubkey, u16),
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum SpotFulfillmentMethod {
    ExternalMarket,
    Match(Pubkey, u16),
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum MarginCalculationMode {
    Standard {
        track_open_orders_fraction: bool,
    },
    Liquidation {
        market_to_track_margin_requirement: Option<MarketIdentifier>,
    },
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum OracleSource {
    Pyth,
    Switchboard,
    QuoteAsset,
    Pyth1K,
    Pyth1M,
    PythStableCoin,
    Prelaunch,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum PostOnlyParam {
    None,
    MustPostOnly,
    TryPostOnly,
    Slide,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum ModifyOrderPolicy {
    TryModify,
    MustModify,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum PerpOperation {
    UpdateFunding,
    AmmFill,
    Fill,
    SettlePnl,
    SettlePnlWithPosition,
    Liquidation,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum SpotOperation {
    UpdateCumulativeInterest,
    Fill,
    Withdraw,
    Liquidation,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum InsuranceFundOperation {
    Init,
    Add,
    RequestRemove,
    Remove,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum MarketStatus {
    Initialized,
    Active,
    FundingPaused,
    AmmPaused,
    FillPaused,
    WithdrawPaused,
    ReduceOnly,
    Settlement,
    Delisted,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum ContractType {
    Perpetual,
    Future,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum ContractTier {
    A,
    B,
    C,
    Speculative,
    HighlySpeculative,
    Isolated,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum AMMLiquiditySplit {
    ProtocolOwned,
    LPOwned,
    Shared,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum SettlePnlMode {
    MustSettle,
    TrySettle,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum SpotBalanceType {
    Deposit,
    Borrow,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum SpotFulfillmentConfigStatus {
    Enabled,
    Disabled,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum AssetTier {
    Collateral,
    Protected,
    Cross,
    Isolated,
    Unlisted,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum ExchangeStatus {
    DepositPaused,
    WithdrawPaused,
    AmmPaused,
    FillPaused,
    LiqPaused,
    FundingPaused,
    SettlePnlPaused,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum UserStatus {
    BeingLiquidated,
    Bankrupt,
    ReduceOnly,
    AdvancedLp,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum AssetType {
    Base,
    Quote,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum OrderStatus {
    Init,
    Open,
    Filled,
    Canceled,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum OrderType {
    Market,
    Limit,
    TriggerMarket,
    TriggerLimit,
    Oracle,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum OrderTriggerCondition {
    Above,
    Below,
    TriggeredAbove,
    TriggeredBelow,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum MarketType {
    Spot,
    Perp,
}

/// Events
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum DriftV2Event {
    NewUserRecord {
        ts: i64,
        user_authority: Pubkey,
        user: Pubkey,
        sub_account_id: u16,
        name: [u8; 32],
        referrer: Pubkey,
    },
    DepositRecord {
        ts: i64,
        user_authority: Pubkey,
        user: Pubkey,
        direction: DepositDirection,
        deposit_record_id: u64,
        amount: u64,
        market_index: u16,
        oracle_price: i64,
        market_deposit_balance: u128,
        market_withdraw_balance: u128,
        market_cumulative_deposit_interest: u128,
        market_cumulative_borrow_interest: u128,
        total_deposits_after: u64,
        total_withdraws_after: u64,
        explanation: DepositExplanation,
        transfer_user: Option<Pubkey>,
    },
    SpotInterestRecord {
        ts: i64,
        market_index: u16,
        deposit_balance: u128,
        cumulative_deposit_interest: u128,
        borrow_balance: u128,
        cumulative_borrow_interest: u128,
        optimal_utilization: u32,
        optimal_borrow_rate: u32,
        max_borrow_rate: u32,
    },
    FundingPaymentRecord {
        ts: i64,
        user_authority: Pubkey,
        user: Pubkey,
        market_index: u16,
        funding_payment: i64,
        base_asset_amount: i64,
        user_last_cumulative_funding: i64,
        amm_cumulative_funding_long: i128,
        amm_cumulative_funding_short: i128,
    },
    FundingRateRecord {
        ts: i64,
        record_id: u64,
        market_index: u16,
        funding_rate: i64,
        funding_rate_long: i128,
        funding_rate_short: i128,
        cumulative_funding_rate_long: i128,
        cumulative_funding_rate_short: i128,
        oracle_price_twap: i64,
        mark_price_twap: u64,
        period_revenue: i64,
        base_asset_amount_with_amm: i128,
        base_asset_amount_with_unsettled_lp: i128,
    },
    CurveRecord {
        ts: i64,
        record_id: u64,
        peg_multiplier_before: u128,
        base_asset_reserve_before: u128,
        quote_asset_reserve_before: u128,
        sqrt_k_before: u128,
        peg_multiplier_after: u128,
        base_asset_reserve_after: u128,
        quote_asset_reserve_after: u128,
        sqrt_k_after: u128,
        base_asset_amount_long: u128,
        base_asset_amount_short: u128,
        base_asset_amount_with_amm: i128,
        total_fee: i128,
        total_fee_minus_distributions: i128,
        adjustment_cost: i128,
        oracle_price: i64,
        fill_record: u128,
        number_of_users: u32,
        market_index: u16,
    },
    OrderRecord {
        ts: i64,
        user: Pubkey,
        order: Order,
    },
    OrderActionRecord {
        ts: i64,
        action: OrderAction,
        action_explanation: OrderActionExplanation,
        market_index: u16,
        market_type: MarketType,
        filler: Option<Pubkey>,
        filler_reward: Option<u64>,
        fill_record_id: Option<u64>,
        base_asset_amount_filled: Option<u64>,
        quote_asset_amount_filled: Option<u64>,
        taker_fee: Option<u64>,
        maker_fee: Option<u64>,
        referrer_reward: Option<u32>,
        quote_asset_amount_surplus: Option<i64>,
        spot_fulfillment_method_fee: Option<u64>,
        taker: Option<Pubkey>,
        taker_order_id: Option<u32>,
        taker_order_direction: Option<PositionDirection>,
        taker_order_base_asset_amount: Option<u64>,
        taker_order_cumulative_base_asset_amount_filled: Option<u64>,
        taker_order_cumulative_quote_asset_amount_filled: Option<u64>,
        maker: Option<Pubkey>,
        maker_order_id: Option<u32>,
        maker_order_direction: Option<PositionDirection>,
        maker_order_base_asset_amount: Option<u64>,
        maker_order_cumulative_base_asset_amount_filled: Option<u64>,
        maker_order_cumulative_quote_asset_amount_filled: Option<u64>,
        oracle_price: i64,
    },
    LPRecord {
        ts: i64,
        user: Pubkey,
        action: LPAction,
        n_shares: u64,
        market_index: u16,
        delta_base_asset_amount: i64,
        delta_quote_asset_amount: i64,
        pnl: i64,
    },
    LiquidationRecord {
        ts: i64,
        liquidation_type: LiquidationType,
        user: Pubkey,
        liquidator: Pubkey,
        margin_requirement: u128,
        total_collateral: i128,
        margin_freed: u64,
        liquidation_id: u64,
        bankrupt: bool,
        canceled_order_ids: Vec<u32>,
        liquidate_perp: LiquidatePerpRecord,
        liquidate_spot: LiquidateSpotRecord,
        liquidate_borrow_for_perp_pnl: LiquidateBorrowForPerpPnlRecord,
        liquidate_perp_pnl_for_deposit: LiquidatePerpPnlForDepositRecord,
        perp_bankruptcy: PerpBankruptcyRecord,
        spot_bankruptcy: SpotBankruptcyRecord,
    },
    SettlePnlRecord {
        ts: i64,
        user: Pubkey,
        market_index: u16,
        pnl: i128,
        base_asset_amount: i64,
        quote_asset_amount_after: i64,
        quote_entry_amount: i64,
        settle_price: i64,
        explanation: SettlePnlExplanation,
    },
    InsuranceFundRecord {
        ts: i64,
        spot_market_index: u16,
        perp_market_index: u16,
        user_if_factor: u32,
        total_if_factor: u32,
        vault_amount_before: u64,
        insurance_vault_amount_before: u64,
        total_if_shares_before: u128,
        total_if_shares_after: u128,
        amount: i64,
    },
    InsuranceFundStakeRecord {
        ts: i64,
        user_authority: Pubkey,
        action: StakeAction,
        amount: u64,
        market_index: u16,
        insurance_vault_amount_before: u64,
        if_shares_before: u128,
        user_if_shares_before: u128,
        total_if_shares_before: u128,
        if_shares_after: u128,
        user_if_shares_after: u128,
        total_if_shares_after: u128,
    },
    SwapRecord {
        ts: i64,
        user: Pubkey,
        amount_out: u64,
        amount_in: u64,
        out_market_index: u16,
        in_market_index: u16,
        out_oracle_price: i64,
        in_oracle_price: i64,
        fee: u64,
    },
}
